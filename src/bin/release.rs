use std::env;
use std::fs;
use std::process;
use std::path::Path;

use regex::Regex;

#[path = "../utils/sub_process.rs"] mod sub_process;

fn get_command_help(command_name: &str) -> String {
    let command_file_path = env::current_exe()
        .expect("Couldn't get the current file path")
        .parent()
        .expect("Couldn't get the parent folder")
        .join(command_name);
    let content = sub_process::exec_result(
        &command_file_path
            .display()
            .to_string(),
        vec!["--help"],
    )
        .replace("`", "\\`");
    let prev_version = env::var("PREV_VERSION")
        .expect("Couldn't get the pervious version");
    let new_version = env::var("NEW_VERSION")
        .expect("Couldn't get the new version");
    let could_find_previous_version = Regex::new(&prev_version)
        .unwrap()
        .is_match(&content);

    if !could_find_previous_version {
        eprintln!("Couldn't find the previous version in `{}`", command_name);
        process::exit(1)
    }

    content.replace(&prev_version, &new_version)
}

fn validate_folder() {
    let exe_file_path = env::current_exe()
        .expect("Couldn't get the current executable file path");
    let exe_dir_path = exe_file_path
        .parent()
        .expect("Couldn't get the folder of the current executable file")
        .file_name();

    if exe_dir_path != Path::new("release").file_name() {
        eprintln!("Should use the production release command");
        process::exit(1);
    }
}

fn main() {
    validate_folder();

    let command_names = vec!["ddocker", "code", "node-parser"];
    let mut content = r#"# Docker images

Here are some helpful commands used in the docker container."#.to_string();

    for command_name in command_names {
        content.push_str(
            &format!(
                r#"

## {}

```
{}
```"#,
                command_name,
                get_command_help(command_name),
            ),
        );
    }

    fs::write("README.md", content)
        .expect("Couldn't write the README.md");
}
