use serde::{Deserialize, Serialize};

use crate::errors::{Result, UpNextError};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Series {
    pub path: String,
    pub next_episode: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeriesList {
    pub series: Vec<Series>,
}

impl SeriesList {
    pub fn new() -> Self {
        SeriesList { series: Vec::new() }
    }

    pub fn add_series(&mut self, path: String) -> Result<()> {
        if self.series.iter().any(|s| s.path == path) {
            Err(UpNextError::SeriesAlreadyExists)?;
        }
        self.series.push(Series {
            path,
            next_episode: 1,
        });
        Ok(())
    }

    pub fn remove_series(&mut self, path: &str) {
        self.series.retain(|s| s.path != path);
    }

    pub fn find_series_index(&self, path: &str) -> Result<usize> {
        self.series
            .iter()
            .position(|s| s.path == path)
            .ok_or_else(|| UpNextError::MissingSeries)
    }

    pub fn at(&self, index: usize) -> Result<&Series> {
        self.series
            .get(index)
            .ok_or_else(|| UpNextError::MissingSeries)
    }

    pub fn at_mut(&mut self, index: usize) -> Result<&mut Series> {
        self.series
            .get_mut(index)
            .ok_or_else(|| UpNextError::MissingSeries)
    }

    pub fn find_series_mut(&mut self, path: &str) -> Result<&mut Series> {
        self.series
            .iter_mut()
            .find(|s| s.path == path)
            .ok_or_else(|| UpNextError::MissingSeries)
    }

    pub fn find_series(&self, path: &str) -> Result<&Series> {
        self.series
            .iter()
            .find(|s| s.path == path)
            .ok_or_else(|| UpNextError::MissingSeries)
    }

    pub fn contains_path(&self, path: &str) -> bool {
        self.series.iter().any(|s| s.path == path)
    }
}
