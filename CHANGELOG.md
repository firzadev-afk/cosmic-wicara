# Changelog

All notable changes to cosmic-wicara are documented here. This project
follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and
Semantic Versioning.

## [Unreleased]

### Added

- Wire all 15 dialogs + about to real libcosmic widgets: entry/password
  inputs, file chooser (xdg-portal), calendar, scale slider, color
  picker, interactive list, text-info with checkbox, progress bar from
  stdin, notification `--listen`, forms.
- `--extra-button`, `--icon`, `--ellipsize`, `--no-wrap`, `--switch`,
  `--default-cancel`, `--json`, `--lang=en|id`, `--window-icon`.
- `DialogResult` contract: stdout data printed once at exit.

### Fixed

- Timeout now exits 5 in all paths (was 1 via message mapping).
- Brand cleanup: no leftover external references in code, man page,
  or desktop entry.


- Initial release: native COSMIC dialog tool (Rust + libcosmic,
  COSMIC-only, Wayland-first).
- 15 dialogs: calendar, entry, error, info, file-selection, list,
  notification, progress, question, warning, scale, text-info,
  color-selection, password, forms, plus about.
- General options: `--title --width --height --timeout --ok-label
  --cancel-label --extra-button --modal --attach` (deprecated no-op),
  per-dialog `--help-*` selectors, and `--version`.
- Exit contract: `0` OK/help, `1` Cancel/close, `5` timeout, `255` error.
  `stdout` data only, `stderr` diagnostics.
- COSMIC window behaviour: floating content-sized dialog surfaces,
  light/dark via `cosmic::Theme`, frosted glass via `enable_blur(true)`
  (behind-window blur), and focus requested on map.
- Binary `cosmic-wicara` installed to `/usr/bin` (Arch `packaging/PKGBUILD`
  with `cargo build --release`, `[profile.release]` LTO + strip).
- Packaging: `man/cosmic-wicara.1`, `cosmic-wicara.desktop`
  (`Exec=cosmic-wicara`, `Terminal=false`, `Categories=Utility;`).
- Tooling: `just` recipes `build / build-release / test / test-cli / install
  / fmt / clippy / list` (see `just --list`).
