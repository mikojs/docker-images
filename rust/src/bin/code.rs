use clap::{crate_version, Command};

#[allow(dead_code)]
#[path = "../utils/args.rs"] mod args;

fn cli() -> Command<'static> {
    Command::new("code")
        .version(crate_version!())
        .about("Use this command to open files in a code-server")
        .arg(args::set_proxy_arg())
}

fn main() {
    let files: Vec<String> = cli()
        .get_matches()
        .remove_many("args")
        .expect("`args` is required")
        .collect();

    println!("{:?}", files);
}
