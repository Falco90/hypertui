# Hypertui

[Hypertui]is a terminal user interface (TUI) for on-chain data on Ethereum. It makes use of Envio's HyperSync feature to query Ethereum and EVM compatible chains.
Hypertui is created during ETHOnline 2024.

## Installation
This application is written in Rust, so make sure you have Rust installed on your system.
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

## Quickstart
To start the application run
```shell
cargo run
```