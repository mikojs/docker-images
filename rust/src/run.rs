use clap::Command;

pub fn command() -> Command<'static> {
    Command::new("run")
        .about("")
}

pub fn execute() {
    println!("run");
}
