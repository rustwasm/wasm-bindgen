const webpack = require('webpack');
const HtmlWebpackPlugin = require('html-webpack-plugin');

const path = require('path');

module.exports = [
  {
    mode: 'production',
    entry: path.join(__dirname, './src/client/index.ts'),
    output: {
      filename: 'app.js',
      path: path.join(__dirname, './dist/client/'),
      publicPath: '/'
    },
    plugins: [
      new webpack.EnvironmentPlugin({
        NODE_ENV: 'production'
      }),
      new HtmlWebpackPlugin({
        filename: 'index.html',
        template: './src/client/index.html'
      })
    ],
    resolve: {
      extensions: [ '.ts', '.tsx', '.js', '.jsx', '.json', '.wasm' ]
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
  },
  {
    mode: 'production',
    target: 'node',
    entry: path.join(__dirname, './src/server/index.ts'),
    output: {
      filename: 'index.js',
      path: path.join(__dirname, './dist/server/')
    },
    plugins: [
      new webpack.EnvironmentPlugin({
        NODE_ENV: 'production'
      })
    ],
    resolve: {
      extensions: [ '.ts', '.tsx', '.js', '.jsx', '.json' ]
    },
    externals: [require('webpack-node-externals')()],
    node: {
      __dirname: false,
      __filename: false,
      console: true
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
                      node: 'current'
                    },
                    modules: false
                  }]
                ]
              }
            },
            {
              loader: 'awesome-typescript-loader'
            }
          ]
        }
      ]
    }
  }
];
