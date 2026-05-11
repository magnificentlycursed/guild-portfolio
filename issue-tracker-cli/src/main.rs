use clap::{Parser, Subcommand};
use std::path::Path;

#[derive(Parser)]
#[command(name = "tracker", about = "Personal issue tracker")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new issue (with optional description, priority, and labels)
    Create {
        /// Issue title
        title: String,
        /// Free-form description (stored verbatim; not trimmed)
        #[arg(long)]
        description: Option<String>,
        /// Priority: low, medium, high (default: medium)
        #[arg(long)]
        priority: Option<String>,
        /// Label (repeatable; deduplicated; case-preserved)
        #[arg(long)]
        label: Vec<String>,
    },
    /// List issues (default: open) with optional status / priority / label filters
    List {
        /// Filter by status: open, in-progress, done
        #[arg(long)]
        status: Option<String>,
        /// Filter by priority: low, medium, high
        #[arg(long)]
        priority: Option<String>,
        /// Filter by label (case-sensitive exact match; single value only)
        #[arg(long)]
        label: Option<String>,
    },
    /// Change an issue's status
    Status {
        /// Issue ID
        id: String,
        /// New status: open, in-progress, done
        status: String,
    },
    /// Show full details for an issue: ID, Title, Status, Priority, Labels, Description, Created, Updated
    Show {
        /// Issue ID (positive integer, >= 1)
        id: String,
    },
    /// Delete an issue. No confirmation prompt (see DESIGN.md D1); deleted IDs are never reused.
    Delete {
        /// Issue ID (positive integer, >= 1)
        id: String,
    },
}

fn main() {
    // Restore default SIGPIPE behavior: Rust's runtime ignores SIGPIPE, which makes
    // every `println!` to a closed pipe panic with an EPIPE error. Setting SIG_DFL
    // makes `tracker list | head` exit silently when `head` closes its end of the pipe,
    // matching the DESIGN.md exit-{0,1} contract and the Unix convention for filters.
    // SAFETY: called once at process startup before any threads are spawned.
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    // DESIGN.md stderr contract: error messages begin with `Error:`; all errors exit 1.
    // clap defaults to lowercase `error:` and exit code 2 for usage errors — transform here.
    // For --help / --version, clap routes via stdout and exits 0; preserve that behavior.
    //
    // Round 2 (RT R10 F1 / stderr Cc-escape rule extended to clap pipeline):
    // user-supplied bytes reflected by clap (e.g. `unrecognized subcommand
    // 'X'` where X may contain `\r`, NEL, C1 bytes from a paste) must be
    // Cc-escaped before reaching stderr — per DESIGN.md "stderr contract".
    // We apply `sanitize_quoted_values` rather than `display_safe` to
    // preserve clap's structural newlines (the `\n\nUsage: ...` block) while
    // still escaping any control bytes that appear INSIDE the `'X'` quoted
    // user-reflection regions.
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) if e.use_stderr() => {
            let raw = e.to_string().replacen("error:", "Error:", 1);
            let safe = tracker::sanitize_quoted_values(&raw);
            eprint!("{}", safe);
            std::process::exit(1);
        }
        Err(e) => {
            print!("{}", e);
            std::process::exit(0);
        }
    };
    let path = Path::new("tracker.json");

    // Round 2 (SE R17 F1 / SA R15 F2): decide ColorMode ONCE in main and
    // thread through to cmd_list / cmd_show. Previously each of those
    // functions called `is_terminal()` independently, duplicating an
    // environmental check that should have a single decision point.
    let color = tracker::color_mode_from_env();

    let result = match cli.command {
        Commands::Create {
            title,
            description,
            priority,
            label,
        } => tracker::cmd_create(
            &tracker::CreateArgs {
                title_raw: &title,
                description_raw: description.as_deref(),
                priority_raw: priority.as_deref(),
                labels_raw: &label,
            },
            path,
        ),
        Commands::List {
            status,
            priority,
            label,
        } => tracker::cmd_list(
            status.as_deref(),
            priority.as_deref(),
            label.as_deref(),
            path,
            color,
        ),
        Commands::Status { id, status } => tracker::cmd_status(&id, &status, path),
        Commands::Show { id } => tracker::cmd_show(&id, path, color),
        Commands::Delete { id } => tracker::cmd_delete(&id, path),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
