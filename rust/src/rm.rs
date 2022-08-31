use clap::Command;

pub const NAME: &str = "rm";

pub fn command() -> Command<'static> {
    Command::new("rm")
        .about("Find the all ids of the stopped containers and remove them")
}

pub fn execute() {
    println!("rm");
}
