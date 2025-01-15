const path = require('path');

module.exports = {
  entry: './src/index.ts',  // Entry file for your TypeScript code
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: ['.ts', '.js'],  // Resolve .ts and .js extensions
  },
  output: {
    filename: 'bundle.js',  // Output file
    path: path.resolve(__dirname, 'dist'),
  },
};

