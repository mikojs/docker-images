use clap::{Arg, ArgMatches};

pub fn set_proxy_arg(required: bool) -> Arg<'static> {
    Arg::new("args")
        .required(required)
        .multiple_values(true)
        .allow_hyphen_values(true)
}

pub fn get_values_from_args(matches: &ArgMatches) -> Vec<&str> {
    let args = matches
        .values_of("args");

    if args.is_none() {
        return vec![];
    }

    args
        .unwrap()
        .collect()
}
