use std::borrow::Cow;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};
use toml_edit::{ArrayOfTables, DocumentMut, Item, value};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Series {
    pub(crate) path: String,
    pub(crate) next_episode: i64,
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
    // Read the existing TOML file to preserve comments or create new document
    let mut doc = {
        let content = fs::read_to_string(&path);
        match content {
            Ok(content) => content.parse::<DocumentMut>().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    Ok(DocumentMut::default())
                } else {
                    Err(e)
                }
            }
        }
    }?;

    let old_array_of_series = {
        match doc.get("series").map(|t| t.as_array_of_tables().expect("series is not array of tables")) {
            Some(series_array) => Cow::Borrowed(series_array),
            None => Cow::Owned(ArrayOfTables::new()),
        }
    };
    let mut new_array_of_series = ArrayOfTables::new();

    // Create new TOML file
    // Since we go through the series from the function argument, removed series do not get copied over
    for series in &series_list.series {
        // Get old table or create a new one by path
        let mut series_table = (*old_array_of_series).iter().find_map(|table| {
            if table["path"].as_str().expect("path is not a string") == series.path {
                Some(table.clone())
            } else {
                None
            }
        }).unwrap_or_else(|| {
            let mut table = toml_edit::Table::new();
            table["path"] = value(&series.path);
            table
        });

        // Update or create next_episode field
        if let Some(next_episode_item) = series_table.get_mut("next_episode") {
            // Get decoration
            let decor = next_episode_item.as_value().expect("next_episode is not value").decor().clone();
            // Update value
            *next_episode_item = value(series.next_episode);
            // Reapply decoration
            if let Some(new_value) = next_episode_item.as_value_mut() {
                *new_value.decor_mut() = decor;
            }
        } else {
            series_table["next_episode"] = value(series.next_episode);
        }

        // Add table to new file (preserving order)
        new_array_of_series.push(series_table);
    }

    // Update the document
    doc["series"] = Item::ArrayOfTables(new_array_of_series);

    let mut file = fs::File::create(path)?;
    file.write_all(doc.to_string().as_bytes())?;
    Ok(())
}

impl SeriesList {
    pub(crate) fn new() -> Self {
        SeriesList { series: Vec::new() }
    }
}
