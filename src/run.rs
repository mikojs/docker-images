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
        &[
            "inspect",
            args::get_container_name(),
            "--format",
            "{{.HostConfig.NetworkMode}}",
        ],
    )
        .replace("\n", "")
}

fn get_volumes_from_args() -> Vec<String> {
    let mut args: Vec<String> = []
        .to_vec();
    let container_name = args::get_container_name();

    if !container_name.is_empty() {
        args.push("--volumes-from".to_string());
        args.push(container_name);
    }

    args
}

fn get_container_network_args() -> Vec<String> {
    let mut args: Vec<String> = []
        .to_vec();
    let network = sub_process::exec_result(
        "docker",
        &[
            "inspect",
            // container name
            "--format",
            "{{.HostConfig.NetworkMode}}",
        ],
    );

    if !network.is_empty() {
        args.push("--network".to_string());
        args.push(network.replace("\n", ""));
    }

    args
}

pub fn execute(sub_matches: &ArgMatches) {
    println!(">>>> {:?}", get_container_network_args());

    sub_process::exec(
        "docker",
        [
            vec![
                "run",
                "-w",
                &args::get_working_directory(),
            ],
            get_volumes_from_args()
                .iter()
                .map(AsRef::as_ref)
                .collect(),
            sub_matches
                .values_of("args")
                .unwrap()
                .collect(),
        ]
            .concat()
            .as_slice(),
    );
}
