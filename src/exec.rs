use clap::{Command, ArgMatches};

#[allow(dead_code)]
#[path = "./utils/sub_process.rs"] mod sub_process;
#[allow(dead_code)]
#[path = "./utils/args.rs"] mod args;
#[path = "./utils/get_working_dir.rs"] mod get_working_dir;

pub fn command() -> Command<'static> {
    Command::new("exec")
        .about(r#"This command would set the working directory with `docker exec`
When the current path is under `/root/work`, the same path would be the initial working directory
Otherwise, this would change to be `/root/work`"#)
        .arg(args::set_proxy_arg(true))
}

pub fn execute(matches: &ArgMatches) {
    sub_process::exec(
        "docker",
        [
            vec![
                "exec",
                "-w",
                &get_working_dir::main(),
            ],
            args::get_values_from_args(matches),
        ]
            .concat(),
    );
}
