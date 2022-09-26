use clap::{Command, Arg, ArgMatches};

#[path = "../utils/proxy_args.rs"] mod proxy_args;

#[path = "./utils/docker_run.rs"] mod docker_run;

pub fn command() -> Command<'static> {
    Command::new("restore")
        .about("Restore the database from a file")
        .arg(
            Arg::new("file-name")
                .required(true)
        )
        .arg(proxy_args::set_proxy_args(false))
}

pub fn execute(matches: &ArgMatches, db_url: &str) {
    docker_run::main(
        [
            vec![
                "pg_restore",
                "--no-owner",
                "-x",
                "-d",
                db_url,
                matches
                    .value_of("file-name")
                    .unwrap(),
            ],
            proxy_args::get_values_from_proxy_args(matches),
        ]
            .concat(),
    );
}
