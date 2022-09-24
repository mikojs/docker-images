use std::fs;

use clap::{Command, ArgMatches};
use regex::Regex;

#[path = "./utils/sub_process.rs"] mod sub_process;
#[path = "./utils/args.rs"] mod args;
#[path = "./utils/get_container_name.rs"] mod get_container_name;
#[path = "./utils/get_working_dir.rs"] mod get_working_dir;

pub fn command() -> Command<'static> {
    Command::new("run")
        .about(r#"This command would mount the same volumes to the current container
When the current path is under `/root/work`, a new container would use the same path as the working directory
Otherwise, this would change to be `/root/work`"#)
        .arg(args::set_proxy_arg(true))
}

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

pub fn execute(matches: &ArgMatches) {
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
            args::filter_args(
                vec!["--volumes-from", &container_name],
            ),
            args::filter_args(
                vec![
                    "--network",
                    &get_network_name(&container_name),
                ],
            ),
            args::get_values_from_args(matches),
        ]
            .concat(),
    );
}
