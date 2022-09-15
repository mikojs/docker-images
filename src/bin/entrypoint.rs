use std::env;
use std::process::Command;

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
    let mut args: Vec<String> = env::args()
        .collect();

    shift_args(&mut args);

    let mut main_args = shellwords::split(
      &shift_args(&mut args),
    )
      .expect("Couldn't get the commands");

    if args.len() == 0 {
        run_main_command(&mut main_args);
        return;
    }

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
}
