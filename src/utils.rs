use crate::errors::UpNextError;

pub fn get_toml_path() -> crate::errors::Result<String> {
    match std::env::var("HOME") {
        Ok(home) => Ok(format!("{}/.upnext.toml", home)),
        Err(err) => Err(UpNextError::GenericError(format!("Could not get home dir: {}", err.to_string()))),
    }
}