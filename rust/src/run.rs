use std::fs;
use std::path::Path;
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

fn get_volumes_from_args(file_path: &str) -> Vec<String> {
    let mut args: Vec<String> = [].to_vec();

    if Path::new(file_path).exists() {
        let content = fs::read_to_string(file_path)
            .expect("Couldn't read the fale")
            .replace("\n", "");

        args.push("--volumes-from".to_string());
        args.push(content);
    }

    args
}

pub fn execute(sub_matches: &ArgMatches) {
    let args: Vec<&str> = sub_matches
        .values_of("args")
        .unwrap()
        .collect();

    println!("{:?}", [
        vec![
            "run",
            "-w",
        ],
        get_volumes_from_args("/etc/hostname")
            .iter()
            .map(AsRef::as_ref)
            .collect(),
        args,
    ].concat().as_slice());
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
