// FIXME: https://github.com/arcanis/clipanion/issues/88
export default (Command, { cli, args, context: { stdout } }) => {
  if (!args.includes('-h') && !args.includes('--help'))
    return false;

  stdout.write(cli.usage(Command, { detailed: true }));

  return true;
}
