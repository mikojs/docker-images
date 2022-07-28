#!/usr/bin/env node

import { Cli } from 'clipanion';

import { version } from '../../package.json';

import Run from '../run';
import Rm from '../rm';

const cli = new Cli({
  binaryLabel: 'dind',
  binaryName: 'dind',
  binaryVersion: version,
});
 
cli.register(Run);
cli.register(Rm);
cli.runExit(process.argv.slice(2));
