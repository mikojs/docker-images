import getStdio from './getStdio';

export default async (
  Command,
  {
    path: [
      type = 'run'
    ],
    cli,
    args,
    context,
  }
) => {
  // FIXME: https://github.com/arcanis/clipanion/issues/88
  if (args.includes('-h') || args.includes('--help')) {
    const { stdout } = context;

    stdout.write(cli.usage(Command, { detailed: true }));
    return;
  }

  await spawn.sync('docker', [
    type,
    '-w',
    /^\/project/.test(process.cwd())
      ? process.cwd()
      : '/project',
    ...args,
  ], getStdio(context));
}
