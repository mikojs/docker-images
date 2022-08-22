import { Command } from 'clipanion';

import Base from './Base';

export default class Rm extends Base {
  static paths = [['rm']];

  static usage = Command.Usage({
    description: 'Remove the not used containers',
    details: `
      This command would find the ids of the stopped container and remove them.
    `,
    examples: [[
      'Remove the stopped container',
      'ddocker rm',
    ]],
  });

  execute = async () => {
    const ids = (await this.execResult('docker', 'ps', '-aqf', 'status=exited'))
      .stdout.toString()
      .split('\n')
      .filter(Boolean);

    if (ids.length === 0) {
      const { stdout } = this.context;

      stdout.write('No containers need to be removed.\n');
      return;
    }

    await spawn.sync('docker', 'rm', ...ids);
  };
}
