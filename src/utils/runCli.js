import { Cli, Builtins } from 'clipanion';

import { version } from '../../package.json';

export default (binaryLabel, ...commands) => {
  const cli = new Cli({
    binaryLabel,
    binaryName: binaryLabel,
    binaryVersion: version,
  });
  
  commands.forEach(command => {
    cli.register(command);
  });

  cli.register(Builtins.HelpCommand);
  cli.register(Builtins.VersionCommand);
  cli.runExit(process.argv.slice(2));
}
