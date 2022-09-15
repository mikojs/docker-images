use std::env;

fn main() {
    let mut args: Vec<String> = env::args()
        .collect();
    let commands_str = args[1].clone();

    args.remove(0);
    args.remove(0);

    let commands = shellwords::split(&commands_str)
      .expect("Couldn't get the commands");

    println!("{:?}", commands);
    println!("{:?}, {}", args, args.len());
}
