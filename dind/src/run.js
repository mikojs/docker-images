import fs from 'fs';

import { Command, Option } from 'clipanion';

import dockerWithWorkdir from './utils/dockerWithWorkdir';

const HOSTNAME_FILE_PATH = '/etc/hostname';

export default class Run extends Command {
  static paths = [['run'], Command.Default];

  static usage = Command.Usage({
    category: 'Docker in docker',
    description: 'Rnu \`docker run\` in the docker container',
    details: `
      This command is based on \`docker in docker\` concept, but this one adds some helpful features.
      You could learn more \`docker in docker\` information from the website: \`https://hub.docker.com/_/docker\`

      - When the user is in the project folder, the working directory would be the same as the current path in a new container.
      - This command would mount the same volumes as the current container.
    `,
    examples: [[
      'Run a alpine container',
      'dind run -it alpine /bin/sh',
    ], [
      'Execute a command in a new container',
      'dind run alpine echo test',
    ]],
  });

  args = Option.Proxy();

  execute = () => {
    if (fs.existsSync(HOSTNAME_FILE_PATH))
      this.args = [
        '--volumes-from',
        fs.readFileSync(HOSTNAME_FILE_PATH, 'utf-8')
          .replace(/\n/g, ''),
        ...this.args,
      ];

    return dockerWithWorkdir(Run, this);
  };
}
