use utils::*;

#[cfg(test)]
mod utils {
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;

    fn toml_file_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap();
        path.push(".upnext.toml");
        path
    }

    fn read_file() -> String {
        fs::read_to_string(toml_file_path()).unwrap()
    }


    pub(crate) fn cargo_mainfest_dir() -> String {
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    }

    pub(super) fn delete_file() {
        let path = toml_file_path();
        if path.exists() {
            fs::remove_file(path).unwrap();
        }
    }

    pub(super) fn set_file(content: String) {
        fs::write(toml_file_path(), content).unwrap();
    }

    pub(super) fn assert_file_is(content: String) {
        let file_content = read_file();
        assert_eq!(file_content, content);
    }

    pub(super) fn run_upnext(args: Vec<&str>, expected_output: Option<&str>) {
        let mut path = PathBuf::from(cargo_mainfest_dir());
        path.push("target/debug/upnext");
        println!("path: {:?}", path);
        let output = Command::new(path)
            .args(args)
            .output()
            .expect("Failed to execute command");

        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));

        if let Some(expected_output) = expected_output {
            assert_eq!(&output.stdout[..], expected_output.as_bytes());
        }
    }
}

#[test]
fn test_init() {
    delete_file();
    run_upnext(vec!["init"], Some("Initializing series\n"));
    assert_file_is(format!(
        "[[series]]
path = \"{}\"
next_episode = 1
", cargo_mainfest_dir()));
}


#[test]
fn test_init_twice() {
    delete_file();
    run_upnext(vec!["init"], Some("Initializing series\n"));
    run_upnext(vec!["init"], None);
    assert_file_is(format!(
        "[[series]]
path = \"{}\"
next_episode = 1
", cargo_mainfest_dir()));
}

#[test]
fn test_increment_episode() {
    set_file(format!(
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
    run_upnext(vec!["inc", "2"], Some("Incrementing episode by 2\n"));
    assert_file_is(
        format!(
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
