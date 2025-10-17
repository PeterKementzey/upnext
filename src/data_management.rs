pub(crate) mod persistence {
    use std::fs;
    use std::io::Write;
    use std::path::Path;

    use toml_edit::{ArrayOfTables, DocumentMut, Item};

    use crate::data_management::utils::{self, update_or_create_series_list};
    use crate::errors::{Result, UpNextError};
    use crate::schema::SeriesList;

    pub fn read_toml_file<P: AsRef<Path>>(path: P) -> Result<SeriesList> {
        let content = fs::read_to_string(path)?;
        if content.is_empty() {
            return Ok(SeriesList::new());
        }
        let series_list: SeriesList = toml::from_str(&content)
            .map_err(|err| UpNextError::SchemaError(err.message().to_string()))?;
        Ok(series_list)
    }

    // Read the existing TOML file to preserve comments or create new document
    // Update the TOML file to values in `series_list`
    pub fn write_toml_file<P: AsRef<Path>>(path: P, series_list: &SeriesList) -> Result<()> {
        let mut doc = utils::parse_toml_doc_from_path(&path)?;

        // Update the series list section of document
        update_or_create_series_list(&mut doc, series_list)?;

        let mut file = fs::File::create(path)?;
        file.write_all(doc.to_string().as_bytes())?;
        Ok(())
    }
}

mod formatting {
    use std::fmt::Display;

    use crate::data_management::utils;
    use crate::schema::Series;

    impl Display for Series {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            fn get_toml_table(s: &Series) -> crate::errors::Result<toml_edit::Table> {
                // Read the existing TOML file to preserve comments or create new document
                let path = crate::utils::get_toml_path()?;
                let doc = utils::parse_toml_doc_from_path(path)?;

                let array_of_series = utils::get_or_create_series_table(&doc)?;
                let mut series_table = utils::update_or_create_table_by_path(s, &array_of_series)?;
                utils::update_or_create_next_episode(s.next_episode, &mut series_table)?;
                Ok(series_table)
            }
            let toml_data = get_toml_table(self).map_err(|_| core::fmt::Error)?;
            write!(f, "[[series]]\n{toml_data}")
        }
    }
}

mod utils {
    use std::{fs, io};
    use std::borrow::Cow;
    use std::path::Path;

    use toml_edit::{value, ArrayOfTables, DocumentMut, Item};

    use crate::errors::{Result, UpNextError};
    use crate::schema::{Series, SeriesList};

    pub(super) fn parse_toml_doc_from_path<P: AsRef<Path>>(path: P) -> Result<DocumentMut> {
        let content = fs::read_to_string(&path);
        let doc = match content {
            Ok(content) => content.parse::<DocumentMut>(),
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    Ok(DocumentMut::default())
                } else {
                    Err(e)?
                }
            }
        }?;
        Ok(doc)
    }

    pub(super) fn update_or_create_series_list(doc: &mut DocumentMut, series_list: &SeriesList) -> Result<()> {
        // Get array of series or create new
        let array_of_series = match doc.get_mut("series") {
            Some(series_array) => {
                series_array.as_array_of_tables_mut().ok_or_else(|| UpNextError::SchemaError("Cannot parse series array".to_string()))
            }
            None => {
                doc["series"] = toml_edit::array();
                Ok(doc.get_mut("series").expect("created rn").as_array_of_tables_mut().expect("created now as array of tables"))
            }
        }?;

        // Remove series that are no longer present
        array_of_series.retain(|table| series_list.contains_path(table["path"].as_str().expect("TODO")));

        // Update series
        for series in &series_list.series {
            // Get old table or create a new one by path
            let series_table = (*array_of_series).iter_mut().find_map(|table| {
                match table.get_mut("path") {
                    Some(path) if path.as_str() == series.path => Some(Ok(table.clone())),
                    Some(_) => None,
                    None => Some(Err(UpNextError::SchemaError("Series path is not a string".to_string()))),
                }
            }).unwrap_or_else(|| {
                let mut table = toml_edit::Table::new();
                table["path"] = toml_edit::value(&series.path);
                Ok(table)
            })?;
        // Update next episode
        update_or_create_next_episode(series.next_episode, &mut series_table)?;

            // Add table to new file (preserving order)
            new_array_of_series.push(series_table);
        }

        Ok(new_array_of_series)
        }

    pub(super) fn get_or_create_series_table(doc: &DocumentMut) -> Result<Cow<ArrayOfTables>> {
        Ok(
            match doc.get("series") {
                Some(series_array) => {
                    let series_array = series_array.as_array_of_tables().ok_or_else(|| UpNextError::SchemaError("Cannot parse series array".to_string()))?;
                    Cow::Borrowed(series_array)
                }
                None => Cow::Owned(ArrayOfTables::new())
            })
    }

    pub(super) fn update_or_create_table_by_path(series: &Series, array_of_series: &ArrayOfTables) -> Result<toml_edit::Table> {
        // Get series with path
        let mut series_table = (*array_of_series).iter().find_map(|table| {
            match table["path"].as_str() {
                Some(path) if path == series.path => Some(Ok(table.clone())),
                Some(_) => None,
                None => Some(Err(UpNextError::SchemaError("Series path is not a string".to_string()))),
            }
        }).unwrap_or_else(|| {
            let mut table = toml_edit::Table::new();
            table["path"] = value(&series.path);
            Ok(table)
        })?;
        // Update next episode
        update_or_create_next_episode(series.next_episode, &mut series_table)?;
        
        Ok(series_table)
    }

    pub(super) fn update_or_create_next_episode(next_episode: i64, series_table: &mut toml_edit::Table) -> Result<()> {
        // Update or create next_episode field
        if let Some(next_episode_item) = series_table.get_mut("next_episode") {
            // Get decoration
            let decor = next_episode_item.as_value().ok_or_else(|| UpNextError::SchemaError("next_episode is not a value".to_string()))?.decor().clone();
            // Update value
            *next_episode_item = value(next_episode);
            // Reapply decoration
            if let Some(new_value) = next_episode_item.as_value_mut() {
                *new_value.decor_mut() = decor;
            }
        } else {
            series_table["next_episode"] = value(next_episode);
        };
        Ok(())
    }
}
