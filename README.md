# Dothereum [![Badge](https://github.com/dothereum/dothereum/workflows/Nightly/badge.svg)](https://github.com/dothereum/dothereum/actions) [![Discord](https://img.shields.io/discord/587923474471845898?label=Discord)](https://discord.gg/JcAQz58)

The Dothereum reference node implemention; written in Rust, based on the Substrate `v2.0` framework.

### Dothereum Beta

- `stage`: Testnet 0.2 Beta
- `id`: `xth_beta`
- `protocolId`: `xth`
- `slip44Index`: 442
- `specVersion`: 1006
- `ss58Format`: 20
- `tokenDecimals`: 18
- `tokenSymbol`: XTH
- `expectedBlockTime`: 15 seconds
- `genesisStateRoot`: `0x9607…9a07`
- `genesisBlockHash`: `0x6b0a…d171`

You can directly connect to the public testnet with:

```bash
dothereum --chain beta
```

- Bootnodes:
  - `/ip4/104.215.6.163/tcp/30342/p2p/QmdNPDNNB3Ct1qDpfrXPEs7J6cdQZZj8b6ZddM3kQPReY9`
  - `/ip4/40.115.178.90/tcp/30342/p2p/QmUP3WTDzfjsPpevHvv5ZHnANBuQRpDLEzzKtwDDHtgbN1`
  - `/ip4/51.15.116.226/tcp/32424/p2p/QmPcQx2Qx3oxmfNJDg33k3xaKngw4ChxrckCRrbfLnKGjs`
  - `/ip4/51.15.70.7/tcp/34242/p2p/Qmd3p9jaHC5wBjvTbEvPqRMpDr5xnUoAzxnGqJPNw6F8KJ`
  - `/ip4/51.15.71.68/tcp/34242/p2p/QmNZgNhqEHdLgsYwvDPPxp6mY8zc5LoQB96Nkgdqzz5cxs`
  - `/ip4/51.158.191.43/tcp/34242/p2p/QmTKikQzqZkq4rjUWC2reQSbGtgBh2J2CGKZPUTVjFH8tj`
- Telemetry: [telemetry.polkadot.io/#list/Dothereum Beta](https://telemetry.polkadot.io/#list/Dothereum%20Beta)

### Build the node from source

1. Install Rust:
  ```bash
  curl https://sh.rustup.rs -sSf | sh
  ```
2. Install additional dependencies, i.e,. for Ubuntu:
  ```bash
  sudo apt install build-essential cmake pkg-config libssl-dev openssl git clang libclang-dev
  ```
3. Install required tools:
  ```bash
  ./scripts/init.sh
  ```
4. Ensure Cargo is in your `$PATH`:
  ```bash
  export PATH=$PATH:$HOME/.cargo/bin
  ```
5. Build the node
  ```bash
  cargo build --release
  ```
6. The binary can be found in
  ```bash
  ./target/release/dothereum
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

### Run a Dothereum development chain

You can quickly start a local development chain with:

```bash
dothereum --dev
```

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
