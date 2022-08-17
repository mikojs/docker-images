#!/usr/bin/env node

import { Cli, Builtins } from 'clipanion';

import { version } from '../../package.json';

import Code from '../Code';

const cli = new Cli({
  binaryLabel: 'dind',
  binaryName: 'dind',
  binaryVersion: version,
});
 
cli.register(Code);
cli.register(Builtins.HelpCommand);
cli.register(Builtins.VersionCommand);
cli.runExit(process.argv.slice(2));
