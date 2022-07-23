#!/usr/bin/env node

import { Cli } from 'clipanion';

import { version } from '../../package.json';
 
const cli = new Cli({
  binaryLabel: 'dind',
  binaryName: 'node dind',
  binaryVersion: version,
});
 
cli.runExit(process.argv.slice(2));
