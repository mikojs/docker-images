use clap::{crate_version, Command};

fn cli() -> Command<'static> {
    Command::new("code")
        .version(crate_version!())
        .about("Use this command to open files in a code-server")
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        _ => unreachable!(),
    }
}
