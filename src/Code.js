import path from 'path';

import { Command, Option } from 'clipanion';
import glob from 'glob';
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
    const files = this.args
      .map(pattern => {
        const result = glob.sync(pattern, { nodir: true, nonull: true, absolute: true });

        if (result.length === 1 && result[0] === pattern) {
          if (/\*/.test(result[0]))
            return [];

          const filePath = path.resolve(process.cwd(), result[0]);

          // TODO: should ask for creating or not
          return [filePath];
        }

        return result;
      })
      .flat();

    await spawn.sync('code-server', files, getStdio(this.context));
  };
}
