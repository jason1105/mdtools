#![warn(clippy::all, clippy::pedantic)]

// mdtools subcommand flags... arguments... paths...
mod prelude {
    pub use clap::{Parser, Subcommand};
    pub use mdtools::commands::*;
}

use std::cell::{Cell, RefCell};

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
    MkFootlinks(mk_footlinks::MkFootlinks),
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::AddTag(add_tag_command) => {
            print!("{:?}", add_tag_command);
            add_tag_command.run();
        }
        Commands::MkFootlinks(command) => {
            print!("Run command: {:?}", command);
            command.run();
        }
    }
}
