#![warn(clippy::all, clippy::pedantic)]

// mdtools subcommand flags... arguments... paths...
mod prelude {
    pub use clap::{Parser, Subcommand};
    pub use log::{info, warn};
    pub use mdtools::commands::*;
}

use prelude::*;

/// Simple program to add tags to files.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add tags to file.
    AddTag(add_tag::AddTag),
    /// Make footer links.
    MkFootlinks(mk_footlinks::MkFootlinks),
}

// env RUST_LOG=info cargo run mk-footlinks --path /c/temp/Readme.md
fn main() {
    env_logger::init();
    let args = Cli::parse();

    info!("Run command: {:?}", args.command);
    match args.command {
        Commands::AddTag(add_tag_command) => {
            add_tag_command.run();
        }
        Commands::MkFootlinks(command) => {
            command.run();
        }
    }
    println!("\x1b[32m{}\x1b[0m", "Complete.");
}
