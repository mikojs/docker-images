use clap::Command;

#[path = "./utils/args.rs"] mod args;

pub fn command() -> Command<'static> {
    Command::new("name")
        .about("Show the current container id")
}

pub fn execute() {
    println!("{}", args::get_container_name());
}
