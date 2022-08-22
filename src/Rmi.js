import { Command } from 'clipanion';

import Base from './Base';

export default class Rmi extends Base {
  static paths = [['rmi']];

  static usage = Command.Usage({
    description: 'Remove the none-images',
    details: `
      This command would find the ids of the none-images and remove them.
    `,
    examples: [[
      'Remove the none images',
      'ddocker rmi',
    ]],
  });

  execute = async () => {
    const ids = (await this.execResult('docker', 'images', '-qf', 'dangling=true'))
      .stdout.toString()
      .split('\n')
      .filter(Boolean);

    if (ids.length === 0) {
      const { stdout } = this.context;

      stdout.write('No none-images need to be removed.\n');
      return;
    }

    await spawn.sync('docker', 'rmi', ...ids);
  };
}
