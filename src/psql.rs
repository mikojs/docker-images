use std::process;

use clap::{Command, ArgMatches};

#[allow(dead_code)]
#[path = "./utils/args.rs"] mod args;

#[allow(dead_code)]
#[path = "./run.rs"] mod run;
#[path = "./psql_show.rs"] mod psql_show;
#[path = "./psql_clone.rs"] mod psql_clone;

pub fn command(db_name: &str) -> Command<'static> {
    Command::new(db_name)
        .about("Database")
        .subcommand(psql_show::command())
        .subcommand(psql_clone::command())
        .arg(args::set_proxy_arg(false))
}

pub fn execute(sub_matches: &ArgMatches, db_name: &str) {
    let db_url = psql_show::execute(db_name);

    match sub_matches.subcommand() {
        Some(("show", _)) => println!("{}", db_url),
        Some(("clone", sub_sub_matches)) => psql_clone::execute(sub_sub_matches, db_name),
        _ => {
            if db_url.is_empty() {
                eprint!(
                    "`{}` isn't in the environment variables.",
                    db_name.to_uppercase(),
                );
                process::exit(1);
            }

            run::execute(
                sub_matches,
                vec!["-it", "--rm", "postgres:alpine", "psql", &db_url],
            );
        },
    }
}
