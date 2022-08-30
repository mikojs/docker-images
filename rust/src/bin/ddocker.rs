use clap::{crate_version, Command};

fn cli() -> Command<'static> {
    Command::new("ddocker")
        .version(crate_version!())
        .about("Some commands are used in the docker-in-docker")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        _ => unreachable!(),
    }
}
