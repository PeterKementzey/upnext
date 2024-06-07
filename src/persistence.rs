use std::fs;
use std::io::{self, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Series {
    pub(crate) path: String,
    pub(crate) next_episode: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SeriesList {
    pub(crate) series: Vec<Series>,
}

pub fn read_toml_file<P: AsRef<Path>>(path: P) -> Result<SeriesList, io::Error> {
    let content = fs::read_to_string(path)?;
    let series_list: SeriesList = toml::from_str(&content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(series_list)
}

pub(crate) fn write_toml_file<P: AsRef<Path>>(path: P, series_list: &SeriesList) -> Result<(), io::Error> {
    
    
    
    let toml_string = toml::to_string(series_list)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let mut file = fs::File::create(path)?;
    file.write_all(toml_string.as_bytes())?;
    Ok(())
}

impl SeriesList {
    pub(crate) fn new() -> Self {
        SeriesList { series: Vec::new() }
    }
}


