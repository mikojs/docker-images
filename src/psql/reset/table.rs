use clap::{Command, Arg, ArgMatches};

#[path = "../../utils/docker_run_with_image.rs"] mod docker_run_with_image;

#[path = "../utils/check_db_url.rs"] mod check_db_url;

pub fn command() -> Command<'static> {
    Command::new("table")
        .about("Reset a table in the database")
        .arg(
            Arg::new("table-name")
                .required(true)
        )
}

pub fn execute(matches: &ArgMatches, db_name: &str, db_url: &str) {
    check_db_url::main(db_name, db_url, false);
    docker_run_with_image::main(
        "postgres",
        vec![],
        vec![
            "psql",
            db_url,
            "-c",
            &format!(
                "TRUNCATE TABLE {};",
                matches
                    .value_of("table-name")
                    .unwrap(),
            ),
        ],
    );
}
