use clap::{crate_version, Command};

fn main() {
    let matches = Command::new("dpsql")
        .version(crate_version!())
        .about("Use psql command in the docker container")
        .get_matches();

    println!("{:?}", matches);
}
