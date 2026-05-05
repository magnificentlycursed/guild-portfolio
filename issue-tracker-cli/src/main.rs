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
    /// Create a new issue (with optional priority and labels)
    Create {
        /// Issue title
        title: String,
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
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) if e.use_stderr() => {
            let msg = e.to_string().replacen("error:", "Error:", 1);
            eprint!("{}", msg);
            std::process::exit(1);
        }
        Err(e) => {
            print!("{}", e);
            std::process::exit(0);
        }
    };
    let path = Path::new("tracker.json");

    let result = match cli.command {
        Commands::Create {
            title,
            priority,
            label,
        } => tracker::cmd_create(&title, priority.as_deref(), &label, path),
        Commands::List {
            status,
            priority,
            label,
        } => tracker::cmd_list(
            status.as_deref(),
            priority.as_deref(),
            label.as_deref(),
            path,
        ),
        Commands::Status { id, status } => tracker::cmd_status(&id, &status, path),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
