# ReleaseSeal

**Generate checksums, SBOM metadata, and release verification artifacts from one CLI.**

> **Status:** early development. No stable release has been published.

ReleaseSeal is intended to make small-project release hygiene easier to reproduce from a single local command.

## Planned v0.1

- SHA-256 checksum generation
- deterministic manifest describing release files
- optional SBOM generation through a documented format/toolchain
- explicit tool/version provenance in generated metadata
- verification command for an existing release manifest
- no signing claims unless cryptographic signing is actually configured

The current repository is a development scaffold. Checksum, SBOM, provenance, and signing functionality are **not implemented yet**.

## Build

Requires Rust 1.74 or newer.

```bash
cargo build
cargo test
```

## Security

See [SECURITY.md](SECURITY.md).

## License

MIT © BLC Core Studio
