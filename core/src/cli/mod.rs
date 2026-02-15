pub mod commands;
pub mod keys;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rhodi")]
#[command(about = "A ledger of truth for research documents", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new .tmd document
    Init {
        /// Path for the new document (default: current directory)
        path: Option<PathBuf>,
        /// Document title
        #[arg(long)]
        title: Option<String>,
        /// Author name
        #[arg(long)]
        author: Option<String>,
    },
    /// Compute hashes, sign, and publish a document
    Seal {
        /// Path to the .tmd document
        path: PathBuf,
        /// Key name to use (default: default)
        #[arg(long)]
        key: Option<String>,
    },
    /// Verify document integrity and traces
    Verify {
        /// Path to the .tmd document
        path: PathBuf,
        /// Exit with error if any trace fails (default: warn only)
        #[arg(long, short)]
        strict: bool,
    },
    /// Refresh hash in all trace blocks
    Update {
        /// Path to the .tmd document
        path: PathBuf,
    },
    /// Show document status and metadata
    Status {
        /// Path to the .tmd document
        path: PathBuf,
    },
    /// Generate a new Ed25519 keypair
    Keygen {
        /// Name for the key (default: default)
        #[arg(long)]
        name: Option<String>,
        /// Show the public key after generation
        #[arg(long, short)]
        show: bool,
    },
}

pub fn run() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init {
            path,
            title,
            author,
        } => {
            if let Err(e) = crate::cli::commands::init::run(path, title, author) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Seal { path, key } => {
            if let Err(e) = crate::cli::commands::seal::run(path, key) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Verify { path, strict } => {
            match crate::cli::commands::verify::run(path, strict) {
                Ok(report) => {
                    if !report.warnings.is_empty() {
                        println!("Warnings:");
                        for warning in &report.warnings {
                            println!("  - {}", warning);
                        }
                    }
                    if !report.errors.is_empty() {
                        eprintln!("Errors found:");
                        for err in &report.errors {
                            eprintln!("  - {}", err);
                        }
                        std::process::exit(1);
                    }
                    if report.warnings.is_empty() && report.errors.is_empty() {
                        println!("✓ Document verified successfully");
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Update { path } => {
            if let Err(e) = crate::cli::commands::update::run(path) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
            println!("Trace hashes updated successfully");
        }
        Commands::Status { path } => {
            if let Err(e) = crate::cli::commands::status::run(path) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Keygen { name, show } => {
            if let Err(e) = crate::cli::commands::keygen::run(name, show) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
