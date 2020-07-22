const CopyPlugin = require('copy-webpack-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const path = require('path');

module.exports = {
    entry: './src/bootstrap.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: "bootstrap.js",
    },
    plugins: [
        new CleanWebpackPlugin(),
        new CopyPlugin({
            patterns: [
                { from: 'public/*', to: path.resolve(__dirname, 'dist'), flatten: true },
            ],
        }),
    ],
};