# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-17

### Added
- Initial implementation of Audiobookshelf CLI in Rust.
- Core subcommands: `ping`, `libraries list`, `items list/get`, `authors list/get`, `playlists list/get`, `series list/get`, `metadata tags/genres`.
- authenticated client using `reqwest` with `rustls-tls` for cross-platform compatibility.
- GitHub Actions CI for automated build and test.
- `beads` integration for issue tracking.
- Automated release workflow for Linux, macOS, and Windows.
