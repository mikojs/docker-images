use clap::Command;

#[path = "./utils/get_container_name.rs"] mod get_container_name;

pub fn command() -> Command<'static> {
    Command::new("name")
        .about("Show the current container id")
}

pub fn execute() {
    println!("{}", get_container_name::main());
}
