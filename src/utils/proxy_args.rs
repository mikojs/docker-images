use clap::{Arg, ArgMatches};

pub fn set_proxy_args(required: bool) -> Arg<'static> {
    Arg::new("args")
        .help("Those arguments would be proxied to the sub command")
        .required(required)
        .multiple_values(true)
        .allow_hyphen_values(true)
}

pub fn get_values_from_proxy_args(matches: &ArgMatches) -> Vec<&str> {
    let args = matches
        .values_of("args");

    if args.is_none() {
        return vec![];
    }

    args
        .unwrap()
        .collect()
}
