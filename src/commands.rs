use crate::commands::utils::{find_files, get_cwd, load_series_list, save_series_list};
use crate::errors::{Result, UpNextError};
use crate::schema::{Series, SeriesList};

pub static APP_NAME: &str = "upnext";

pub(super) fn print_series_info() -> Result<()> {
    let series_list = load_series_list()?;
    let current_dir = get_cwd()?;
    let series = series_list.find_series(&current_dir)?;
    Ok(println!("{series}"))
}

pub(super) fn print_all_series_info() -> Result<()> {
    let content = std::fs::read_to_string(crate::utils::get_toml_path()?)?;
    Ok(println!("{content}"))
}

pub(super) fn init() -> Result<()> {
    let mut series_list: SeriesList = load_series_list().unwrap_or_else(|_| SeriesList::new());
    let current_dir = get_cwd()?;

    series_list.add_series(current_dir.clone())?;
    let series = series_list.series.last().ok_or_else(|| UpNextError::GenericError("Could not get last series".to_string()))?;
    save_series_list(&series_list)?;
    Ok(println!("{series}"))
}

pub(super) fn increment(n: i64) -> Result<()> {
    let mut series_list = load_series_list()?;
    let current_dir = get_cwd()?;
    let series = series_list.find_series_mut(&current_dir)?;
    println!("{series}");
    series.next_episode += n;
    save_series_list(&series_list)?;

    let series = series_list.find_series(&current_dir)?;
    Ok(println!("{series}"))
}

pub(super) fn set_next_episode(n: u32) -> Result<()> {
    let mut series_list = load_series_list()?;
    let current_dir = get_cwd()?;
    let series = series_list.find_series_mut(&current_dir)?;
    println!("{series}");

    series.next_episode = i64::from(n);
    save_series_list(&series_list)?;

    let series = series_list.find_series(&current_dir)?;
    Ok(println!("{series}"))
}

pub(super) fn remove() -> Result<()> {
    let mut series_list = load_series_list()?;
    let current_dir = get_cwd()?;
    let series = series_list.find_series(&current_dir)?;
    println!("{series}");

    series_list.remove_series(&get_cwd()?);
    save_series_list(&series_list)?;

    Ok(println!("Series removed."))
}

pub(super) fn edit_in_default_editor() -> Result<()> {
    let path = crate::utils::get_toml_path()?;
    println!("Opening the toml file in the default editor.");
    let _output = std::process::Command::new("xdg-open")
        .arg(path)
        .output()?;
    Ok(())
}

// TODO when testing also test ignore casing
#[allow(clippy::needless_for_each)]
pub(super) fn find_series(search_term: &str) -> Result<()> {
    let series_list = load_series_list()?;
    let lower_search_term = search_term.to_lowercase();
    let found_series: Vec<&Series> = series_list.series.iter().filter(|s| s.path.to_lowercase().contains(&lower_search_term)).collect();
    if found_series.is_empty() {
        println!("No series found with the search term: {search_term}");
    } else {
        found_series.iter().for_each(|s| println!("{s}"));
    }
    Ok(())
}

pub(super) fn print_toml_path() -> Result<()> {
    Ok(println!("{}", crate::utils::get_toml_path()?))
}

pub(super) fn play_next_episode() -> Result<()> {
    let mut series_list = load_series_list()?;
    let current_dir = get_cwd()?;
    let series = series_list.find_series_mut(&current_dir)?;
    println!("{series}");

    let files = find_files(&series.path)?;
    if series.next_episode > i64::try_from(files.len())? {
        Err(UpNextError::SeriesOver)
    } else {
        let file_path = &files[usize::try_from(series.next_episode)? - 1];
        println!("Starting episode \"{}\" at {}.\n", file_path.file_name().unwrap().to_string_lossy(), chrono::Local::now().format("%H:%M"));
        player::play_in_vlc(file_path)?;

        series.next_episode += 1;
        save_series_list(&series_list)?;

        let series = series_list.find_series(&current_dir)?;
        Ok(println!("{series}"))
    }
}

pub(super) fn play() -> Result<()> {
    let mut series_list = load_series_list()?;
    let current_dir = get_cwd()?;
    let i = series_list.find_series_index(&current_dir)?;
    let series = series_list.at_mut(i)?;
    let files = find_files(&series.path)?;

    println!("{series}");
    if series_list.at(i)?.next_episode <= i64::try_from(files.len())? {
        let series = series_list.at_mut(i)?;
        let file_path = &files[usize::try_from(series.next_episode)? - 1];
        player::play_in_vlc(file_path)?;
        series.next_episode += 1;
        save_series_list(&series_list)?;
        let series = series_list.find_series(&current_dir)?;
        println!("{series}");
    }
    while series_list.at(i)?.next_episode <= i64::try_from(files.len())? {
        let series = series_list.at_mut(i)?;
        player::countdown(8);
        let file_path = &files[usize::try_from(series.next_episode)? - 1];
        println!("Starting episode \"{}\" at {}.\n", file_path.file_name().unwrap().to_string_lossy(), chrono::Local::now().format("%H:%M"));
        player::play_in_vlc(file_path)?;
        series.next_episode += 1;
        save_series_list(&series_list)?;
        let series = series_list.find_series(&current_dir)?;
        println!("{series}");
    }

    Err(UpNextError::SeriesOver)
}

mod player {
    use std::path::Path;

    use crate::errors::{Result, UpNextError};

    const VLC_COMMAND: &str = {
        #[cfg(target_os = "linux")]
        {
            "vlc"
        }
        #[cfg(target_os = "macos")]
        {
            "/Applications/VLC.app/Contents/MacOS/VLC"
        }
    };

    pub(super) fn play_in_vlc(file_path: &Path) -> Result<()> {
        let output = std::process::Command::new(VLC_COMMAND)
            .arg(file_path)
            .arg("--play-and-exit")
            .arg("--fullscreen")
            .output();
        match output {
            Ok(output) => {
                if output.status.success() {
                    Ok(())
                } else {
                    Err(UpNextError::VlcError(format!(
                        "exited with status: {0}",
                        output.status
                    )))
                }
            }
            Err(e) => Err(UpNextError::VlcError(e.to_string())),
        }
    }

    pub(super) fn countdown(seconds: u64) {
        println!("Playing next episode in {seconds} seconds...");
        for i in (0..seconds).rev() {
            std::thread::sleep(std::time::Duration::from_secs(1));
            println!("{i}");
        }
    }
}

mod utils {
    use std::path::PathBuf;

    use crate::data_management::persistence;
    use crate::errors::{Result, UpNextError};
    use crate::schema::SeriesList;
    use crate::utils;

    pub(super) fn save_series_list(series_list: &SeriesList) -> Result<()> {
        persistence::write_toml_file(utils::get_toml_path()?, series_list)
    }

    pub(super) fn load_series_list() -> Result<SeriesList> {
        persistence::read_toml_file(utils::get_toml_path()?)
    }

    pub(super) fn get_cwd() -> Result<String> {
        std::env::current_dir()?.to_str().ok_or_else(|| UpNextError::GenericError("Could not convert cwd to string".to_string())).map(ToString::to_string)
    }

    pub(super) fn find_files(path: &str) -> Result<Vec<PathBuf>> {
        let extensions = ["mkv", "mp4", "avi", "flv", "mov", "wmv", "webm", "mpg", "mpeg", "m4v"];
        let mut files = vec![];
        // read all files in the directory
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let ext = path.extension()
                    .ok_or(UpNextError::GenericError(format!("Path contained no extension: {}", path.to_str().unwrap())))?
                    .to_str()
                    .expect("Could not convert extension to string.");
                {
                    if extensions.contains(&ext) {
                        files.push(path);
                    }
                }
            }
        }
        files.sort();
        Ok(files)
    }
}
