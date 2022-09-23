use std::fs;
use std::path::{Path, PathBuf};

use clap::{crate_version, Command};
use serde_json::Value;
use semver::{VersionReq, Op};

#[allow(dead_code)]
#[path = "../utils/args.rs"] mod args;
#[path = "../utils/get_version.rs"] mod get_version;
#[path = "../utils/get_current_dir.rs"] mod get_current_dir;
#[path = "../utils/generate_arg_matches.rs"] mod generate_arg_matches;
#[allow(dead_code)]
#[path = "../run.rs"] mod run;

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

fn get_node_version(engine_name: &str) -> String {
    let package_json_path = find_package_json(
        get_current_dir::main()
    )
        .display()
        .to_string();

    if package_json_path.is_empty() {
        return "".to_string();
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

            return format!("{}-alpine", version);
        }
    }

    "".to_string()
}

fn main() {
    let matches = Command::new("dnode")
        .version(crate_version!())
        .about("Run node command in a docker container")
        .arg(args::set_proxy_arg(false))
        .get_matches();
    let args = args::get_values_from_args(&matches);
    let mut engine_name = "node";

    if args.len() != 0 {
        engine_name = match args[0] {
            "yarn" => "yarn",
            "npm" => "npm",
            "npx" => "npm",
            _ => "node",
        }
    }

    let version = get_version::main(
        engine_name,
        vec![&get_node_version(engine_name), "lts-alpine"],
    );

    if version != "lts-alpine" {
        println!("custom node version: {}", version);
    }

    run::execute(
        &generate_arg_matches::main(
            [
                vec![
                    "-it",
                    "--rm",
                    &format!("node:{}", version),
                ],
                args,
            ]
                .concat(),
        ),
    );
}
