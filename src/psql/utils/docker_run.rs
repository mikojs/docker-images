use crate::utils::docker_run;

pub fn main(args: Vec<&str>) {
    docker_run::main(
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
