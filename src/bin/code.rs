use std::fs;
use std::env;

use clap::{crate_version, Command};
use glob;
use inquire::Confirm;
use regex::Regex;

use docker_images::utils::proxy_args;
use docker_images::utils::get_current_dir;
use docker_images::utils::sub_process;

const OPTIONS: glob::MatchOptions = glob::MatchOptions {
    case_sensitive: false,
    require_literal_separator: false,
    require_literal_leading_dot: false,
};

fn confirm_to_create_file(file_name: &str) -> String {
    let message = format!("Couldn't find `{}`. Do you want to create this or not:", file_name);
    let result = match Confirm::new(&message).prompt() {
        Ok(true) => true,
        _ => false,
    };

    if !result {
        return "".to_string();
    }

    let file_path = get_current_dir::main()
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

            if !file_path.is_empty() {
                files.push(file_path);
            }
        }
    }

    files
}

fn main() {
    let matches = Command::new("code")
        .version(crate_version!())
        .about("Use this command to open files in a code-server")
        .arg(proxy_args::set_proxy_args(true))
        .get_matches();
    let patterns = proxy_args::get_values_from_proxy_args(&matches);
    let mut files = vec![];

    for pattern in patterns {
        files.append(&mut find_files(&pattern));
    }

    if files.len() == 0 {
        return println!("Couldn't find any files to open.");
    }

    sub_process::exec(
        "code-server",
        files
            .iter()
            .map(AsRef::as_ref)
            .collect(),
    );
}
