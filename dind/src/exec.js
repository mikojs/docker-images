import { Command, Option } from 'clipanion';

import dockerWithWorkdir from './utils/dockerWithWorkdir';

export default class Exec extends Command {
  static paths = [['exec']];

  static usage = Command.Usage({
    category: 'Docker in docker',
    description: 'Run `docker exec` in the docker container',
    details: `
      When the user is in the project folder, the working directory would be the same as the current path in a new container.
    `,
    examples: [[
      'Execuate a container',
      'dind exec -it <id> sh',
    ]],
  });

  args = Option.Proxy();

  execute = () => dockerWithWorkdir(Exec, this);
}
