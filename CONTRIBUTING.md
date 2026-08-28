# Contributing

Contributions around deterministic manifests, checksum verification, SBOM interoperability, provenance metadata, tests, and documentation are welcome.

Before opening a pull request:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

Cryptographic or supply-chain claims must be precise, testable, and documented. Follow `SECURITY.md` for vulnerability reports.
