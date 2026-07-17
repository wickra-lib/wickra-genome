# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Repository scaffold: governance, supply-chain configuration (`deny.toml`,
  `lychee.toml`, `osv-scanner.toml`, `repo-metadata.toml`), the Rust workspace
  (`genome-core`, `genome-cli`, `genome-bench`) with the language-binding crates,
  and the `wickra-core` / `wickra-data` dependencies (the streaming indicators
  that build every asset's vector) plus the `wickra-exchange` git dependency (a
  live market feed, behind the `live` feature).

[Unreleased]: https://github.com/wickra-lib/wickra-genome/commits/main
