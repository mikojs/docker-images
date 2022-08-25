import { Command, Option } from 'clipanion';

import Base from './Base';

export default class Exec extends Base {
  static paths = [['exec']];

  static usage = Command.Usage({
    description: 'Run `docker exec` in the docker container',
    details: `
      When the user is in the project folder, the working directory would be the same as the current path in a new container.
    `,
    examples: [[
      'Execuate a container',
      '$0 exec -it <id> sh',
    ]],
  });

  args = Option.Proxy({ useHelp: true });

  execute = () =>
    this.exec(
      'docker',
      'exec',
      '-w',
      /^\/project/.test(process.cwd())
        ? process.cwd()
        : '/project',
      ...this.args,
    );
}
