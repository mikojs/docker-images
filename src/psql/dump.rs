use clap::{Command, Arg, ArgMatches};

#[path = "../utils/proxy_args.rs"] mod proxy_args;

#[path = "./utils/docker_run.rs"] mod docker_run;

pub fn command() -> Command<'static> {
    Command::new("dump")
        .about("Dump the database to a file")
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
                "pg_dump",
                "-Fc",
                "-f",
                matches
                    .value_of("file-name")
                    .unwrap(),
                db_url,
            ],
            proxy_args::get_values_from_proxy_args(matches),
        ]
            .concat(),
    );
}
