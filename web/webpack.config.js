const CopyPlugin = require('copy-webpack-plugin');
const path = require('path');

module.exports = {
    entry: './bootstrap.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: "bootstrap.js",
    },
    plugins: [
        new CopyPlugin({
            patterns: [
                { from: 'index.html', to: path.resolve(__dirname, 'dist') },
            ],
        }),
    ],
};