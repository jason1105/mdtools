// hugox subcommand flags... arguments... paths...
use clap::{Parser, Subcommand};
use hugox::RunCommand;

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
    AddTag(hugox::add_tag::AddTag),
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
