use clap::{Parser, Subcommand};
use crate::persistence::SeriesList;

mod persistence;
mod commands;
#[cfg(test)]
mod tests;
mod schema;
mod errors;

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

fn get_toml_path() -> String {
    let home = std::env::var("HOME").expect("Could not get home directory");
    format!("{}/.upnext.toml", home)
}

fn get_series_list() -> Option<persistence::SeriesList> {
    let toml_path = get_toml_path();
    persistence::read_toml_file(&toml_path).ok()
}

fn create_series_list() -> persistence::SeriesList {
    persistence::SeriesList::new()
}

fn save_series_list(series_list: &persistence::SeriesList) {
    let toml_path = get_toml_path();
    persistence::write_toml_file(&toml_path, series_list).expect("Could not write toml file");
}

fn main() {
    let cli = Cli::parse();


    match &cli.command {
        Commands::Init => {
            println!("Initializing series");
            let mut series_list: SeriesList = get_series_list().unwrap_or_else(create_series_list);
            series_list.init_series();
            save_series_list(&series_list);
        }
        Commands::Play => println!("Playing series"),
        Commands::Next => println!("Playing next episode"),
        Commands::Info => println!("Printing series information"),
        Commands::IncrementEpisode { n } => {
            println!("Incrementing episode by {}", n);
            let mut series_list = get_series_list().expect("Could not read series list");
            let series = series_list.series.iter_mut().find(|s| s.path == std::env::current_dir().unwrap().to_str().unwrap());
            match series {
                Some(series) => {
                    series.next_episode += *n;
                    save_series_list(&series_list);
                }
                None => println!("No series found in current directory"),
            }
        }
        Commands::SetNextEpisode { n } => println!("Setting next episode to {}", n),
        Commands::Remove => println!("Removing series data"),
        Commands::List => println!("Listing all series"),
        Commands::Edit => println!("Opening the toml file in the default editor"),
    }
}
