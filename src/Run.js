import fs from 'fs';

import { Command, Option } from 'clipanion';

import Base from './Base';

export default class Run extends Base {
  static paths = [['run']];

  static usage = Command.Usage({
    description: 'Rnu `docker run` in the docker container',
    details: `
      This command is based on \`docker in docker\` concept, but this one adds some helpful features.
      You could learn more \`docker in docker\` information from the website: \`https://hub.docker.com/_/docker\`

      - When the user is in the project folder, the working directory would be the same as the current path in a new container.
      - This command would mount the same volumes as the current container.
    `,
    examples: [[
      'Run a alpine container',
      '$0 run -it alpine /bin/sh',
    ], [
      'Execute a command in a new container',
      '$0 run alpine echo test',
    ]],
  });

  args = Option.Proxy({ useHelp: true });

  execute = () =>
    this.exec(
      'docker',
      'run',
      '-w',
      /^\/project/.test(process.cwd())
        ? process.cwd()
        : '/project',
      '--volumes-from',
      fs.readFileSync('/etc/hostname', 'utf-8')
        .replace(/\n/g, ''),
      ...this.args,
    );
}
