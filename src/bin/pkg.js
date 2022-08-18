#!/usr/bin/env node
// FIXME: https://github.com/vercel/pkg/issues/1689

import fs from 'fs';
import path from 'path';

import { exec } from 'pkg';

fs.readdirSync(__dirname)
  .reduce(
    async (result, filename) => {
      await result;

      const filePath = path.resolve(__dirname, filename);

      if (filePath === __filename)
        return;

      await exec([
        filePath,
        '--target',
        'alpine',
        '--out-path',
        './docker-images/bin',
        '--compress',
        'GZip',
      ]);
    },
    Promise.resolve(),
  )
