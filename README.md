# µChain - Blockchain for prototyping

> Work in progress

µChain _pronounced_ `mu-chain`, is a blockchain base/framework for prototyping blockchain projects

## Quickstart

### 🧰 Install Dependencies

* **Rust Toolchain `Linux/MacOS`**
    ```shell
    curl https://sh.rustup.rs -sSf | sh
    ```
    ```shell
    rustup default nightly
    ```
* **Install gRPC, RockDB dependencies**

  **`Linux`**

  ```shell
  sudo apt install clang libclang-dev llvm llvm-dev linux-kernel-headers libev-dev
  ```
  ```shell
  sudo apt install cmake libprotobuf-dev protobuf-compiler
  ```
  **`MacOS`**

  ```shell
  brew install --with-toolchain llvm
  ```
  ```shell
  brew install cmake automake libtool
  ```

### ⬇️ Download

* Download the source code
    ```shell
    git clone https://github.com/mambisi/uchain
    ```
    ```shell
    cd uchain
    ```

### ⌛️ Running node `Linux/MacOS`

* Build the node from source
    ```shell
    cargo build --release
    ```
* Generate Node Identity
    ```shell
    ./target/release/uchain identity generate
    ```
* Initialize node configuration
    ```shell
    ./target/release/uchain config init
    ```
* Create a miner account (optional - required if you want to run as a miner)
  ```shell
  ./target/release/uchain account new
  ```
  Output:
  ```json
  {
    "address": "0xffff…ffff",
    "secret" : "0xffff…ffff"
  }
  ```
  Set miner
  ```shell
  ./target/release/uchain config update --miner="0xffff…ffff"
  ```
* Run node
  ```shell
  ./target/release/uchain run
  ```

### RPC Usage

µChain uses gRPC to interact with the node, user can use [bloomRPC](https://github.com/bloomrpc/bloomrpc.git) to
interact with the blockchain by importing [.proto](/proto/schema) files, default rpc port `9121`.

* **Protobuf files can be found** [here](/proto/schema)
* **Documentation**
  [RPC Documentation](/docs/rpc.md)
* **Clients**
    * Gui: [bloomRPC](https://github.com/bloomrpc/bloomrpc.git)
    * Cli: [gRPCurl](https://github.com/fullstorydev/grpcurl)
