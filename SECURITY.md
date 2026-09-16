# Security Policy

## Scope

Abyssal Abacus is a local, offline calculator (CLI + desktop GUI). It does
not make network requests, read or write files other than its own binary,
or handle untrusted input beyond the arithmetic expressions/keystrokes a
user types into it directly. The realistic attack surface is therefore
small, but we still welcome reports about:

- Panics, crashes, or hangs triggered by malformed CLI input or GUI input
  (e.g. crafted expressions, extreme numbers, malformed one-shot arguments).
- Memory-safety issues (this project uses only safe Rust; `unsafe` in a
  dependency that's reachable from our code counts too).
- Vulnerabilities in a direct dependency (see `Cargo.lock`) that are
  actually reachable through how we use that dependency.
- Supply-chain concerns with the release pipeline
  (`.github/workflows/release.yml`) or `release.sh`.

Out of scope: the fact that division/modulo by zero returns `0` instead of
an error - that's an intentional design choice (see `ARCHITECTURE.md`), not
a bug.

## Supported Versions

Only the latest tagged release is actively supported. Please upgrade to the
newest version from the [Releases](https://github.com/AbyssalOath/abyssal-abacus/releases)
page before reporting an issue, in case it's already fixed.

## Reporting a Vulnerability

Please report security issues privately using
[GitHub Security Advisories](https://github.com/AbyssalOath/abyssal-abacus/security/advisories/new)
for this repository, rather than opening a public issue.

Include, where relevant:

- The version/commit affected.
- Steps to reproduce (exact input, platform, and whether it's the CLI or GUI).
- The observed impact (crash, panic, incorrect result, etc.).

We aim to acknowledge reports within a few days. Since this is a small,
volunteer-maintained project, fixes are prioritized by severity and shipped
in the next tagged release rather than as hotfix patches.

## Dependencies

`abyssal_abacus_core` has zero third-party dependencies by design. Third-party
dependencies (egui, eframe, and their transitive dependency tree) are
isolated to `abyssal_abacus_gui`; see `Cargo.lock` for exact pinned versions.
Dependency updates are reviewed for security advisories (e.g. via
`cargo audit`) before being merged.
