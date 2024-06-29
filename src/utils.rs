use crate::errors::UpNextError;

pub fn get_toml_path() -> crate::errors::Result<String> {
    match std::env::var("UPNEXT_CONFIG_PATH") {
        Ok(path) => Ok(path),
        Err(_) => match std::env::var("HOME") {
            Ok(home) => Ok(format!("{home}/.upnext.toml")),
            Err(err) => Err(UpNextError::GenericError(format!("Could not get home dir: {err}"))),
        },
    }
}
