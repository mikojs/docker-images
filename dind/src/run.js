import fs from 'fs';

import { Command, Option } from 'clipanion';
import spawn from 'cross-spawn';

import getStdio from './utils/getStdio';

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

  async execute() {
    // FIXME: https://github.com/arcanis/clipanion/issues/88
    if (this.args.includes('-h') || this.args.includes('--help')) {
      const { stdout } = this.context;

      stdout.write(this.cli.usage(Run, { detailed: true }));
      return;
    }

    const args = [
      'run',
      '-w',
      /^\/project/.test(process.cwd())
        ? process.cwd()
        : '/project',
    ];

    if (fs.existsSync(HOSTNAME_FILE_PATH))
      args.push(
        '--volumes-from',
        fs.readFileSync(HOSTNAME_FILE_PATH, 'utf-8')
          .replace(/\n/g, ''),
      );

    await spawn.sync('docker', [
      ...args,
      ...this.args,
    ], getStdio(this.context));
  }
}
