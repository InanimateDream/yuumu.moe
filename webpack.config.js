const path = require('path');
const dist_path = path.resolve(__dirname, "dist");

const wasm_pack = require('@wasm-tool/wasm-pack-plugin');
const copy_webpack = require('copy-webpack-plugin');

module.exports = (env, argv) => {
  return {
    devServer: {
      hot: true,
      inline: true,
      port: 8000,
      contentBase: dist_path,
      compress: argv.mode === 'production',
    },
    entry: './init.js',
    output: {
      path: dist_path,
      filename: "yuumu.js",
      webassemblyModuleFilename: "yuumu.wasm"
    },
    plugins: [
      new copy_webpack([
        { from: './static', to: dist_path }
      ]),
      new wasm_pack({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
      })
    ],
    module: {
      rules: [
        {
          test: /\.less$/,
          loader: 'less-loader',
        },
      ],
    },
    watch: argv.mode !== 'production'
  };
};