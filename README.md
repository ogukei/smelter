# Smelter

Simple web app interface builder in Rust.

`smelter-ui` contains the core functionality. `smelter-webapp` is an example app that runs on web-sys with Webpack 5.

## Setup

Install the latest npm using n on Ubuntu 20.04 LTS. Skip this step if you have already installed npm.

```
sudo apt install nodejs npm
sudo npm install -g n
sudo n stable
sudo apt purge nodejs npm
exec $SHELL -l
node -v
npm -v
```

Execute npm install.
```
cd <repository-dir>/smelter-webapp
npm install
```

## Build

```
npm run build
```

## Run

```
npm run serve
```
