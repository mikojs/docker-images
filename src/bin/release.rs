use std::env;
use std::fs;

#[allow(dead_code)]
#[path = "../utils/sub_process.rs"] mod sub_process;

fn get_command_help(command_name: &str) -> String {
    let command_file_path = env::current_exe()
        .expect("Couldn't get the current file path")
        .parent()
        .expect("Couldn't get the parent folder")
        .join(command_name);

    sub_process::exec_result(
        &command_file_path
            .display()
            .to_string(),
        vec!["--help"],
    )
        .replace("`", "\\`")
}

fn main() {
    let command_names = vec!["ddocker", "code", "node-parser"];
    let mut content = r#"# Docker images

Here are some helpful commands used in the docker container."#.to_string();

    for command_name in command_names {
        let new_content = format!(
            r#"

## {}

```
{}
```"#,
            command_name,
            get_command_help(command_name),
        );

        content.push_str(&new_content);
    }

    fs::write("README.md", content)
        .expect("Couldn't write the README.md");
}
