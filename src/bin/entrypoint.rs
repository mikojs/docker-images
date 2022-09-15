use std::env;

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
    let custom_commands = shift_args(&mut args);
    
    println!("{:?}, {:?}, {:?}", main_commands, custom_commands, args);
}
