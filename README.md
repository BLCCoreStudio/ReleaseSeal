# ReleaseSeal

**Create and verify deterministic SHA-256 release manifests from one CLI.**

> **Status:** development preview. No stable release has been published.

ReleaseSeal is intended to make small-project release hygiene easier to reproduce without pretending that checksums alone provide signing, provenance, or complete supply-chain security.

## Current preview

Create a checksum manifest:

```bash
releaseseal create dist/SHA256SUMS dist/app.tar.gz dist/app.tar.gz.asc
```

Verify it later:

```bash
releaseseal verify dist/SHA256SUMS
```

The current implementation:

- computes SHA-256 internally with no runtime checksum utility dependency
- streams files instead of loading entire release artifacts into memory
- writes manifest entries in deterministic path order
- writes the manifest through a temporary file and rename
- verifies each listed file and reports `OK`, `MISMATCH`, or `ERROR`

A successful verification exits `0`; checksum/file failures exit `1`; invalid input or manifest errors exit `2`.

## Scope

SBOM generation, provenance attestations, and cryptographic signing are **not implemented yet**. ReleaseSeal will not make signing or attestation claims until those capabilities actually exist and are testable.

## Build

Requires Rust 1.74 or newer.

```bash
cargo build --locked
cargo test --locked
```

## Security

See [SECURITY.md](SECURITY.md).

## License

MIT © BLC Core Studio
