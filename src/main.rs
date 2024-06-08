use clap::{Parser, Subcommand};

use crate::commands::{increment, init};
use crate::errors::UpNextError;

mod persistence;
mod commands;
#[cfg(test)]
mod tests;
mod schema;
mod errors;
mod utils;

/// A simple CLI app to keep track of your progress in watching TV shows, series.
#[derive(Parser)]
#[command(
    long_about = "This simple app helps you keep track of the progress in the TV shows you are watching. \n\
                  Data is saved in the file `~/.upnext.toml`. For each show, it saves the path and the \n\
                  episode number. If you change the path or delete, add, rename (reorder) episodes the \n\
                  tracking will be broken. You can fix it by editing the file. Use the app to play next \n\
                  episodes automatically in VLC."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the current directory as a series.
    #[command(name = "init")]
    Init,
    /// Start playing series (autoplay next episode).
    #[command(name = "play")]
    Play,
    /// Play next episode.
    #[command(name = "next")]
    Next,
    /// Print the series information in the current directory.
    #[command(name = "info")]
    Info,
    /// Increment the episode number by the given amount.
    #[command(name = "inc")]
    IncrementEpisode {
        /// Increment by this amount. If the new episode number is equal to the total number of episodes, the series is considered complete.
        n: i64,
    },
    /// Set the next episode number explicitly.
    #[command(name = "set")]
    SetNextEpisode {
        /// The episode number to set. Starts at 1. If equal to the total number of episodes, the series is considered complete.
        n: usize,
    },
    /// Remove data about the series in current directory.
    #[command(name = "remove")]
    Remove,
    /// Print all series information.
    #[command(name = "list")]
    List,
    /// Open the toml file in the default editor.
    #[command(name = "edit")]
    Edit,
}


fn main() {
    let cli = Cli::parse();

    let res = match &cli.command {
        Commands::Init => {
            init()
        }
        // Commands::Play => println!("Playing series"),
        // Commands::Next => println!("Playing next episode"),
        // Commands::Info => println!("Printing series information"),
        Commands::IncrementEpisode { n } => {
            increment(*n)
        }
        // Commands::SetNextEpisode { n } => println!("Setting next episode to {}", n),
        // Commands::Remove => println!("Removing series data"),
        // Commands::List => println!("Listing all series"),
        // Commands::Edit => println!("Opening the toml file in the default editor"),
        _ => Err(UpNextError::Unimplemented),
    };

    if let Err(e) = res {
        eprintln!("{}", e);
    }
}
