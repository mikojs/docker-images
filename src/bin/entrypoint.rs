use std::env;

use clap::{crate_version, Command, Arg};

use docker_images::utils::{Error, proxy_args, sub_process};

fn shift_args(args: &mut Vec<String>) -> String {
    let command = args[0].clone();

    args.remove(0);
    command
}

fn run_main_command(args: &mut Vec<String>) -> Result<(), Error> {
    sub_process::exec(
        &shift_args(args),
        args
            .iter()
            .map(AsRef::as_ref)
            .collect(),
    )?;
    Ok(())
}

fn main() -> Result<(), Error> {
    let matches = Command::new("entrypoint")
        .version(crate_version!())
        .about("Use this command in the dockerfile entrypoint command")
        .arg(
            Arg::new("main-command")
                .help("This command would be executed if the proxy arguments don't work")
                .required(true)
        )
        .arg(proxy_args::set_proxy_args(false))
        .get_matches();
    let mut main_args = shellwords::split(
        matches
            .value_of("main-command")
            .unwrap(),
    )
        .unwrap();
    let mut args: Vec<String> = proxy_args::get_values_from_proxy_args(&matches)
        .iter()
        .map(|s| s.to_string())
        .collect();

    if args.len() == 0 {
        run_main_command(&mut main_args)?;
        return Ok(());
    }

    let custom_command = shift_args(&mut args);

    if sub_process::command_exist(&custom_command) {
        sub_process::exec(
            &custom_command,
            args
                .iter()
                .map(AsRef::as_ref)
                .collect(),
        )?;
    } else {
        run_main_command(&mut main_args)?;
    }
    Ok(())
}
