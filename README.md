# cosmic-wicara

Native COSMIC dialog tool in Rust + libcosmic. Single binary
`cosmic-wicara` for shell scripts, installers, and desktop workflows on
COSMIC.

> **COSMIC-only, Wayland-first.** Built for a COSMIC session. Not a GTK
> app, not portable to other desktop environments or X11.

## Status

- Builds with `cargo build --release -p cli` (LTO + strip).
- Workspace tests, clippy, and fmt are green in CI.
- Dialog windows float and size to their content: no tiled application
  window, no compositor header, and no double title.
- A visual check inside a live COSMIC session is still recommended before
  cutting a release. Headless validation (exit 255 on bad input) is
  covered by tests.

## Features

- **15 dialogs + about**: calendar, entry, error, info, file-selection,
  list, notification, progress, question, warning, scale, text-info,
  color-selection, password, forms, about.
- **General options**: `--title --width --height --timeout --ok-label
  --cancel-label --extra-button --modal --attach` (deprecated no-op),
  plus `--json` (machine-readable result), `--lang=en|id`,
  `--window-icon`, `--icon`, `--ellipsize`, `--no-wrap`, `--switch`,
  `--default-cancel`.
- **Real widgets per dialog**: file chooser via xdg-portal, calendar
  picker, slider, color picker, list with checklist/radio/editable,
  text viewer with `--checkbox`, progress bar fed from stdin.
- **stdout = data only, stderr = diagnostics**; exactly one dialog type
  per invocation (two or more → exit 255).
- **Light & dark**: follows `cosmic::Theme` (system COSMIC theme).
- **Frosted glass**: window `enable_blur(true)` — behind-window blur like
  native COSMIC dialogs. Blurred frost; no synthetic shadow.
- **Exit contract**: `0` OK/help/`--version`, `1` Cancel/close,
  `5` timeout, `255` error (unrecognized/missing/invalid dialog selection).

## Usage

```bash
cosmic-wicara --question --title="Confirm" --text="Proceed?"
cosmic-wicara --entry --title="Login" --text="Enter name:"         # prints to stdout
cosmic-wicara --file-selection --directory --multiple             # prints paths
cosmic-wicara --progress --pulsate --title="Working"              # reads stdin %
cosmic-wicara --notification --text="Done" --icon=dialog-information
cosmic-wicara --help-question                                     # per-dialog help
man cosmic-wicara                                                 # see man/cosmic-wicara.1
```

Use the same flags across scripts; keep stdout clean for data and write
diagnostics to stderr.

> Do **not** launch a terminal via `cosmic-term -e`. This is a dialog;
> `.desktop` uses `Terminal=false`.

## Build steps

```bash
cargo build --release -p cli      # release binary: LTO + strip (Cargo.toml [profile.release])
just build                       # debug workspace
just test                        # cargo test --workspace
cargo test -p cli                # scaffold proof (must stay green)
just install                     # -> ~/.local/bin/cosmic-wicara
```

Install system-wide (Arch):

```bash
cd packaging && makepkg -sif      # see packaging/PKGBUILD
# installs /usr/bin/cosmic-wicara, /usr/share/man/man1/cosmic-wicara.1,
# /usr/share/applications/cosmic-wicara.desktop
```

Requires `cargo 1.99.1`, Edition 2024, `rust-version = 1.87`.
`clap 4.6.7` (derive) is the CLI boundary; `libcosmic` is git-pinned
at `dd108aa99d7306110eef06da43eafc109e1ab151` (features `about,wgpu,xdg-portal`).

## Workspace

```text
cosmic-wicara/
  Cargo.toml            # [workspace] + [profile.release] lto/strip
  crates/cli/           # package `cli`, binary `cosmic-wicara`
    src/main.rs         # thin: parse -> count -> dispatch
    src/args/           # CLI flags (general + per-dialog, via flatten)
    src/builders/       # per-dialog builders (Cli -> DialogRequest)
  crates/core/          # Dialog/Exit/theme/icons + notify (bus/listen)
  crates/app/           # libcosmic window shell (15 dialogs)
    src/runner/         # config/state/output/views (+file_task): dialog runtime
    src/window/         # shell state + geometry + blur
    src/dialogs/        # one file per dialog
  crates/applet/        # tray applet (poll + tray)
  justfile              # build / test / clippy / fmt
  man/cosmic-wicara.1   # man page
  packaging/PKGBUILD    # Arch packaging (cargo build --release, /usr/bin)
  cosmic-wicara.desktop # Exec=cosmic-wicara Terminal=false Categories
```

## Community

Contributions, bug reports, and COSMIC UX feedback are welcome. Please open
an issue or pull request with:

- the dialog type and exact CLI invocation,
- the COSMIC/libcosmic version and session type (Wayland), and
- the expected vs. actual exit code / stdout / stderr.

For COSMIC-wide design questions, prefer the upstream COSMIC/libcosmic
projects; this repo focuses on dialog behaviour.

## License

GPL-2.0-or-later. See [LICENSE](LICENSE).

## Design

- `.desktop` `Exec=cosmic-wicara`, `Terminal=false`,
  `Categories=Utility;`, `Keywords=dialog;wicara;cosmic;`,
  `Icon=dialog-information`.
- Light/dark follows the COSMIC system theme. Frosted glass is the
  `enable_blur(true)` window hint on every dialog.
- Dialog windows are content-sized floating surfaces: fixed `min == max`
  size, client-side decorations, and focus requested on map.
