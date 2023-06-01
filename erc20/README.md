# Casper ERC20

A library and example implementation of ERC20 token for the Casper network.

## Prerequisites

You need to have a x86_64 CPU to build the code, as Casper dependencies for now don't support ARM (e.g. M1) architecture CPU's.
It is recommended to use Linux, Debian-based distributions (e.g. Ubuntu 22.04).

First, you need to install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

To check for successful installation, one needs to run this command:

```bash
rustup --version.
```

Secondly, you'll need to install `wabt` tooling:

```bash
sudo apt install wabt
```

Thirdly, you need to install CMake:

```bash
sudo apt-get -y install cmake
```

To check for successful installation, one needs to run this command:

```bash
cmake --version
```

To install `casper-client`, to interact with the contract using the CLI you must install these dependencies:

```bash
sudo apt-get install pkg-config
sudo apt-get install openssl
sudo apt-get install libssl-dev
```

And finally:

```bash
cargo install casper-client
```

To check for successful installation, one needs to run this command:

```bash
casper-client get-state-root-hash --node-address http://65.21.235.219:7777
```


## Install
Make sure the `wasm32-unknown-unknown` Rust target is installed.
```
make prepare
```

## Build Smart Contracts
To build the example ERC20 contract and supporting test contracts:
```
make build-contracts
```

## Test
```
make test
```
