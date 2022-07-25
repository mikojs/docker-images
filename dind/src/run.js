import { Command } from 'clipanion';
import spawn from 'cross-spawn';

export default class Run extends Command {
  static paths = [['run'], Command.Default];

  async execute() {
    const { stdin, stdout, stderr } = this.context;

    await spawn.sync('docker', {
      stdio: [stdin, stdout, stderr],
    });
  }
}
