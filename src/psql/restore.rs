use clap::{Command, Arg, ArgMatches};

#[path = "../utils/proxy_args.rs"] mod proxy_args;
#[path = "../utils/docker_run_with_image.rs"] mod docker_run_with_image;

#[path = "./utils/check_db_url.rs"] mod check_db_url;

pub fn command() -> Command<'static> {
    Command::new("restore")
        .about("Restore the database from a file")
        .arg(
            Arg::new("file-name")
                .required(true)
        )
        .arg(proxy_args::set_proxy_args(false))
}

pub fn execute(matches: &ArgMatches, db_name: &str, db_url: &str) {
    check_db_url::main(db_name, db_url, false);
    docker_run_with_image::main(
        "postgres",
        vec!["DOCKER_POSTGRES_VERSION"],
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
