# Contributing to SolarChain Contract

Thank you for contributing to SolarChain Contract. This repository is intended to be a clean, maintainable Soroban smart contract foundation for community solar crowdfunding on Stellar.

## How to contribute

- Open an issue for bugs, improvements, or feature requests.
- Fork the repository and create a branch named after the issue or change. Example: `feature/campaign-milestones` or `bug/fix-storage-key`.
- Send a pull request from your fork.

## Code standards

- Use `rustfmt` formatting.
- Keep code simple and documented.
- Add tests for new contract behavior.
- Avoid breaking changes without clear migration guidance.

## Development workflow

1. Fork and clone the repository.
2. Create a feature branch.
3. Implement the change.
4. Run formatting and tests.
5. Open a pull request with a clear description.

## Testing

Run:

```bash
cargo fmt --all
cargo test --release --target wasm32-unknown-unknown
```

## Issue reports

Provide:

- A concise title.
- Steps to reproduce.
- Expected behavior.
- Actual behavior.
- Any relevant logs or error output.

## Pull request expectations

A good PR includes:

- Summary of the change.
- Linked issue when available.
- Test coverage for new behavior.
- Updated documentation if needed.
- A successful CI run.

## Code of conduct

By participating in this project, you are expected to follow the [Code of Conduct](CODE_OF_CONDUCT.md).
