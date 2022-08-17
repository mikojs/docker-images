import { Command, Option } from 'clipanion';
import spawn from 'cross-spawn';

import getStdio from './utils/getStdio';

export default class Code extends Command {
  static paths = [Command.Default];

  static usage = Command.Usage({
    category: '',
    description: '',
    details: `
    `,
    examples: [[
    ]],
  });

  args = Option.Proxy();

  execute = async () => {
    // FIXME: check if files exist in args
    await spawn.sync('code-server', this.args, getStdio(this.context));
  };
}
