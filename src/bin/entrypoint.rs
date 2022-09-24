use std::env;
use std::process;
use clap::{crate_version, Command, Arg};

#[allow(dead_code)]
#[path = "../utils/sub_process.rs"] mod sub_process;
#[path = "../utils/proxy_args.rs"] mod proxy_args;

fn shift_args(args: &mut Vec<String>) -> String {
    let command = args[0].clone();

    args.remove(0);
    command
}

fn run_main_command(args: &mut Vec<String>) {
    sub_process::exec(
        &shift_args(args),
        args
            .iter()
            .map(AsRef::as_ref)
            .collect(),
    );
}

fn main() {
    let matches = Command::new("entrypoint")
        .version(crate_version!())
        .about("Use this command in the dockerfile entrypoint command")
        .arg(
            Arg::new("main-command")
                .required(true)
        )
        .arg(proxy_args::set_proxy_args(false))
        .get_matches();
    let mut main_args = shellwords::split(
        matches
            .value_of("main-command")
            .unwrap(),
    )
      .expect("Couldn't get the commands");
    let mut args: Vec<String> = proxy_args::get_values_from_proxy_args(&matches)
        .iter()
        .map(|s| s.to_string())
        .collect();

    if args.len() == 0 {
        run_main_command(&mut main_args);
        return;
    }

    let custom_command = shift_args(&mut args);

    match process::Command::new(&custom_command).output() {
        Ok(_) => sub_process::exec(
            &custom_command,
            args
                .iter()
                .map(AsRef::as_ref)
                .collect(),
        ),
        Err(_) => run_main_command(&mut main_args),
    }
}
