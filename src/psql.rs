use std::process;

use clap::{App, Command, ArgMatches};

#[allow(dead_code)]
#[path = "./utils/args.rs"] mod args;

#[allow(dead_code)]
#[path = "./run.rs"] mod run;
#[path = "./psql_show.rs"] mod psql_show;
#[path = "./psql_clone.rs"] mod psql_clone;

pub fn command(app: App<'static>) -> Command<'static> {
    app
        .subcommand(psql_show::command())
        .subcommand(psql_clone::command())
        .arg(args::set_proxy_arg(false))
}

pub fn execute(sub_matches: &ArgMatches, db_name: &str) {
    let db_url = psql_show::execute(db_name);

    match sub_matches.subcommand() {
        Some(("show", _)) => println!("{}", db_url),
        Some(("clone", _)) => psql_clone::execute(db_url),
        _ => {
            if db_url.is_empty() {
                eprint!(
                    "`{}` isn't in the environment variables.",
                    db_name.to_uppercase(),
                );
                process::exit(1);
            }

            run::execute(
                &args::generate_arg_matches(
                    [
                        vec![
                            "-it",
                            "--rm",
                            "postgres:alpine",
                            "psql",
                            &db_url,
                        ],
                        args::get_values_from_args(sub_matches),
                    ]
                        .concat(),
                ),
            );
        },
    }
}
