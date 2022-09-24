use std::fs;

use regex::Regex;

#[path = "./sub_process.rs"] mod sub_process;
#[path = "./get_container_name.rs"] mod get_container_name;
#[path = "./get_working_dir.rs"] mod get_working_dir;

fn get_network_name(container_name: &str) -> String {
    sub_process::exec_result(
        "docker",
        vec![
            "inspect",
            container_name,
            "--format",
            "{{.HostConfig.NetworkMode}}",
        ],
    )
        .replace("\n", "")
}

fn get_env_file(container_name: &str) -> String {
    let file_path = "/root/.ddocker.env";
    let mut content = sub_process::exec_result(
        "docker",
        vec![
            "inspect",
            container_name,
            "--format",
            "{{.Config.Env}}",
        ],
    )
        .replace("[", "")
        .replace("]", "");

    content = Regex::new(r"PATH=[^ ]+ ").unwrap()
        .replace_all(&content, "")
        .to_string()
        .replace(" ", "\n");

    match fs::write(file_path, content) {
        Ok(_) => file_path.to_string(),
        _ => unreachable!(),
    }
}

fn filter_args(args: Vec<&str>) -> Vec<&str> {
    if args[1].is_empty() {
        return vec![];
    }

    args
}

pub fn main(args: Vec<&str>) {
    let container_name = get_container_name::main();

    sub_process::exec(
        "docker",
        [
            vec![
                "run",
                "-w",
                &get_working_dir::main(),
                "--env-file",
                &get_env_file(&container_name),
            ],
            filter_args(
                vec!["--volumes-from", &container_name],
            ),
            filter_args(
                vec![
                    "--network",
                    &get_network_name(&container_name),
                ],
            ),
            args,
        ]
            .concat(),
    );
}
