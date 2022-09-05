use clap::{Command, ArgMatches};

#[allow(dead_code)]
#[path = "./utils/sub_process.rs"] mod sub_process;
#[allow(dead_code)]
#[path = "./utils/args.rs"] mod args;

pub fn command() -> Command<'static> {
    Command::new("exec")
        .about(r#"This command would set the working directory with `docker exec`
When the current path is under `/project`, the same path would be the initial working directory
Otherwise, this would change to be `/project`"#)
        .arg(args::set_proxy_arg())
}

pub fn execute(sub_matches: &ArgMatches) {
    sub_process::exec(
        "docker",
        [
            vec![
                "exec",
                "-w",
                &args::get_working_directory(),
            ],
            sub_matches
                .values_of("args")
                .unwrap()
                .collect(),
        ]
            .concat()
            .as_slice(),
    );
}
