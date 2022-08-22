#!/usr/bin/env node

import Run from '../Run';
import Exec from '../Exec';
import Rm from '../Rm';
import Rmi from '../Rmi';

import runExit from '../utils/runExit';

runExit(__filename, Run, Exec, Rm, Rmi);
