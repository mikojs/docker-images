use std::env;
use clap::{crate_version, Command, Arg};

#[allow(dead_code)]
#[path = "../utils/sub_process.rs"] mod sub_process;

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
        .arg(
            Arg::new("args")
                .multiple_values(true)
                .allow_hyphen_values(true)
        )
        .get_matches();
    let mut main_args = shellwords::split(
        matches
            .value_of("main-command")
            .unwrap(),
    )
      .expect("Couldn't get the commands");

    if matches.value_of("args").is_none() {
        run_main_command(&mut main_args);
        return;
    }

    /*
    let custom_command = shift_args(&mut args);

    match Command::new(&custom_command).output() {
        Ok(_) => sub_process::exec(
            &custom_command,
            args
                .iter()
                .map(AsRef::as_ref)
                .collect(),
        ),
        Err(_) => run_main_command(&mut main_args),
    }
    */
}
