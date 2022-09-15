use std::env;
use std::process::Command;

#[allow(dead_code)]
#[path = "../utils/sub_process.rs"] mod sub_process;

fn shift_args(args: &mut Vec<String>) -> String {
    let command = args[0].clone();

    args.remove(0);
    command
}

fn main() {
    let mut args: Vec<String> = env::args()
        .collect();

    shift_args(&mut args);

    let main_commands = shellwords::split(
      &shift_args(&mut args),
    )
      .expect("Couldn't get the commands");
    let custom_command = shift_args(&mut args);

    match Command::new(&custom_command).output() {
        Ok(_) => sub_process::exec(
            &custom_command,
            args
                .iter()
                .map(AsRef::as_ref)
                .collect(),
        ),
        Err(_) => {
            println!("Run main command");
        }
    }
}
