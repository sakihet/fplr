---
title: Installation
description: Install fplr with Homebrew or from source.
---

## Homebrew

```sh
brew tap sakihet/tap
brew install fplr
```

## Cargo

Install from [crates.io](https://crates.io/crates/fplr). Requires a Rust toolchain.

```sh
cargo install fplr
```

## From source

Requires a Rust toolchain.

```sh
git clone https://github.com/sakihet/fplr.git
cd fplr
cargo install --path .
```

## Verify

```sh
fplr --help
```
