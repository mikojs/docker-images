import spawn from 'cross-spawn';
import { Command } from 'clipanion';

export default class Base extends Command {
  execResult = async (command, ...args) =>
    spawn.sync(command, args);

  exec = async (command, ...args) => {
    const { stdin, stdout, stderr } = this.context;

    return spawn.sync(command, args, {
      stdio: [ stdin, stdout, stderr ],
    });
  };
}
