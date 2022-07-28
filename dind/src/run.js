import fs from 'fs';

import { Command, Option } from 'clipanion';
import spawn from 'cross-spawn';

import getStdio from './utils/getStdio';

const HOSTNAME_FILE_PATH = '/etc/hostname';

export default class Run extends Command {
  static paths = [['run'], Command.Default];

  args = Option.Proxy();

  async execute() {
    const args = [
      'run',
      '-it',
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
