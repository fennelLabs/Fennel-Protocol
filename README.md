# Fennel Protocol

## Getting Started

Follow the steps below to get started. Make sure you have `libclang` installed:

#### Homebrew
```sh
brew install --with-toolchain llvm
```

#### Chocolatey
```sh
choco install llvm
```

#### Ubuntu
```sh
apt install libclang-dev
```

### Repository Set-Up

After cloning this repository, be sure to include necessary submodules.

```sh
git submodule init
git submodule update
```

### Rust Setup

If you do not already have Rust installed, with the nightly build set as default, complete the [installation](https://www.rust-lang.org/tools/install).

Following the installation of Rust, ensure your environment is defaulting to the nightly build:

```sh
rustup default nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

### Run

Use Rust's native `cargo` command to build and launch the template node:

```sh
cargo run --release -- --dev --tmp
```

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/release/node-fennel-protocol --dev
```

Purge the development chain's state:

```bash
./target/release/node-fennel-protocol purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_BACKTRACE=1 ./target/release/node-fennel-protocol -ldebug --dev
```

### Connect with Polkadot-JS Apps Front-end

Once the node is running locally, you can connect it with **Polkadot-JS Apps** front-end
to interact with your chain. [Click
here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your
local node template.

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action, refer to our
[Start a Private Network tutorial](https://docs.substrate.io/tutorials/v3/private-network).

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
docker compose build; docker compose up -d
```

This command will firstly compile your code, and then start a local development network.
