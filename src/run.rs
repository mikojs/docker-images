use clap::{Command, ArgMatches};

#[path = "./utils/sub_process.rs"] mod sub_process;
#[path = "./utils/args.rs"] mod args;
#[path = "./utils/get_container_name.rs"] mod get_container_name;

pub fn command() -> Command<'static> {
    Command::new("run")
        .about(r#"This command would mount the same volumes to the current container
When the current path is under `/root`, a new container would use the same path as the working directory
Otherwise, this would change to be `/root`"#)
        .arg(args::set_proxy_arg(true))
}

fn get_network_name() -> String {
    sub_process::exec_result(
        "docker",
        vec![
            "inspect",
            &get_container_name::main(),
            "--format",
            "{{.HostConfig.NetworkMode}}",
        ],
    )
        .replace("\n", "")
}

pub fn execute(matches: &ArgMatches) {
    sub_process::exec(
        "docker",
        [
            vec![
                "run",
                "-w",
                &args::get_working_directory(),
            ],
            args::filter_args(
                vec![
                    "--volumes-from",
                    &get_container_name::main(),
                ],
            ),
            args::filter_args(
                vec![
                    "--network",
                    &get_network_name(),
                ],
            ),
            args::get_values_from_args(matches),
        ]
            .concat(),
    );
}
