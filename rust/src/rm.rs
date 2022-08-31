use clap::Command;

pub fn command() -> Command<'static> {
    Command::new("rm")
        .about("Find the all ids of the stopped containers and remove them")
}
