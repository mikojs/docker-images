# Docker images

Here are some helpful commands used in the docker container.

## ddocker

```
ddocker 0.1.0
Some docker commands are used in a docker container

USAGE:
    ddocker <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    exec    This command would set the working directory with \`docker exec\`
                When the current path is under \`/project\`, the same path would be the initial
                working directory
                Otherwise, this would change to be \`/project\`
    help    Print this message or the help of the given subcommand(s)
    name    Show the current container id
    rm      Find the all ids of the stopped containers and remove them
    rmi     Find the all ids of the none-images and remove them
    run     This command would mount the same volumes to the current container
                When the current path is under \`/project\`, a new container would use the same path
                as the working directory
                Otherwise, this would change to be \`/project\`

```

## code

```
code 0.1.0
Use this command to open files in a code-server

USAGE:
    code <args>...

ARGS:
    <args>...    

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

```

## node-parser

```
node-parser 0.1.0
Use to parse the node version from the package.json

USAGE:
    node-parser [name]

ARGS:
    <name>    [default: node] [possible values: node, yarn, npm]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

```