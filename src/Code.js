import fs from 'fs';
import path from 'path';

import { Command, Option } from 'clipanion';
import glob from 'glob';
import { Confirm } from 'enquirer';
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

  conformToCreateFile = async relativePath => {
    const answer = await new Confirm({
      name: 'question',
      message: `Couldn't find \`${relativePath}\`. Do you want to create this?`,
    }).run();

    if (!answer) return [];

    const filePath = path.resolve(process.cwd(), relativePath);

    fs.writeFileSync(filePath, '\n', 'utf-8');

    return [filePath];
  };

  execute = async () => {
    const files = (await Promise.all(
      this.args.map(async pattern => {
        const result = glob.sync(pattern, { nodir: true, nonull: true, absolute: true });

        if (result.length === 1 && result[0] === pattern)
          return /\*/.test(pattern)
            ? []
            : await this.conformToCreateFile(pattern);

        return result;
      })
    )).flat();

    await spawn.sync('code-server', files, getStdio(this.context));
  };
}
