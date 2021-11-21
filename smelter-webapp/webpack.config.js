const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    mode: 'production',
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    module: {
        rules: [
            {
                test: /\.wasm$/,
                type: 'webassembly/sync',
            }
        ]
    },
    experiments: {
        syncWebAssembly: true
    },
    plugins: [
        new HtmlWebpackPlugin(),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, ".")
        })
    ]
};
