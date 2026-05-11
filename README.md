# SolarChain Contract

A Soroban smart contract repository for community solar fundraising and transparent contribution tracking on Stellar.

## Overview

SolarChain Contract is the blockchain core for SolarChain, enabling campaign management, donation recording, and fund tracking in a Soroban-compatible smart contract.

## Highlights

- On-chain campaign creation and contribution tracking
- Soroban smart contract built for the Stellar ecosystem
- Foundation for community solar financing, donor transparency, and impact verification
- GitHub Actions CI for build, format, and test validation

## Repository structure

- `Cargo.toml` — Rust package manifest for the contract
- `rust-toolchain.toml` — pins the Rust toolchain and required components
- `src/lib.rs` — smart contract implementation
- `.github/workflows/ci.yml` — automated build and test workflow
- `CONTRIBUTING.md` — contribution guidelines
- `CODE_OF_CONDUCT.md` — community standards
- `.github/ISSUE_TEMPLATE/` — issue templates for bugs and feature requests
- `.github/pull_request_template.md` — pull request guidance

## Getting started

Prerequisites:

- Rust toolchain (stable)
- `wasm32-unknown-unknown` target
- Soroban SDK dependency configured in `Cargo.toml`

Install the wasm target:

```bash
rustup target add wasm32-unknown-unknown
```

Build the contract:

```bash
cargo build --target wasm32-unknown-unknown --release
```

Run tests:

```bash
cargo test --release --target wasm32-unknown-unknown
```

Format code:

```bash
cargo fmt --all
```

## Development

This repository is designed to be a clean, maintainable foundation for Soroban smart contract work.

Recommended workflow:

1. Create a branch for your change.
2. Run `cargo fmt --all`.
3. Run tests with `cargo test --release --target wasm32-unknown-unknown`.
4. Open a pull request using the template in `.github/pull_request_template.md`.

## Contribution and community

Please review `CONTRIBUTING.md` before submitting changes.

This project follows the guidelines in `CODE_OF_CONDUCT.md`.

## License

This repository is licensed under the MIT License. See `LICENSE` for details.
