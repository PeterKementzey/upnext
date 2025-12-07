use std::{fmt::Display, fs, io, io::Write, path::Path};

use toml_edit::{value, ArrayOfTables, DocumentMut, Table};

use crate::errors::{Result, UpNextError};
use crate::schema::{Series, SeriesList};

pub fn read_toml_file<P: AsRef<Path>>(path: P) -> Result<SeriesList> {
    let content = fs::read_to_string(path)?;
    if content.is_empty() {
        return Ok(SeriesList::new());
    }
    let series_list: SeriesList = toml::from_str(&content)
        .map_err(|err| UpNextError::SchemaError(err.message().to_string()))?;
    Ok(series_list)
}

// Read the existing TOML file for the comments and formatting. Then apply
// changes from `series_list` while preserving comments. Alternatively create
// a new TOML DocumentMut if file doesn't exist.
pub fn write_toml_file<P: AsRef<Path>>(path: P, series_list: &SeriesList) -> Result<()> {
    let mut doc = create_or_load_toml_doc(&path)?;

    update_or_create_list_of_series(&mut doc, series_list)?;

    let mut file = fs::File::create(path)?;
    file.write_all(doc.to_string().as_bytes())?;
    Ok(())
}

fn update_or_create_list_of_series(doc: &mut DocumentMut, series_list: &SeriesList) -> Result<()> {
    let array_of_series: &mut ArrayOfTables = get_or_create_array_of_series(doc)?;

    remove_deleted_series(array_of_series, series_list);

    for series in &series_list.series {
        let series_table: &mut Table = get_or_create_series_table(array_of_series, series)?;
        update_or_create_next_episode(series_table, series.next_episode)?;
    }

    Ok(())
}

impl Display for Series {
    // Read the existing TOML file to include comments or create new document
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let series = self;
        let path = crate::utils::get_toml_path().map_err(|_| core::fmt::Error)?;
        let mut doc = create_or_load_toml_doc(path).map_err(|_| core::fmt::Error)?;
        let array_of_series: &mut ArrayOfTables =
            get_or_create_array_of_series(&mut doc).map_err(|_| core::fmt::Error)?;
        let series_table: &mut Table =
            get_or_create_series_table(array_of_series, series).map_err(|_| core::fmt::Error)?;
        update_or_create_next_episode(series_table, series.next_episode)
            .map_err(|_| core::fmt::Error)?;
        let toml_data = series_table;
        write!(f, "[[series]]\n{toml_data}")
    }
}

fn create_or_load_toml_doc<P: AsRef<Path>>(path: P) -> Result<DocumentMut> {
    let content = fs::read_to_string(&path);
    let doc = match content {
        Ok(content) => content.parse::<DocumentMut>()?,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => DocumentMut::default(),
            _ => Err(e)?,
        },
    };
    Ok(doc)
}

fn get_or_create_array_of_series(doc: &mut DocumentMut) -> Result<&mut ArrayOfTables> {
    if doc.contains_key("series") {
        let array_of_series = doc
            .get_mut("series")
            .expect("in this if branch it has to be present");

        array_of_series
            .as_array_of_tables_mut()
            .ok_or(UpNextError::SchemaError(
                "Value of 'series' key is not array of tables".to_string(),
            ))
    } else {
        doc["series"] = toml_edit::array();
        let res = doc
            .get_mut("series")
            .expect("created just now")
            .as_array_of_tables_mut()
            .expect("created just now as array of tables");
        Ok(res)
    }
}

fn remove_deleted_series(array_of_series: &mut ArrayOfTables, series_list: &SeriesList) -> () {
    array_of_series.retain(|table| {
        table
            .get("path")
            .and_then(|path| path.as_str())
            .map(|path| series_list.contains_path(path))
            .unwrap_or(false)
    })
}

fn get_or_create_series_table<'a>(
    array_of_series: &'a mut ArrayOfTables,
    series: &Series,
) -> Result<&'a mut Table> {
    fn get_path_from_series_toml_table(table: &Table) -> Result<&str> {
        match table.get("path") {
            None => Err(UpNextError::SchemaError("Series has no path".to_string())),
            Some(path) => match path.as_str() {
                None => Err(UpNextError::SchemaError("Path is not string".to_string())),
                Some(path) => Ok(path),
            },
        }
    }

    fn check_if_already_exists(
        array_of_series: &mut ArrayOfTables,
        series: &Series,
    ) -> Result<bool> {
        let already_exists_search_result =
            array_of_series
                .iter()
                .find_map(|table| match get_path_from_series_toml_table(table) {
                    Err(e) => Some(Err(e)),
                    Ok(path) if path == series.path => Some(Ok(true)),
                    Ok(_) => None,
                });

        match already_exists_search_result {
            Some(Err(e)) => Err(e),
            Some(Ok(already_exists)) => Ok(already_exists),
            None => Ok(false),
        }
    }

    if check_if_already_exists(array_of_series, series)? {
        let existing_table = array_of_series
            .iter_mut()
            .find(|table| {
                get_path_from_series_toml_table(table).expect(
                    "already checked path can be parsed in all tables in `check_if_already_exists`",
                ) == series.path
            })
            .expect("already checked series with this path exists in `check_if_already_exists`");
        Ok(existing_table)
    } else {
        let mut table = toml_edit::Table::new();
        table["path"] = toml_edit::value(&series.path);
        array_of_series.push(table);
        let new_table = array_of_series
            .iter_mut()
            .last()
            .expect("just added a table");
        Ok(new_table)
    }
}

fn update_or_create_next_episode(
    series_table: &mut toml_edit::Table,
    next_episode: i64,
) -> Result<()> {
    if let Some(next_episode_item) = series_table.get_mut("next_episode") {
        // Get decoration
        let decor = next_episode_item
            .as_value()
            .ok_or_else(|| UpNextError::SchemaError("next_episode is not a value".to_string()))?
            .decor()
            .clone();
        // Update value
        *next_episode_item = value(next_episode);
        // Reapply decoration
        if let Some(new_value) = next_episode_item.as_value_mut() {
            Ok(*new_value.decor_mut() = decor)
        } else {
            Err(UpNextError::SchemaError(
                "next_episode_item is not value".to_string(),
            ))
        }
    } else {
        Ok(series_table["next_episode"] = value(next_episode))
    }
}
