# Dothereum
The Dothereum reference node implemention; written in Rust, based on the Substrate framework.

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

### Run a Dothereum development chain

You can start a local development chain with:

```bash
dothereum --dev
```

Detailed logs may be shown by running the node with the following environment variables set: `RUST_LOG=debug RUST_BACKTRACE=1 dothereum --dev`.

### Get help

Additional CLI usage options are available and may be shown by running `dothereum --help`.

For questions and bug reports, please use the [Github issue tracker](https://github.com/dothereum/dothereum/issues).