const fs = require('fs');
const path = require('path');

const binFolder = path.resolve(__dirname, './src/bin');
const entry = fs.readdirSync(binFolder)
  .reduce(
    (result, filePath) => {
      const name = filePath.replace(/\.js$/, '');

      return {
        ...result,
        [name]: path.resolve(binFolder, filePath),
      };
    },
    {},
  );

module.exports = {
  entry,
  output: {
    path: path.resolve(__dirname, './docker-images/bin'),
    filename: '[name].js',
  },
};
