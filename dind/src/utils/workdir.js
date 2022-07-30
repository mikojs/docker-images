export default /^\/project/.test(process.cwd())
  ? process.cwd()
  : '/project';
