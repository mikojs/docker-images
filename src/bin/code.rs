use std::env;
use std::fs;
use std::path::Path;

use clap::{crate_version, Command};
use glob;
use regex::Regex;

use docker_images::utils::{args, prompt, sub_process, Error};

const OPTIONS: glob::MatchOptions = glob::MatchOptions {
    case_sensitive: false,
    require_literal_separator: false,
    require_literal_leading_dot: false,
};

fn confirm_to_create_file(file_name: &str) -> Result<String, Error> {
    if !prompt(&format!(
        "Couldn't find `{}`. Do you want to create this or not:",
        file_name
    )) {
        return Ok("".to_string());
    }

    let file_path = env::current_dir()?.join(file_name);
    let file_dir = match file_path.parent() {
        Some(value) => value,
        _ => Path::new("/"),
    };

    if !file_dir.exists() {
        fs::create_dir_all(file_dir)?;
    }

    fs::File::create(&file_path)?;
    Ok(file_path.display().to_string())
}

fn find_files(pattern: &str) -> Result<Vec<String>, Error> {
    let mut files = vec![];

    for entry in glob::glob_with(pattern, OPTIONS)? {
        if let Ok(path) = entry {
            files.push(fs::canonicalize(path)?.display().to_string());
        }
    }

    if files.len() == 0 {
        let skip_confirm = Regex::new(r"\*")?.is_match(pattern);

        if !skip_confirm {
            let file_path = confirm_to_create_file(pattern)?;

            if !file_path.is_empty() {
                files.push(file_path);
            }
        }
    }

    Ok(files)
}

fn main() -> Result<(), Error> {
    let matches = Command::new("code")
        .version(crate_version!())
        .about("Use this command to open files in a code-server")
        .arg(
            args::set_proxy(true)
                .value_name("files")
                .help("Open the file with filenames or regular expression"),
        )
        .get_matches();
    let patterns = args::get_values_from_proxy(&matches);
    let mut files = vec![];

    for pattern in patterns {
        files.append(&mut find_files(&pattern)?);
    }

    if files.len() == 0 {
        println!("Couldn't find any files to open.");
        return Ok(());
    }

    sub_process::exec("code-server", files.iter().map(AsRef::as_ref).collect())?;
    Ok(())
}
