# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Changed

- Corrected the macOS bundle identifier used when packaging the `.app`/`.dmg`.
- Updated documentation to reflect the current GitHub username/repository.

## [0.1.0] - 2026-08-28

Initial release.

### Added

- Shared `abyssal_abacus_core` crate with two arithmetic APIs:
  - `evaluate_expr`, a one-shot text-expression parser (`"3+4"`, `"5*-3"`, ...).
  - `CalcEngine`, a stateful four-function engine for button/keyboard-driven UIs.
- `abyssal_abacus_cli`: an interactive REPL and a one-shot mode
  (`abyssal-abacus-cli "3 + 4"`) for scripting.
- `abyssal_abacus_gui`: a retro black/red desktop calculator built on
  egui/eframe, with full keyboard support, a custom-drawn title bar, and a
  resizable, scale-and-center layout.
- Support for `+ - * / %`, decimals, and negative numbers.
- Division and modulo by zero resolve to `0` instead of erroring.
- GitHub Actions release workflow that builds and publishes Linux, macOS,
  and Windows binaries for both the CLI and GUI on `vX.Y.Z` tags.

[Unreleased]: https://github.com/AbyssalOath/abyssal-abacus/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/AbyssalOath/abyssal-abacus/releases/tag/v0.1.0
