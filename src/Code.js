import fs from 'fs';
import path from 'path';

import { Command, Option } from 'clipanion';
import glob from 'glob';
import { Confirm } from 'enquirer';

import Base from './Base';

export default class Code extends Base {
  static paths = [Command.Default];

  static usage = Command.Usage({
    description: 'Use this command to open files in a code-server',
    details: `
      Unlike code-server, this command checks if the files exist. If not, this would ask you to create and open the file.
    `,
    examples: [[
      'Open files',
      'code a.file b.file',
    ], [
      'Use pattern to open files',
      'code "*.file"',
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

    if (files.length !== 0) {
      await this.exec('code-server', ...files);
      return;
    }

    const { stdout } = this.context;

    stdout.write('Couldn\'t find any files to open.\n');
  };
}
