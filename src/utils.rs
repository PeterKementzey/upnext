use crate::errors::UpNextError;

// If you change this, make sure to update the documentation in the `main.rs` file.
pub(crate) static TOML_PATH_ENV_VAR_NAME: &str = "UPNEXT_TOML_PATH";

pub(crate) fn get_toml_path() -> crate::errors::Result<String> {
    match std::env::var(TOML_PATH_ENV_VAR_NAME) {
        Ok(path) => Ok(path),
        Err(_) => match std::env::var("HOME") {
            Ok(home) => Ok(format!("{home}/.upnext.toml")),
            Err(err) => Err(UpNextError::GenericError(format!("Could not get home dir: {err}"))),
        },
    }
}
