use std::fmt::Display;

use crate::errors::UpNextError;
use crate::schema::Series;
use crate::persistence;

pub fn get_toml_path() -> crate::errors::Result<String> {
    match std::env::var("HOME") {
        Ok(home) => Ok(format!("{}/.upnext.toml", home)),
        Err(err) => Err(UpNextError::GenericError(format!("Could not get home dir: {}", err.to_string()))),
    }
}

impl Display for Series {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn get_toml_table(s: &Series) -> crate::errors::Result<toml_edit::Table> {
            // Read the existing TOML file to preserve comments or create new document
            let path = get_toml_path()?;
            let doc = persistence::utils::parse_toml_doc_from_path(path)?;

            let old_array_of_series = persistence::utils::get_or_create_series_table(&doc)?;
            let mut series_table = persistence::utils::get_table_by_path_or_create_new(s, &old_array_of_series)?;
            persistence::utils::set_or_create_next_episode(s.next_episode, &mut series_table)?;
            Ok(series_table)
        }
        let toml_data = get_toml_table(self).map_err(|_| core::fmt::Error)?;
        write!(f, "[[series]]\n{}", toml_data)
    }
}
