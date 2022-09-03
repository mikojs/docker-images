use std::fs;

use clap::{crate_version, Command};
use glob;
use inquire::Confirm;

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

fn confirm_to_create_file(file_name: &str) {
    let message = format!("Couldn't find `{}`. Do you want to create this?", file_name);
    let result = Confirm::new(&message)
        .prompt();

    if result.ok() != Some(true) {
        return;
    }

    println!("create a file");
}

fn find_files(pattern: &str) -> Vec<String> {
    let mut files: Vec<String> = []
        .to_vec();

    for entry in glob::glob_with(pattern, OPTIONS).unwrap() {
        if let Ok(path) = entry {
            files.push(
                fs::canonicalize(path)
                    .expect("Canonicalize path fail")
                    .display()
                    .to_string()
            );
        }
    }

    if files.len() == 0 {
        confirm_to_create_file(pattern);
    }

    files
}

fn main() {
    let mut files: Vec<String> = []
        .to_vec();
    let patterns: Vec<String> = cli()
        .get_matches()
        .remove_many("args")
        .expect("`args` is required")
        .collect();

    for pattern in patterns {
        files.append(&mut find_files(&pattern));
    }

    if files.len() == 0 {
        return println!("Couldn't find any files to open.");
    }

    println!("{:?}", files);
}
