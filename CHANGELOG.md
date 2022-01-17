# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- `/docs/`, a submodule that contains the overarching documentation of RTIC Scope, which is rendered at [the organization profile](https://github.com/rtic-scope).

### Changed
- On `--serial /path/to/dev`, `dev` will no longer unconditionally configure for 115200 B/s; the baud rate specified with `tpiu_baud` in the `[package.metadata.rtic-scope]` block in `Cargo.toml` will instead be applied.
  For example, `tpiu_baud = 9600` will configure `dev` for 9600 B/s.
  Valid baud rates are listed in [`nix::sys::termios::BaudRate`](https://docs.rs/nix/0.23.1/nix/sys/termios/enum.BaudRate.html), with the exception of `B0`.
- Improved the warning message when an overflow packet is decoded.
  It will now detail that non-timestamp packets have been dropped and/or that the local timestamp counter wrapped which means that timestamps from then on are *potentially* diverged.
- Ignore enters and exits relating to the `ThreadMode` interrupt: RTIC always executes tasks in handler mode and then returns to `ThreadMode` on `cortex_m::asm::wfi()`.
- Bumped `itm` to v0.7.0.

### Deprecated

### Removed

### Fixed
- No longer prints "Target reset and flashed." on `trace --dont-touch-target`.

### Security

## [0.3.0] - 2022-01-05
Initial release tracked by this changelog.

[Unreleased]: https://github.com/rtic-scope/cargo-rtic-scope/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/rtic-scope/cargo-rtic-scope/releases/tag/v0.3.0