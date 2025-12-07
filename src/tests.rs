use crate::tests::utils::test;

#[test]
fn test_init() {
    test("test_init", &vec!["init"]);
}

#[test]
fn test_init_twice() {
    test("test_init_twice", &vec!["init"]);
}

#[test]
fn test_increment_episode() {
    test("test_increment_episode", &vec!["inc", "2"]);
}

#[test]
fn test_increment_episode_default_value() {
    test("test_increment_episode_default_value", &vec!["inc"]);
}

#[test]
fn test_remove_from_empty() {
    test("test_remove_from_empty", &vec!["remove"]);
}

#[test]
fn test_print_series_info() {
    test("test_print_series_info", &vec!["info"]);
}

#[test]
fn test_set_next_episode() {
    test("test_set_next_episode", &vec!["set", "42"]);
}

#[cfg(test)]
mod utils {
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn build() {
        INIT.call_once(|| {
            Command::new("cargo")
                .args(["build"])
                .output()
                .expect("Failed to execute command");
        });
    }

    pub fn test(name: &str, args: &Vec<&str>) {
        build();
        let (toml_path, expected_stdout, expected_stderr, before, after) = read_test_files(name);
        let (stdout, stderr, file_content) = {
            if let Some(before) = before {
                set_toml_file(before, PathBuf::from(&toml_path));
            } else {
                delete_toml_file(PathBuf::from(&toml_path));
            }
            run_app(args, &toml_path)
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

        delete_toml_file(PathBuf::from(&toml_path));
    }

    fn cargo_manifest_dir() -> String {
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    }

    fn delete_toml_file(path: PathBuf) {
        if path.exists() {
            fs::remove_file(path).unwrap();
        }
    }

    fn set_toml_file(content: String, path: PathBuf) {
        fs::write(path, content).unwrap();
    }

    fn run_app(args: &Vec<&str>, toml_path: &String) -> (Vec<u8>, Vec<u8>, Option<String>) {
        let mut path = PathBuf::from(cargo_manifest_dir());
        path.push("target/debug/upnext");
        let output = Command::new(path)
            .args(args)
            .env(crate::TOML_PATH_ENV_VAR_NAME, toml_path.clone())
            .output()
            .expect("Failed to execute command");
        let file_content = fs::read_to_string(toml_path).unwrap();

        (output.stdout, output.stderr, Some(file_content))
    }

    /// returns `expected_stdout`, `expected_stderr`, .upnext.toml at start, .upnext.toml at end
    fn read_test_files(
        test_name: &str,
    ) -> (
        String,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
    ) {
        fn inject_path(content: &str) -> String {
            content.replace("PATH", &cargo_manifest_dir())
        }

        let path = {
            let mut path = PathBuf::from(cargo_manifest_dir());
            path.push("test-resources");
            path.push(test_name);
            path
        };

        let res: Vec<Option<String>> = (0..4)
            .map(|i| {
                let mut path = path.clone();
                path.push(match i {
                    0 => "stdout.txt",
                    1 => "stderr.txt",
                    2 => "before.toml",
                    3 => "after.toml",
                    _ => unreachable!(),
                });
                fs::read_to_string(&path).ok().map(|s| inject_path(&s))
            })
            .collect();

        let toml_path: String = {
            let mut toml_path = path.clone();
            toml_path.push("res.toml");
            toml_path.into_os_string().into_string().unwrap()
        };

        delete_toml_file(PathBuf::from(&toml_path));
        let number_of_files_in_dir = fs::read_dir(&path).unwrap().count();
        let number_of_identified_test_files = res.iter().filter(|x| x.is_some()).count();
        assert_eq!(
            number_of_files_in_dir, number_of_identified_test_files,
            "Not all files in dir are identified as test files"
        );
        assert_ne!(number_of_files_in_dir, 0, "No files in dir");

        (
            toml_path,
            res[0].clone(),
            res[1].clone(),
            res[2].clone(),
            res[3].clone(),
        )
    }
}
