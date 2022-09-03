use clap::{crate_version, Command};

fn cli() -> Command<'static> {
    Command::new("code")
        .version(crate_version!())
        .about("Use this command to open files in a code-server")
        .arg_required_else_help(true)
}

fn main() {
    match cli().get_matches().subcommand() {
        _ => unreachable!(),
    }
}
