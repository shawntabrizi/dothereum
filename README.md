# Dothereum

The Dothereum reference node implemention; written in Rust, based on the Substrate `v2.0` framework.

### Build the node from source

1. Install Rust:
  ```bash
  curl https://sh.rustup.rs -sSf | sh
  ```
2. Install required tools:
  ```bash
  ./scripts/init.sh
  ```
3. Ensure Cargo is in your `$PATH`:
  ```bash
  export PATH=$PATH:$HOME/.cargo/bin
  ```
4. Build the node
  ```bash
  cargo build --release
  ```
5. The binary can be found in
  ```bash
  ./target/release/dothereum
  ```

### Run a Dothereum development chain

You can quickly start a local development chain with:

```bash
dothereum --dev
```

### Run a local Dothereum testnet

You can run a multi-node local testnet by using the built-in chain specification `local` and start validating blocks with the preset accounts for `--alice` and `--bob`:

```bash
dothereum --chain local \
  --base-path /tmp/local-xth/alice \
  --alice \
  --port 31337 \
  --validator \
  --node-key 00000000000000000000000000000000000000000000000000000000000a11c3
```

The `--bootnodes` flag ensures Bob connecting to Alice.

```bash
dothereum --chain local \
  --base-path /tmp/local-xth/bob \
  --bob \
  --port 34242 \
  --validator \
  --node-key 0000000000000000000000000000000000000000000000000000000000000b0b \
  --bootnodes /ip4/127.0.0.1/tcp/31337/p2p/QmWboyUFLWqHnkYzGLq5fYFzviDJbvuYG3RNNK5r8xZkYG
```

To add more validators to your network, use the preconfigured accounts `--charlie`, `--dave`, `--eve`, and `--ferdie`. Make sure to connect them to Alice's and Bob's nodes using the `--bootnodes` flag.

### Get help

Detailed logs may be shown by running the node with the following environment variables set:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 dothereum --dev
```

Additional CLI usage options are available and may be shown by running:

```bash
dothereum --help
```

For questions and bug reports, please use the [Github issue tracker](https://github.com/dothereum/dothereum/issues).