use clap::{ArgMatches, Command};

use crate::utils::{args, docker, Error};

pub fn command() -> Command<'static> {
    Command::new("run")
        .about(r#"This command would mount the same volumes to the current container
When the current path is under `/root/work`, a new container would use the same path as the working directory
Otherwise, this would change to be `/root/work`"#)
        .arg(args::set_proxy(true))
}

pub fn execute(matches: &ArgMatches) -> Result<(), Error> {
    docker::run(args::get_values_from_proxy(matches))?;
    Ok(())
}
