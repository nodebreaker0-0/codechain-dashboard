# CodeChain Dashboard Client

## Requirements

The following are the software dependencies required to install and run CodeChain-Dashboard-Client:

- [CodeChain](https://github.com/CodeChain-io/codechain)
- [CodeChain-agent-server](../server)

### Install dependencies (Ubuntu)

```
sudo apt install pkg-config libssl-dev
```

## Run

To run CodeChain-Dashboard-Client, just run

```
cargo run -- --agent-hub-url <agent-hub-url> --codechain-dir <codechain-dir> --codechain-p2p-address <codechain-p2p-address> --name <name>
```

## Formatting

Make sure you run `rustfmt` before creating a PR to the repo. You need to install the nightly-2018-12-06 version of `rustfmt`.

```
rustup toolchain install nightly-2019-10-13
rustup component add rustfmt-preview --toolchain nightly-2019-10-13
```

To run `rustfmt`,

```
cargo +nightly-2019-10-13 fmt
```

## User Manual

Under `docs` folder, run following command.

```
make html
```

User manual will be generated at `docs/_build/html`.
