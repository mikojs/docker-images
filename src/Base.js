import spawn from 'cross-spawn';
import { Command } from 'clipanion';

export default class Base extends Command {
  exec = async (command, ...args) => {
    const { stdin, stdout, stderr } = this.context;

    return spawn.sync(command, args, {
      stdio: [ stdin, stdout, stderr ],
    });
  };
}
