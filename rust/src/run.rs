use clap::Command;

#[path = "./utils/sub_process.rs"] mod sub_process;

pub fn command() -> Command<'static> {
    return Command::new("run")
        .about("
            This command is based on \`docker in docker\` concept, but this one adds some helpful features.
            You could learn more \`docker in docker\` information from the website: \`https://hub.docker.com/_/docker\`

            - When the user is in the project folder, the working directory would be the same as the current path in a new container.
            - This command would mount the same volumes as the current container.
        ")
}

pub fn execute() {
    println!("run");
}
