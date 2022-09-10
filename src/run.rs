use clap::{Command, ArgMatches};

#[path = "./utils/sub_process.rs"] mod sub_process;
#[path = "./utils/args.rs"] mod args;

pub fn command() -> Command<'static> {
    Command::new("run")
        .about(r#"This command would mount the same volumes to the current container
When the current path is under `/project`, a new container would use the same path as the working directory
Otherwise, this would change to be `/project`"#)
        .arg(args::set_proxy_arg())
}

fn get_network_name() -> String {
    sub_process::exec_result(
        "docker",
        vec![
            "inspect",
            &args::get_container_name(),
            "--format",
            "{{.HostConfig.NetworkMode}}",
        ],
    )
        .replace("\n", "")
}

fn filter_args(args: Vec<&str>) -> Vec<&str> {
    if args[1].is_empty() {
        return vec![];
    }

    args
}

pub fn execute(sub_matches: &ArgMatches) {
    sub_process::exec(
        "docker",
        [
            vec![
                "run",
                "-w",
                &args::get_working_directory(),
            ],
            filter_args(
                vec![
                    "--volumes-from",
                    &args::get_container_name(),
                ],
            ),
            filter_args(
                vec![
                    "--network",
                    &get_network_name(),
                ],
            ),
            args::get_values_from_args(sub_matches),
        ]
            .concat(),
    );
}
