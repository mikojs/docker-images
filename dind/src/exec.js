import { Command, Option } from 'clipanion';

import dockerWithWorkdir from './utils/dockerWithWorkdir';

export default class Exec extends Command {
  static paths = [['exec']];

  static usage = Command.Usage({
    category: 'Docker in docker',
    description: '',
    details: `
    `,
    examples: [[
    ]],
  });

  args = Option.Proxy();

  execute = () => dockerWithWorkdir(Exec, this);
}
