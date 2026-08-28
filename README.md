# ReleaseSeal

**Focused deterministic release-manifest research.**

> **Companion research status:** ReleaseSeal's useful deterministic SHA-256 manifest creation and verification direction has been integrated into [HashCheck](https://github.com/BLCCoreStudio/HashCheck). This repository remains public as a focused implementation history/reference rather than being deleted or republished.

ReleaseSeal explored a narrow workflow for creating and verifying SHA-256 manifests for groups of release artifacts.

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

- computes SHA-256 locally
- streams files instead of loading entire release artifacts into memory
- writes manifest entries in deterministic path order
- writes the manifest through a temporary file and rename
- verifies each listed file and reports `OK`, `MISMATCH`, or `ERROR`

## Primary integration target

New checksum-manifest development now targets **HashCheck**, which already provides released SHA-256/SHA-512 single-file calculation and verification. The integrated development line adds:

```bash
hashcheck manifest create <MANIFEST> <FILE>...
hashcheck manifest verify <MANIFEST>
```

The HashCheck integration also constrains manifest entries to relative paths inside the manifest directory and rejects absolute/parent-traversal entries during verification.

ReleaseSeal remains useful for understanding the original focused experiment and preserving its commit history and existing links.

## Scope

Checksums and checksum manifests provide byte-integrity verification; they do not by themselves provide artifact authenticity.

SBOM generation, provenance attestations, and cryptographic signing are **not implemented here** and are not implied by the HashCheck integration.

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
