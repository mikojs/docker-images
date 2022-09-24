use std::env;

use clap::{crate_version, Command, Arg};

#[path = "../utils/proxy_args.rs"] mod proxy_args;
#[path = "../utils/docker_run_with_image.rs"] mod docker_run_with_image;

fn main() {
    let matches = Command::new("dcmd")
        .version(crate_version!())
        .about("Run docker with the DOCKER_<IMAGE NAME>_VERSION environment variable")
        .arg(
            Arg::new("image-name")
                .required(true)
        )
        .arg(
            Arg::new("versions")
                .help("It could be an image version or DOCKER_<IMAGE NAME>_VERSION environment variable")
                .long("--versions")
                .multiple_values(true)
                .takes_value(true)
        )
        .arg(proxy_args::set_proxy_args(false))
        .get_matches();
    let matched_versions = matches.values_of("versions");
    let mut versions = vec![];

    if !matched_versions.is_none() {
        versions = matched_versions
            .unwrap()
            .collect();
    }

    docker_run_with_image::main(
        matches.value_of("image-name")
            .unwrap(),
        versions,
        proxy_args::get_values_from_proxy_args(&matches)
    );
}
