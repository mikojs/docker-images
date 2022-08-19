import { Command } from 'clipanion';

import remove from './utils/remove';

export default class Rm extends Command {
  static paths = [['rm']];

  static usage = Command.Usage({
    category: 'Docker',
    description: 'Remove the not used containers',
    details: `
      This command would find the ids of the stopped container and remove them.
    `,
    examples: [[
      'Remove the stopped container',
      'ddocker rm',
    ]],
  });

  execute = () => remove(this.path[0], this.context);
}
