use clap::{Command, ArgMatches};

#[allow(dead_code)]
#[path = "./utils/args.rs"] mod args;

pub fn command(db_name: &str) -> Command<'static> {
    Command::new(db_name)
        // TODO: .about(&format!("Connect to `{}`", db_name))
        .arg(args::set_proxy_arg(false))
}

pub fn execute(sub_matches: &ArgMatches) {
    println!(
        "{:?}",
        args::get_values_from_args(sub_matches),
    );
}
