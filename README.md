# HyperTui

HyperTui is a tool that displays on-chain data and analytics in a terminal user interface (TUI)! It makes use of Envio's [HyperSync](https://docs.envio.dev/docs/HyperSync/overview) feature to query Ethereum and EVM compatible chains. It heavily leverages the [Ratatui](https://ratatui.rs/) library to create the UI.
Hypertui is created during ETHOnline 2024.

## Installation
HyperTui is written in Rust, so make sure you have Rust [installed](https://www.rust-lang.org/tools/install) on your system.
In order to build the HyperSync rust-client you will also need to install the capnproto tool:

Linux
```shell
apt-get install -y capnproto libcapnp-dev
```

Windows
```shell
choco install capnproto
```

MacOs
```shell
brew install capnp
```

## Running the Application
To start the application run
```shell
cargo run
```