use clap::{Command, ArgMatches};

use crate::utils::{proxy_args, docker};

pub fn command() -> Command<'static> {
    Command::new("run")
        .about(r#"This command would mount the same volumes to the current container
When the current path is under `/root/work`, a new container would use the same path as the working directory
Otherwise, this would change to be `/root/work`"#)
        .arg(proxy_args::set_proxy_args(true))
}

pub fn execute(matches: &ArgMatches) {
    docker::run(proxy_args::get_values_from_proxy_args(matches));
}
