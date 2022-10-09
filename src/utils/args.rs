use std::any::Any;

use clap::{Arg, ArgMatches};

pub fn set_proxy(required: bool) -> Arg<'static> {
    Arg::new("proxy")
        .help("Those arguments would be proxied to the sub command")
        .required(required)
        .multiple_values(true)
        .allow_hyphen_values(true)
}

pub fn get_values_from_proxy(matches: &ArgMatches) -> Vec<&str> {
    match matches.values_of("proxy") {
        Some(args) => args.collect(),
        _ => vec![],
    }
}

pub fn value_of<'a>(matches: &'a ArgMatches, name: &'a str) -> &'a str {
    match matches.value_of(name) {
        Some(arg) => arg,
        _ => "",
    }
}

pub fn get_one<'a, T: Any + Clone + Send + Sync + 'static>(
    matches: &'a ArgMatches,
    name: &'a str,
    default: &'a T,
) -> &'a T {
    match matches.get_one::<T>(name) {
        Some(arg) => arg,
        _ => default,
    }
}

#[cfg(test)]
mod tests {
    use clap::{ArgAction, Command};

    use super::*;

    #[test]
    fn get_proxy_args() {
        let testings: Vec<Vec<&str>> = vec![vec!["foo", "bar"], vec![]];

        for testing in testings {
            let matches = Command::new("test")
                .arg(set_proxy(false))
                .get_matches_from([vec!["test"], testing.to_vec()].concat());

            assert_eq!(get_values_from_proxy(&matches), testing);
        }
    }

    #[test]
    fn value_of_arg() {
        let matches = Command::new("test")
            .arg(Arg::new("arg"))
            .get_matches_from(["test", "foo"]);

        assert_eq!(value_of(&matches, "arg"), "foo");
    }

    #[test]
    fn get_one_arg() {
        let matches = Command::new("test")
            .arg(Arg::new("arg").long("--arg").action(ArgAction::Set))
            .get_matches_from(["test", "--arg", "foo"]);

        assert_eq!(
            get_one::<String>(&matches, "arg", &"default".to_string()),
            "foo"
        );
    }

    #[test]
    fn get_one_default_arg() {
        let matches = Command::new("test")
            .arg(Arg::new("arg").long("--arg").action(ArgAction::Set))
            .get_matches_from(["test"]);

        assert_eq!(
            get_one::<String>(&matches, "arg", &"default".to_string()),
            "default"
        );
    }
}
