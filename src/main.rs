// mdtools subcommand flags... arguments... paths...

mod commands;

mod prelude {
    pub use crate::commands::*;
    pub use clap::Args;
    pub use clap::{Parser, Subcommand};
    pub use mdtools::file_utils;
    pub use regex::Regex;
    pub use std::{
        collections::BTreeSet,
        ffi::OsString,
        io::{BufReader, Error, ErrorKind, Result},
        path::PathBuf,
    };
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
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::AddTag(add_tag_command) => {
            print!("{:?}", add_tag_command);
            add_tag_command.run();
        }
    }
}
