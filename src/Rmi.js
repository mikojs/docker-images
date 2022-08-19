import { Command } from 'clipanion';

import remove from './utils/remove';

export default class Rmi extends Command {
  static paths = [['rmi']];

  static usage = Command.Usage({
    category: 'Docker',
    description: 'Remove the none images',
    details: `
      This command would find the ids of the none images and remove them.
    `,
    examples: [[
      'Remove the none images',
      'ddocker rmi',
    ]],
  });

  execute = () => remove(this.path[0], this.context);
}
