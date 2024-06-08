use utils::*;

#[test]
fn test_init() {
    delete_toml_file();
    run_app(vec!["init"], Some(&format!(
        "path = \"{}\"
next_episode = 1\n\n", cargo_mainfest_dir())), Some(""));
    assert_toml_file_is(format!(
        "[[series]]
path = \"{}\"
next_episode = 1
", cargo_mainfest_dir()));
}

#[test]
fn test_init_twice() {
    delete_toml_file();
    test_init();
    run_app(vec!["init"], Some(""), Some("Current directory is already initialized.\n"));
    assert_toml_file_is(format!(
        "[[series]]
path = \"{}\"
next_episode = 1
", cargo_mainfest_dir()));
}

#[test]
fn test_increment_episode() {
    delete_toml_file();
    set_toml_file(format!(
        "# 1
[[series]] # 2
# 3
     # 4
       #5
       path = \"{}\" # 6666
       # 7
       next_episode = 1 # 8
       # 9
       ", cargo_mainfest_dir()));
    run_app(vec!["inc", "2"], Some("Incrementing episode by 2\n"), None);
    assert_toml_file_is(format!(
        "# 1
[[series]] # 2
# 3
     # 4
       #5
       path = \"{}\" # 6666
       # 7
       next_episode = 3 # 8
       # 9
       ",
        cargo_mainfest_dir()));
}


#[cfg(test)]
mod utils {
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;

    pub(super) fn cargo_mainfest_dir() -> String {
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    }

    pub(super) fn delete_toml_file() {
        let path = toml_file_path();
        if path.exists() {
            fs::remove_file(path).unwrap();
        }
    }

    pub(super) fn set_toml_file(content: String) {
        fs::write(toml_file_path(), content).unwrap();
    }

    pub(super) fn assert_toml_file_is(content: String) {
        let file_content = read_file();
        assert_eq!(file_content, content);
    }

    pub(super) fn run_app(args: Vec<&str>, expected_stdout: Option<&str>, expected_stderr: Option<&str>) {
        let mut path = PathBuf::from(cargo_mainfest_dir());
        path.push("target/debug/upnext");
        println!("path: {:?}", path);
        let output = Command::new(path)
            .args(args)
            .output()
            .expect("Failed to execute command");

        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

        if let Some(expected_output) = expected_stdout {
            assert_eq!(&output.stdout[..], expected_output.as_bytes());
        }
        if let Some(expected_output) = expected_stderr {
            assert_eq!(&output.stderr[..], expected_output.as_bytes());
        }
    }

    fn toml_file_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap();
        path.push(".upnext.toml");
        path
    }

    fn read_file() -> String {
        fs::read_to_string(toml_file_path()).unwrap()
    }
}
