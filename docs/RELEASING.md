# Releasing the Rust SDK

Run the **Release Rust SDK** workflow and select a semantic-version bump. It
formats, lints, tests, and packages the root Cargo crate, commits the version,
and creates a GitHub Release containing the `.crate` source package.

GitHub Packages does not offer a Cargo-compatible registry. The GitHub-native
distribution is therefore a versioned GitHub Release artifact rather than a
crates.io publication.

Local release validation:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo package
```
