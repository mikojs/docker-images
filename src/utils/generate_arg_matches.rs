use clap::{Command, ArgMatches};

#[allow(dead_code)]
#[path = "./args.rs"] mod args;

pub fn main(args: Vec<&str>) -> ArgMatches {
    Command::new("generate-arg-matches")
        .arg(args::set_proxy_arg(true))
        .get_matches_from(
            [
                vec!["generate-arg-matches"],
                args,
            ]
                .concat(),
        )
}
