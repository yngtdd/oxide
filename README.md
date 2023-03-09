# Oxide

[![Rust](https://github.com/yngtdd/oxide/actions/workflows/rust.yml/badge.svg)](https://github.com/yngtdd/oxide/actions/workflows/rust.yml)

Adding a little Rust to reliability software.

## Installation

Building the AWS lambda function to run on the Graviton2 processor:

```
cargo lambda build --release --arm64
```

Deploying to AWS lambda:

```
cargo lambda deploy oxide-lambda
```

