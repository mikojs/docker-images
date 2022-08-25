const fs = require('fs');
const path = require('path');

const isProduction = process.env.NODE_ENV === 'production';
const binFolder = path.resolve(__dirname, './src/bin');
const entry = fs.readdirSync(binFolder)
  .reduce(
    (result, filePath) => {
      const name = filePath.replace(/\.js$/, '');

      if (name === 'pkg') return result;

      return {
        ...result,
        [name]: path.resolve(binFolder, filePath),
      };
    },
    {},
  );

module.exports = {
  mode: isProduction ? 'production' : 'development',
  target: 'node',
  entry,
  output: {
    path: path.resolve(__dirname, './docker-images/bin'),
    filename: '[name]',
  },
  module: {
    rules: [
      {
        test: /\.js$/,
        use: {
          loader: 'babel-loader',
          options: {
            presets: ['@babel/env'],
          },
        },
      },
    ],
  },
};
