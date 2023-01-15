# Develop Environment Setup

## Requirements

* Rust 13+
* Node 19+
* wasm-pack 0.10+


## Setup

```
cargo install wasm-pack
nvm use # Use NVM to install node 19
```

## Develop Run

```shell
cd wasm
wasm-pack build
cd -
npm ci
npm run serve
```
