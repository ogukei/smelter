# Smelter

Simple web app interface builder in Rust.

```rust
let mut builder = DocumentBuilder::new();
let context = &mut builder;
Division::new(context).children(|context| {
    Paragraph::new(context)
        .text("Smelter");
    Division::new(context).children(|context| {
        Button::new(context)
            .text("Hello world!");
    });
});
let document = builder.build();
/*
<div>
  <p>Smelter</p>
  <div>
    <button>Hello world!</button>
  </div>
</div>
*/
```

The repository contains some Cargo workspace members. `smelter-ui` contains the core functionality. `smelter-reflux` is a Combine-like declarative event processing library. `smelter-webapp` is an example app that runs on web-sys with Webpack 5.

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
