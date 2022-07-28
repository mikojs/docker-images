import fs from 'fs';

import { Command, Option } from 'clipanion';
import spawn from 'cross-spawn';

export default class Rm extends Command {
  static paths = [['rm'], ['rmi']];

  async execute() {
    const { stdin, stdout, stderr } = this.context;
    const [type] = this.path;
    const result = await spawn.sync(
      'docker',
      type === 'rm'
        ? ['ps', '-aqf', 'status=exited']
        : ['images', '-qf', 'dangling=true'],
    );
    const ids = result.stdout
      .toString()
      .split('\n')
      .filter(Boolean);

    if (ids.length !== 0)
      await spawn.sync('docker', [
        type,
        ...ids,
      ], {
        stdio: [stdin, stdout, stderr],
      });
    else
      stdout.write(
        `Here doesn't have any must-remove ${
          type === 'rm' ? 'containers' : 'images'
        }.\n`,
      );
  }
}
