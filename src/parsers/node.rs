use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use clap::{crate_version, Command, Arg};
use serde_json::Value;
use semver::{VersionReq, Op};

fn cli() -> Command<'static> {
    Command::new("node-parser")
        .version(crate_version!())
        .about("Use to parse the node version from the package.json")
        .arg(
            Arg::new("name")
                .default_value("node")
                .value_parser(["node", "yarn", "npm"])
        )
}

fn find_package_json(cwd: PathBuf) -> PathBuf {
    let file_path = cwd.join("package.json");

    if !file_path.exists() {
        if cwd == Path::new("/") {
            return Path::new("")
                .to_path_buf();
        }

        return find_package_json(
            cwd.parent()
                .expect("couldn't find the parent directory")
                .to_path_buf(),
        );
    }

    file_path
}

fn main() {
    let engine_name = cli()
        .get_matches()
        .value_of("name")
        .expect("Couldn't get name from the arguments")
        .to_string();
    let cwd = env::current_dir()
        .expect("Couldn't get the currenct directory");
    let package_json_path = find_package_json(cwd)
        .display()
        .to_string();

    if package_json_path.is_empty() {
        return;
    }

    let content = fs::read_to_string(package_json_path)
        .expect("Couldn't read the file");

    if let Ok(value) = serde_json::from_str::<Value>(&content) {
        if let Some(engine_version) = value["engines"].get(engine_name) {
            let comparators = VersionReq::parse(
                &engine_version
                    .to_string()
                    .replace("\"", "")
            )
                .unwrap()
                .comparators;
            let mut version = 0;

            for comparator in comparators {
                if version < comparator.major {
                    version =  match comparator.op {
                        Op::Less => comparator.major - 1,
                        Op::Greater => comparator.major + 1,
                        _ => comparator.major,
                    }
                }
            }

            println!("{}-alpine", version);
        }
    }
    else {
        unreachable!();
    }
}
