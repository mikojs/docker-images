import spawn from 'cross-spawn';

import getStdio from './getStdio';

export default async (type, context) => {
  const { stdout } = context;
  const result = await spawn.sync(
    'docker',
    type === 'rm'
      ? ['ps', '-aqf', 'status=exited']
      : ['images', '-qf', 'dangling=true'],
  );
  const ids = result.stdout
    .toString()
    .split('\n')
    .filter(Boolean);

  if (ids.length !== 0)
    await spawn.sync('docker', [
      type,
      ...ids,
    ], getStdio(context));
  else
    stdout.write(
      `Here doesn't have any must-remove ${
        type === 'rm' ? 'containers' : 'images'
      }.\n`,
    );
};
