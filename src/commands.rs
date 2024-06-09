use crate::commands::utils::{get_cwd, load_series_list, save_series_list};
use crate::errors::{Result, UpNextError};
use crate::schema::SeriesList;

pub static APP_NAME: &str = "upnext";

pub(super) fn print_series_info() -> Result<()> {
    let series_list = load_series_list()?;
    let current_dir = get_cwd()?;
    let series = series_list.find_series(&current_dir)?;
    Ok(println!("{}", series))
}

pub(super) fn print_all_series_info() -> Result<()> {
    let content = std::fs::read_to_string(crate::utils::get_toml_path()?)?;
    Ok(println!("{}", content))
}

pub(super) fn init() -> Result<()> {
    let mut series_list: SeriesList = load_series_list().unwrap_or_else(|_| SeriesList::new());
    let current_dir = get_cwd()?;

    series_list.add_series(current_dir.clone())?;
    let series = series_list.series.last().ok_or_else(|| UpNextError::GenericError("Could not get last series".to_string()))?;
    save_series_list(&series_list)?;
    Ok(println!("{}", series))
}

pub(super) fn increment(n: i64) -> Result<()> {
    let mut series_list = load_series_list()?;
    let current_dir = get_cwd()?;
    let series = series_list.find_series_mut(&current_dir)?;
    println!("{}", series);

    series.next_episode += n;
    save_series_list(&series_list)?;

    let series = series_list.find_series(&current_dir)?;
    Ok(println!("{}", series))
}

pub(super) fn set_next_episode(n: i64) -> Result<()> {
    let mut series_list = load_series_list()?;
    let current_dir = get_cwd()?;
    let series = series_list.find_series_mut(&current_dir)?;
    println!("{}", series);

    series.next_episode = n;
    save_series_list(&series_list)?;

    let series = series_list.find_series(&current_dir)?;
    Ok(println!("{}", series))
}

pub(super) fn remove() -> Result<()> {
    let mut series_list = load_series_list()?;
    let current_dir = get_cwd()?;
    let series = series_list.find_series(&current_dir)?;
    println!("{}", series);

    series_list.remove_series(&get_cwd()?);
    save_series_list(&series_list)?;

    Ok(println!("Series removed"))
}

pub(super) fn play_next_episode() -> Result<()> {
    let mut series_list = load_series_list()?;
    let current_dir = get_cwd()?;
    let series = series_list.find_series_mut(&current_dir)?;
    println!("{}", series);

    let files = utils::find_files(&series.path)?;
    if series.next_episode > files.len() as i64 {
        Err(UpNextError::SeriesOver)
    } else {
        println!("Starting episode {} at {}\n", series.next_episode, chrono::Local::now().format("%H:%M"));
        let file_path = &files[series.next_episode as usize - 1];
        let _output = std::process::Command::new("vlc")
            .arg(file_path)
            .arg("--play-and-exit")
            .arg("--fullscreen")
            .output()?;

        series.next_episode += 1;
        save_series_list(&series_list)?;

        let series = series_list.find_series(&current_dir)?;
        Ok(println!("{}", series))
    }
}

mod utils {
    use crate::{persistence, utils};
    use crate::errors::{Result, UpNextError};
    use crate::schema::SeriesList;

    pub(super) fn save_series_list(series_list: &SeriesList) -> Result<()> {
        persistence::write_toml_file(utils::get_toml_path()?, series_list)
    }

    pub(super) fn load_series_list() -> Result<SeriesList> {
        persistence::read_toml_file(utils::get_toml_path()?)
    }

    pub(super) fn get_cwd() -> Result<String> {
        std::env::current_dir()?.to_str().ok_or_else(|| UpNextError::GenericError("Could not convert cwd to string".to_string())).map(|s| s.to_string())
    }

    pub(super) fn find_files(path: &str) -> Result<Vec<String>> {
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
                    .expect("Could not convert extension to string");
                {
                    if extensions.contains(&ext) {
                        files.push(path.to_str().unwrap().to_string());
                    }
                }
            }
        }
        files.sort();
        Ok(files)
    }
}
