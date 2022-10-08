use std::fs;

use regex::Regex;
use crate::utils::sub_process;

fn generate_env_content(content: String) -> String {
    let contents: Vec<&str> = content
        .split(" ")
        .filter(|x| !Regex::new(r"^PATH=.+").unwrap().is_match(x))
        .collect();

    contents.join("\n")
}

pub fn get(container_name: &str) -> String {
    let file_path = "/root/.ddocker.env";
    let content = generate_env_content(
        sub_process::exec_result(
            "docker",
            vec![
                "inspect",
                container_name,
                "--format",
                "{{.Config.Env}}",
            ],
        )
            .expect("TODO")
            .replace("[", "")
            .replace("]", "")
    );

    match fs::write(file_path, content) {
        Ok(_) => file_path.to_string(),
        _ => unreachable!(),
    }
}

#[test]
fn check_env_file() {
    let testings = vec![
        "PATH=foo env1=bar env2=bar",
        "env1=bar PATH=foo env2=bar",
        "env1=bar env2=bar PATH=foo",
    ];

    for testing in testings {
        assert_eq!(
            generate_env_content(testing.to_string()),
            r#"env1=bar
env2=bar"#,
        );
    }
}
