#!/usr/bin/env node

import { Cli, Builtins } from 'clipanion';

import { version } from '../../package.json';

import Run from '../Run';
import Exec from '../Exec';
import Rm from '../Rm';
import Rmi from '../Rmi';

const cli = new Cli({
  binaryLabel: 'dind',
  binaryName: 'dind',
  binaryVersion: version,
});
 
cli.register(Run);
cli.register(Exec);
cli.register(Rm);
cli.register(Rmi);
cli.register(Builtins.HelpCommand);
cli.register(Builtins.VersionCommand);
cli.runExit(process.argv.slice(2));
