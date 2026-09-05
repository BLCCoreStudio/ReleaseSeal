# ReleaseSeal

> **Project status: sunset / discontinued.**

ReleaseSeal was a focused deterministic release-manifest experiment. Its useful SHA-256 manifest creation and verification direction has been integrated into [HashCheck](https://github.com/BLCCoreStudio/HashCheck), which is the maintained implementation line.

The repository remains public for historical reference and to preserve existing links and commit history, but **no further feature development or routine maintenance is planned**.

## Historical scope

ReleaseSeal explored a narrow local workflow for:

- creating deterministic SHA-256 manifests for release artifacts;
- streaming files instead of loading complete artifacts into memory;
- ordering manifest entries deterministically;
- writing manifests through a temporary file and rename;
- verifying listed artifacts and reporting integrity failures explicitly.

Checksums provide byte-integrity verification; they do not by themselves establish artifact authenticity. Signing, provenance attestations, and SBOM generation were outside this project's scope.

## Maintained alternative

For checksum calculation, verification, and manifest workflows, use [HashCheck](https://github.com/BLCCoreStudio/HashCheck).

## Historical source

Previous implementation details, tests, documentation, and development history remain available through the Git history.

## License

MIT © BLC Core Studio
