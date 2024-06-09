use crate::commands::utils::{get_cwd, load_series_list, save_series_list};
use crate::errors::{Result, UpNextError};
use crate::schema::SeriesList;

pub static APP_NAME: &str = "upnext";

pub(super) fn init() -> Result<()> {
    let mut series_list: SeriesList = load_series_list().unwrap_or_else(|_| SeriesList::new());
    let current_dir = get_cwd()?;
    series_list.add_series(current_dir.clone())?;
    let series = series_list.series.last().ok_or_else(|| UpNextError::GenericError("Could not get last series".to_string()))?;
    save_series_list(&series_list)?;
    Ok(println!("{}", series))
}

pub(super) fn increment(n: i64) -> Result<()> {
    let mut series_list = load_series_list().map_err(|err| match err {
        UpNextError::IoError(e) if e.kind() == std::io::ErrorKind::NotFound => UpNextError::MissingSeries,
        _ => err,
    })?;
    let series = series_list.find_series_mut(&get_cwd()?);
    match series {
        Some(series) => {
            println!("{}", series);
            series.next_episode += n;
            save_series_list(&series_list)?;

            let series = series_list
                .find_series(&get_cwd()?)
                .ok_or_else(|| UpNextError::MissingSeries)?;
            Ok(println!("{}", series))
        }
        None => Err(UpNextError::MissingSeries),
    }
}

mod utils {
    use crate::errors::{Result, UpNextError};
    use crate::{persistence, utils};
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
