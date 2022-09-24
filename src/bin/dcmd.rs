use std::env;

use clap::{crate_version, Command, Arg};

#[path = "../utils/proxy_args.rs"] mod proxy_args;
#[path = "../utils/docker_run_with_image.rs"] mod docker_run_with_image;

fn main() {
    let matches = Command::new("dcmd")
        .version(crate_version!())
        .about("Run docker with the DOCKER_NAME_VERSION environment variable")
        .arg(
            Arg::new("image-name")
                .required(true)
        )
        .arg(proxy_args::set_proxy_args(false))
        .get_matches();
    let docker_image = matches.value_of("image-name")
        .unwrap();

    docker_run_with_image::main(
        docker_image,
        vec![&format!("DOCKER_{}_VERSION", docker_image.to_uppercase())],
        proxy_args::get_values_from_proxy_args(&matches)
    );
}
