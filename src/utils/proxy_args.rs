use clap::{Arg, ArgMatches};

pub fn set_proxy_args(required: bool) -> Arg<'static> {
    Arg::new("args")
        .help("Those arguments would be proxied to the sub command")
        .required(required)
        .multiple_values(true)
        .allow_hyphen_values(true)
}

pub fn get_values_from_proxy_args(matches: &ArgMatches) -> Vec<&str> {
    match matches.values_of("args") {
        Some(args) => args.collect(),
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use clap::Command;
    use super::*;

    #[test]
    fn get_proxy_args() {
        let testings: Vec<Vec<&str>> = vec![
            vec!["foo", "bar"],
            vec![],
        ];

        for testing in testings {
            let matches = Command::new("test")
                .arg(set_proxy_args(false))
                .get_matches_from([vec!["test"], testing.to_vec()].concat());

            assert_eq!(get_values_from_proxy_args(&matches), testing);
        }
    }
}
