# Security Policy

## Reporting a Vulnerability

Report vulnerabilities privately via
[GitHub Security Advisories](https://github.com/zircote/adrscope/security/advisories/new).
Do not open a public issue for security reports.

## Verifying Release Artifacts

Every release binary is built on GitHub Actions and carries
[SLSA build provenance](https://slsa.dev/spec/v1.0/provenance) attested with
`actions/attest-build-provenance`. The release pipeline verifies every
attestation fail-closed before the GitHub Release is published — a tag
publishes nothing unattested.

To verify a downloaded artifact yourself (requires the
[`gh` CLI](https://cli.github.com/), authenticated):

```sh
gh release download v<version> --repo zircote/adrscope
gh attestation verify adrscope-<version>-<platform> --repo zircote/adrscope
```

For example:

```sh
gh attestation verify adrscope-0.4.0-linux-amd64 --repo zircote/adrscope
```

A successful verification prints `✓ Verification succeeded!` and confirms
the binary is byte-identical to what GitHub Actions built from this
repository. Verification fails closed if the file was modified, rebuilt
elsewhere, or attested by any other repository or workflow.

The `zircote/adrscope` GitHub Action performs this same verification
automatically before running any downloaded release binary.

### SBOM

Each release ships a CycloneDX SBOM (`adrscope-<version>-sbom.cdx.json`)
generated with Syft, and every binary carries an SBOM attestation binding
it to that SBOM. To verify:

```sh
gh attestation verify adrscope-<version>-<platform> --repo zircote/adrscope \
  --predicate-type https://cyclonedx.org/bom
```

### crates.io Source Package

The published `.crate` source archive also carries SLSA build provenance,
attested against the exact bytes the registry serves:

```sh
curl -fsSLO https://static.crates.io/crates/adrscope/adrscope-<version>.crate
gh attestation verify adrscope-<version>.crate --repo zircote/adrscope
```

Note that binaries you compile yourself from the crate are not
byte-identical to the attested release binaries — Rust builds are not
reproducible by default. The attestation covers the source archive;
crates.io's checksum chain and Cargo.lock pin it from there.

### Checksums

`adrscope-<version>-checksums.txt` lists SHA-256 digests of every release
asset for quick integrity checks (`sha256sum -c`). Checksums are a
convenience; the attestations above are the authoritative, fail-closed
verification path.
