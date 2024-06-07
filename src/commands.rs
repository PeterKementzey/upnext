use crate::persistence::{Series, SeriesList};

impl SeriesList {
    pub(crate) fn init_series(&mut self) {
        let cwd = std::env::current_dir().unwrap().to_str().unwrap().to_string();
        self.series.push(Series { path: cwd, next_episode: 1 });
    }
}
