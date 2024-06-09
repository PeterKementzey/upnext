use crate::tests::utils::test;

#[test]
fn test_init() {
    test("test_init", vec!["init"]);
}

#[test]
fn test_init_twice() {
    test("test_init_twice", vec!["init"]);
}

#[test]
fn test_increment_episode() {
    test("test_increment_episode", vec!["inc", "2"]);
}

#[test]
fn test_increment_episode_default_value() {
    test("test_increment_episode_default_value", vec!["inc"]);
}

#[test]
fn test_remove_from_emtpy() {
    test("test_remove_from_empty", vec!["remove"]);
}

#[test]
fn test_print_series_info() {
    test("test_print_series_info", vec!["info"]);
}


#[cfg(test)]
mod utils {
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;

    static MUTEX: std::sync::Mutex<i32> = std::sync::Mutex::new(1);

    pub fn test(name: &str, args: Vec<&str>) {
        let (expected_stdout, expected_stderr, before, after) = read_test_files(name);
        let (stdout, stderr, file_content) = match MUTEX.lock() {
            Ok(_guard) => {
                if let Some(before) = before {
                    set_toml_file(before);
                } else {
                    delete_toml_file();
                }
                run_app(args)
            }
            Err(poisoned) => panic!("Mutex poisoned: {:?}", poisoned),
        };
        println!("stdout: {}", String::from_utf8_lossy(&stdout));
        println!("stderr: {}", String::from_utf8_lossy(&stderr));
        if let Some(expected_stdout) = expected_stdout {
            assert_eq!(String::from_utf8_lossy(&stdout), expected_stdout);
        }
        if let Some(expected_stderr) = expected_stderr {
            assert_eq!(String::from_utf8_lossy(&stderr), expected_stderr);
        }
        assert_eq!(file_content, after);
    }

    fn cargo_mainfest_dir() -> String {
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    }

    fn toml_file_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap();
        path.push(".upnext.toml");
        path
    }

    fn delete_toml_file() {
        let path = toml_file_path();
        if path.exists() {
            fs::remove_file(path).unwrap();
        }
    }

    fn set_toml_file(content: String) {
        fs::write(toml_file_path(), content).unwrap();
    }

    fn run_app(args: Vec<&str>) -> (Vec<u8>, Vec<u8>, Option<String>) {
        let mut path = PathBuf::from(cargo_mainfest_dir());
        path.push("target/debug/upnext");
        let output = Command::new(path)
            .args(args)
            .output()
            .expect("Failed to execute command");
        let file_content = fs::read_to_string(&toml_file_path()).unwrap();

        (output.stdout, output.stderr, Some(file_content))
    }

    /// returns expected_stdout, expected_stderr, .upnext.toml at start, .upnext.toml at end
    fn read_test_files(test_name: &str) -> (Option<String>, Option<String>, Option<String>, Option<String>) {
        fn inject_path(content: String) -> String {
            content.replace("PATH", &cargo_mainfest_dir())
        }

        let path = {
            let mut path = PathBuf::from(cargo_mainfest_dir());
            path.push("test-resources");
            path.push(test_name);
            path
        };

        let res: Vec<Option<String>> = (0..4).map(|i| {
            let mut path = path.clone();
            path.push(match i {
                0 => "stdout.txt",
                1 => "stderr.txt",
                2 => "before.toml",
                3 => "after.toml",
                _ => unreachable!(),
            });
            fs::read_to_string(&path).map(inject_path).ok()
        }).collect();

        let number_of_files_in_dir = fs::read_dir(&path).unwrap().count();
        let number_of_identified_test_files = res.iter().filter(|x| x.is_some()).count();
        assert_eq!(number_of_files_in_dir, number_of_identified_test_files, "Not all files in dir are identified as test files");
        assert_ne!(number_of_files_in_dir, 0, "No files in dir");

        (res[0].clone(), res[1].clone(), res[2].clone(), res[3].clone())
    }
}
