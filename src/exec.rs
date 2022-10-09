use clap::{Command, ArgMatches};

use crate::utils::{Error, args, sub_process, docker};

pub fn command() -> Command<'static> {
    Command::new("exec")
        .about(r#"This command would set the working directory with `docker exec`
When the current path is under `/root/work`, the same path would be the initial working directory
Otherwise, this would change to be `/root/work`"#)
        .arg(args::set_proxy(true))
}

pub fn execute(matches: &ArgMatches) -> Result<(), Error> {
    sub_process::exec(
        "docker",
        [
            vec![
                "exec",
                "-w",
                &docker::working_dir()?,
            ],
            args::get_values_from_proxy(matches),
        ]
            .concat(),
    )?;
    Ok(())
}
