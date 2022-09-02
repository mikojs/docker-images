use clap::{Command, Arg, ArgMatches};

pub fn command() -> Command<'static> {
    Command::new("run")
        .about(r#"This command would mount the same volumes to the current container
When the current path is `/project`, a new container would use the same path.
Otherwise, this would change to be `/project`"#)
        .arg(
            Arg::new("args")
                .required(true)
                .multiple_values(true)
        )
}

pub fn execute(sub_matches: &ArgMatches) {
    let args: Vec<&str> = sub_matches
        .values_of("args")
        .unwrap()
        .collect();

    println!("{:?}", args);
    /*
    let status = sub_process::exec(
        "docker",
        [
            vec![
                "run",
                "-w",
                /*
                /^\/project/.test(process.cwd())
                    ? process.cwd()
                    : "/project",
                    */
                "--volumes-from",
                /*
                fs.readFileSync("/etc/hostname", "utf-8")
                    .replace(/\n/g, ""),
                    */
            ],
            args,
        ].concat().as_slice(),
    );

    assert!(status.success());
    */
}
