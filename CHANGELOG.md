# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2026-04-19

### Added
- **Book Upload**: New `abs upload` command to upload books directly to the server.
- **Folder Auto-resolution**: Automatically detects the correct library folder if only one exists.

## [1.0.0] - 2026-04-18

### Added
- **Secure Authentication**: System keyring integration (macOS Keychain, Linux Secret Service) for API keys.
- **Shell Autocompletion**: Support for Bash, Zsh, Fish, and PowerShell.
- **End-to-End Testing**: Integration test suite using `assert_cmd` for binary verification.
- Improved error mapping for common API failure modes.

### Changed
- Standardized command argument structure across all subcommands.
- Updated README with comprehensive usage examples and security guidelines.

## [0.3.0] - 2026-04-17

### Added
- Item metadata update support (`abs items update`).
- Bulk item metadata updates (`abs items bulk-update`).
- Support for library item matching/unmatching against external providers.

## [0.2.0] - 2026-04-17

### Added
- Global search command (`abs search`) across all authorized libraries.
- Library scan command (`abs libraries scan`).
- Server status and information command (`abs info`).

## [0.1.0] - 2026-04-17

### Added
- Initial implementation of Audiobookshelf CLI in Rust.
- Core subcommands: `ping`, `libraries list`, `items list/get`, `authors list/get`, `playlists list/get`, `series list/get`, `metadata tags/genres`.
- Authenticated client using `reqwest` with `rustls-tls` for cross-platform compatibility.
- GitHub Actions CI for automated build and test.
- `beads` integration for issue tracking.
- Automated release workflow for Linux, macOS, and Windows.

