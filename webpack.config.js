const fs = require('fs');
const path = require('path');

const webpack = require('webpack');

const isProduction = process.env.NODE_ENV === 'production';
const binFolder = path.resolve(__dirname, './src/bin');
const outputFolder = path.resolve(__dirname, './docker-images/bin');
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
    path: outputFolder,
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
  plugins: [
    new webpack.BannerPlugin({ banner: "#!/usr/bin/env node", raw: true }),
    function makeExecutable() {
      this.hooks.done.tapPromise('Make executable', async () => {
        fs.readdirSync(outputFolder)
          .forEach(filePath => {
            fs.chmodSync(path.resolve(outputFolder, filePath), '755');
          })
      });
    }
  ],
};
