use crate::utils::docker;

pub fn run(args: Vec<&str>) {
    docker::run(
        [
            vec![
                "-it",
                "--rm",
                "postgres:<DOCKER_POSTGRES_VERSION|alpine>",
            ],
            args,
        ]
            .concat(),
    );
}
