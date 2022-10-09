use std::fs;

use regex::Regex;

use crate::utils::{sub_process, Error};

fn generate_env_content(content: String) -> Result<String, Error> {
    let path_regex = Regex::new(r"^PATH=.+")?;
    let contents: Vec<&str> = content
        .split(" ")
        .filter(|x| !path_regex.is_match(x))
        .collect();

    Ok(contents.join("\n"))
}

pub fn get(container_name: &str) -> Result<String, Error> {
    let file_path = "/root/.ddocker.env";
    let content = generate_env_content(
        sub_process::exec_result(
            "docker",
            vec!["inspect", container_name, "--format", "{{.Config.Env}}"],
        )?
        .replace("[", "")
        .replace("]", ""),
    )?;

    fs::write(file_path, content)?;
    Ok(file_path.to_string())
}

#[test]
fn check_env_file() -> Result<(), Error> {
    let testings = vec![
        "PATH=foo env1=bar env2=bar",
        "env1=bar PATH=foo env2=bar",
        "env1=bar env2=bar PATH=foo",
    ];

    for testing in testings {
        assert_eq!(
            generate_env_content(testing.to_string())?,
            r#"env1=bar
env2=bar"#,
        );
    }
    Ok(())
}
