const webpack = require('webpack');
const HtmlWebpackPlugin = require('html-webpack-plugin');

const path = require('path');

module.exports = [
  {
    mode: 'development',
    entry: path.join(__dirname, './src/client/index.ts'),
    output: {
      filename: 'app.js',
      path: path.join(__dirname, './dist/client/')
    },
    devtool: 'inline-source-map',
    plugins: [
      new webpack.EnvironmentPlugin({
        NODE_ENV: 'development'
      }),
      new HtmlWebpackPlugin({
        filename: 'index.html',
        template: './src/client/index.html'
      })
    ],
    resolve: {
      extensions: [ '.ts', '.tsx', '.js', '.jsx', '.json', '.wasm' ]
    },
    watchOptions: {
      ignored: [
        /target\/.*/
      ]
    },
    module: {
      rules: [
        {
          test: /\.tsx?$/,
          exclude: /node_modules/,
          use: [
            {
              loader: 'babel-loader',
              query: {
                babelrc: false,
                presets: [
                  ['env', {
                    targets: {
                      browsers: [
                        '>1%'
                      ]
                    },
                    useBuiltIns: 'entry',
                    modules: false
                  }]
                ],
                plugins: [
                  'syntax-dynamic-import'
                ]
              }
            },
            {
              loader: 'awesome-typescript-loader'
            }
          ]
        },
        {
          test: /\.html$/,
          loader: 'html-loader'
        }
      ]
    }
  }
];
