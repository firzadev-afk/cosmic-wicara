# Changelog

> All notable changes to **cosmic-wicara** are documented here.
> The format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and [Semantic Versioning](https://semver.org/).
> Each entry explains *what changed* and *why it matters*.

---

## [Unreleased]

> Not released yet. This section lists changes that are done in code but not part of an official release.

### Added

- **All 15 dialogs + `about` are wired to real `libcosmic` widgets:**
  - Text input (`entry` / `password`)
  - File chooser via `xdg-portal` (native COSMIC file picker)
  - Calendar (*calendar*), numeric slider (*scale*), color picker (*color-selection*)
  - Interactive list (*list*), text viewer with checkbox (*text-info*)
  - Progress bar from `stdin` (*progress*), notification with `--listen` (*notification*)
  - Multi-field forms (*forms*)
- **New options:**
  - `--extra-button` — add an extra button (may be repeated)
  - `--icon`, `--window-icon` — dialog icon / window icon
  - `--ellipsize`, `--no-wrap` — text display controls
  - `--switch`, `--default-cancel` — button and focus controls
  - `--json` — dialog result as JSON (machine-readable)
  - `--lang=en|id` — language for built-in dialog text (English / Indonesian)
- **Dialog result contract (`DialogResult`):** `stdout` data is printed once when the dialog finishes, so scripts can easily capture it.

### Fixed

- **Timeout now consistently exits `5` on every path** (previously one path exited `1` due to message mapping).
- **Brand cleanup:** no leftover external references in code, the man page (`man/cosmic-wicara.1`), or the desktop entry (`cosmic-wicara.desktop`).
- **Documentation:** the `docs/` folder (old plans) was removed; all documentation now lives in `README.md`, `CHANGELOG.md`, and the man page `man/cosmic-wicara.1` so it is easier to find and understand.

---

## [0.1.0] — Initial release

> First release: a native COSMIC dialog tool (Rust + `libcosmic`, COSMIC-only, Wayland-first).

### Added

- **15 dialogs + `about`:**
  - `calendar`, `entry`, `error`, `info`, `file-selection`, `list`, `notification`, `progress`, `question`, `warning`, `scale`, `text-info`, `color-selection`, `password`, `forms`, plus `about`.
- **General options:**
  - `--title`, `--width`, `--height`, `--timeout`, `--ok-label`, `--cancel-label`
  - `--extra-button`, `--modal`
  - `--attach` (*deprecated*, no-op — kept for script compatibility)
  - `--json`, `--lang=en|id`, `--window-icon`
- **Per-dialog help selectors:** `--help-<dialog>` (e.g. `--help-question`) and `--version`.
- **Exit and stream contract:**
  - Exit `0` = OK/help, `1` = Cancel/close, `5` = timeout, `255` = error.
  - `stdout` carries data only; `stderr` carries diagnostics.
  - Exactly one dialog type per invocation.
- **COSMIC window behaviour:**
  - Dialog windows float and size to their content (not a tiled application window).
  - Light/dark follows `cosmic::Theme` (the COSMIC system theme).
  - Frosted glass via `enable_blur(true)` (blur behind the window).
  - Focus requested on map.
- **Packaging:**
  - `cosmic-wicara` binary in `/usr/bin` (Arch `packaging/PKGBUILD`, `cargo build --release`, `[profile.release]` LTO + strip).
  - Man page `man/cosmic-wicara.1`.
  - Desktop entry `cosmic-wicara.desktop` (`Exec=cosmic-wicara`, `Terminal=false`, `Categories=Utility;`).
- **Tooling:**
  - `just` recipes: `build`, `build-release`, `test`, `test-cli`, `install`, `fmt`, `clippy`, `list` (see `just --list`).

### Changed

- No behaviour changes in the first release (baseline).

### Fixed

- No bug fixes in the first release (baseline).

---

## How to read this changelog

| Section heading | Meaning |
|---|---|
| `Added` | New features |
| `Changed` | Changes to existing behaviour |
| `Fixed` | Bugs that were fixed |
| `Unreleased` | Changes not yet released |
| `[0.1.0]` | Release version, following Semantic Versioning |
