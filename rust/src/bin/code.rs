use clap::{crate_version, Command};
use glob;

#[allow(dead_code)]
#[path = "../utils/args.rs"] mod args;

const OPTIONS: glob::MatchOptions = glob::MatchOptions {
    case_sensitive: false,
    require_literal_separator: false,
    require_literal_leading_dot: false,
};

fn cli() -> Command<'static> {
    Command::new("code")
        .version(crate_version!())
        .about("Use this command to open files in a code-server")
        .arg(args::set_proxy_arg())
}

fn find_files(pattern: &str) {
    for entry in glob::glob_with(pattern, OPTIONS).unwrap() {
        if let Ok(path) = entry {
            println!("{:?}", path.display())
        }
    }
}

fn main() {
    let patterns: Vec<String> = cli()
        .get_matches()
        .remove_many("args")
        .expect("`args` is required")
        .collect();

    for pattern in patterns {
        find_files(&pattern);
    }
}
