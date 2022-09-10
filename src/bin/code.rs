use std::fs;
use std::env;

use clap::{crate_version, Command};
use glob;
use inquire::Confirm;
use regex::Regex;

#[path = "../utils/main.rs"] mod utils;
#[allow(dead_code)]
#[path = "../utils/sub_process.rs"] mod sub_process;
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

fn confirm_to_create_file(file_name: &str) -> String {
    let message = format!("Couldn't find `{}`. Do you want to create this?", file_name);
    let result = Confirm::new(&message)
        .prompt();

    if let Ok(false) = result {
        return "".to_string();
    }

    let file_path = utils::get_current_dir()
        .join(file_name);
    let file_dir = file_path
        .parent()
        .expect("Couldn't get the file directory");

    if !file_dir.exists() {
        fs::create_dir_all(file_dir)
          .expect("Couldn't create the file directory");
    }

    fs::File::create(&file_path)
        .expect("Error encountered while creating file");
    file_path
        .display()
        .to_string()
}

fn find_files(pattern: &str) -> Vec<String> {
    let mut files = vec![];

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
        let skip_confirm = Regex::new(r"\*")
            .unwrap()
            .is_match(pattern);

        if !skip_confirm {
            let file_path = confirm_to_create_file(pattern);

            if file_path != "" {
                files.push(file_path);
            }
        }
    }

    files
}

fn main() {
    let mut files = vec![];
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

    sub_process::exec(
        "code-server",
        &files
            .iter()
            .map(AsRef::as_ref)
            .collect::<Vec<&str>>(),
    );
}
