import fs from 'fs';

import { Command, Option } from 'clipanion';
import spawn from 'cross-spawn';

export default class Rm extends Command {
  static paths = [['rm'], ['rmi']];

  async execute() {
    const { stdin, stdout, stderr } = this.context;

    console.log(this.paths);
    /*
    await spawn.sync('docker', [
      ...args,
      ...this.args,
    ], {
      stdio: [stdin, stdout, stderr],
    });
    */
  }
}
