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
}
