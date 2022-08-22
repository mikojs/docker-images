#!/usr/bin/env node

import Run from '../Run';
import Exec from '../Exec';
import Rm from '../Rm';
import Rmi from '../Rmi';

import runCli from '../utils/runCli';

runCli(__filename, Run, Exec, Rm, Rmi);
