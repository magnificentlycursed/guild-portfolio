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
    /// List open issues
    List,
}

fn main() {
    let cli = Cli::parse();
    let path = Path::new("tracker.json");

    let result = match cli.command {
        Commands::Create { title } => tracker::cmd_create(&title, path),
        Commands::List => tracker::cmd_list(path),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
