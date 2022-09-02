use clap::{crate_version, Command};

fn cli() -> Command<'static> {
    return Command::new("code")
        .version(crate_version!())
        .about("Use this command to open files in a code-server")
        .arg_required_else_help(true)
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        _ => unreachable!(),
    }
}
