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
    /// Create a new issue
    Create {
        /// Issue title
        title: String,
    },
    /// List issues (default: open only)
    List {
        /// Filter by status: open, in-progress, done
        #[arg(long)]
        status: Option<String>,
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
    let cli = Cli::parse();
    let path = Path::new("tracker.json");

    let result = match cli.command {
        Commands::Create { title } => tracker::cmd_create(&title, path),
        Commands::List { status } => tracker::cmd_list(status.as_deref(), path),
        Commands::Status { id, status } => tracker::cmd_status(&id, &status, path),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
