use clap::{CommandFactory, Parser, Subcommand};

use crate::commands::{
    edit_in_default_editor, find_series, increment, init, play, play_next_episode,
    print_all_series_info, print_current_series_info, print_toml_path, remove, set_next_episode,
};

mod commands;
mod errors;
mod persistence;
mod schema;
#[cfg(test)]
mod tests;
mod utils;

pub static APP_NAME: &str = "upnext";
pub static TOML_PATH_ENV_VAR_NAME: &str = "UPNEXT_TOML_PATH"; // keep docs below in sync if changed

/// A simple CLI app to keep track of your progress in watching TV shows, series.
#[derive(Parser)]
#[command(
    long_about = "This simple app helps you keep track of the progress in the TV shows you are watching. \n\
                  Data is saved in the file `~/.upnext.toml`. For each show, it saves the path and the \n\
                  episode number. If you change the path or delete, add, rename (reorder) episodes the \n\
                  tracking will be broken. You can fix it by editing the file. Use the app to play next \n\
                  episodes automatically in VLC.\n\
                  You can override the location where the data is saved by setting the environment variable \n\
                  `UPNEXT_TOML_PATH` to the desired path."
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
    Play {
        /// Delay between episodes in seconds. If not specified, defaults to 5 seconds.
        #[arg(long, short = 'd', default_value_t = 5)]
        delay_seconds: u64,
    },
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
        #[arg(default_value_t = 1)]
        n: i64,
    },
    /// Set the next episode number explicitly.
    #[command(name = "set")]
    SetNextEpisode {
        /// The episode number to set. Starts at 1. If equal to the total number of episodes, the series is considered complete.
        n: u32,
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
    /// Find and print all series where the path contains the search string.
    #[command(name = "find")]
    Find {
        /// The search term.
        search_term: String,
    },
    /// Print the path to the toml file. (For debugging purposes.)
    #[command(name = "which")]
    Which,
    /// Generate shell completions.
    #[command(name = "completions")]
    Completions {
        /// The shell to generate completions for.
        shell: clap_complete::Shell,
    },
}

fn main() {
    let cli = Cli::parse();

    let res = match &cli.command {
        Commands::Init => init(),
        Commands::Play { delay_seconds } => play(*delay_seconds),
        Commands::Next => play_next_episode(),
        Commands::Info => print_current_series_info(),
        Commands::IncrementEpisode { n } => increment(*n),
        Commands::SetNextEpisode { n } => set_next_episode(*n),
        Commands::Remove => remove(),
        Commands::List => print_all_series_info(),
        Commands::Edit => edit_in_default_editor(),
        Commands::Find { search_term } => find_series(search_term),
        Commands::Which => print_toml_path(),
        Commands::Completions { shell } => Ok(clap_complete::generate(
            *shell,
            &mut Cli::command(),
            APP_NAME,
            &mut std::io::stdout(),
        )),
    };

    if let Err(e) = res {
        eprintln!("{e}");
    }
}
