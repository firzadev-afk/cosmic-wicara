# cosmic-wicara

> A native dialog tool for the **COSMIC** desktop — written in Rust + `libcosmic`, one binary `cosmic-wicara` for shell scripts, installers, and desktop workflows on COSMIC.

**COSMIC-only, Wayland-first.** Built for a COSMIC session. Not a GTK app, not portable to other desktop environments (GNOME/KDE/XFCE/…), and not for X11.

![License](https://img.shields.io/badge/license-GPL--2.0--or--later-blue) ![Rust](https://img.shields.io/badge/rust-1.87%2B-orange) ![COSMIC](https://img.shields.io/badge/COSMIC-native-informational)

---

## Table of Contents

- [What Is cosmic-wicara?](#what-is-cosmic-wicara)
- [What Is It For?](#what-is-it-for)
- [When to Use It / When Not To](#when-to-use-it--when-not-to)
- [Key Features](#key-features)
- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [CLI Contract: stdout, stderr & Exit Codes](#cli-contract-stdout-stderr--exit-codes)
- [Dialog Types](#dialog-types)
- [General Options](#general-options)
- [Project Structure](#project-structure)
- [Build & Development](#build--development)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [License](#license)

---

## What Is cosmic-wicara?

**cosmic-wicara** is a command-line *dialog tool* that shows native COSMIC dialog windows from a terminal or shell script, then returns the user's choice through `stdout` and an *exit code*.

Put simply: it is to COSMIC what `zenity` / `yad` are to GTK — but native, built directly on `libcosmic`. No GTK, no web view, no wrapper.

You call it from a script:

```bash
cosmic-wicara --question --title="Confirm" --text="Continue?"
```

COSMIC shows the dialog window. The user's answer (OK / Cancel / typed text / selected file / slider value / …) is returned to the script as *text on stdout* and an *exit code*.

---

## What Is It For?

cosmic-wicara bridges **shell scripts ↔ the COSMIC desktop user**. Its main purposes:

| Purpose | Real-world example |
|---|---|
| **Confirmation** | An installer script asks *"Install now?"*; a backup script asks *"Overwrite the old file?"* |
| **Text input** | Ask for a file name, user name, password, or a short answer without opening an interactive terminal |
| **Pick a file / folder** | Installers and utility scripts let the user choose a file or folder through the native COSMIC file chooser (via `xdg-portal`) |
| **Pick a date, color, or number** | Simple forms: calendar, color picker, numeric slider (e.g. choose a size or level) |
| **Show info / error / warning** | Script finishes → show info; script fails → show an error with the right icon |
| **Progress bar** | Long-running work (copy, render, download) shows progress updated from `stdin` |
| **Notifications** | A background script tells the user *"Done"* through a COSMIC notification bubble |
| **Show text / license** | Display a text file, README, or license, with an optional *"I have read and agree"* checkbox |
| **Forms** | A single dialog with several fields at once (entry, password, calendar, list, combo) |
| **Desktop automation** | Shortcuts, launchers, `.desktop` files, or startup scripts that need visual interaction |

**In short:** whenever a script needs to *ask, request input, inform, or ask the user to choose* something — without writing a GUI of its own — use cosmic-wicara.

### What cosmic-wicara is NOT

- Not a standalone GUI application (it is always called from a script or the CLI).
- Not a GTK/Qt replacement on other desktops — COSMIC only.
- Not a terminal TUI tool — its dialogs are COSMIC windows.
- Not a library for other applications — it is a CLI binary (`cosmic-wicara`).

---

## When to Use It / When Not To

**Use cosmic-wicara when:**

- You write shell scripts or installers on COSMIC and need a visual dialog.
- You want the dialog result returned to your script (`stdout` + exit code).
- You want a truly native COSMIC look (light/dark follows the system, frosted blur, COSMIC decorations).

**Do not use it when:**

- You are on GNOME/KDE/XFCE — use `zenity` / `yad` / that desktop's native engine instead.
- You need X11 — cosmic-wicara is Wayland-first for COSMIC sessions.
- You need a dialog that runs without a display server (headless).

---

## Key Features

- **16 dialog choices** — 15 interactive dialogs plus an `about` dialog (see [Dialog Types](#dialog-types)).
- **Real widgets per dialog** — not static images:
  - File chooser via `xdg-portal` (native COSMIC file picker).
  - Calendar picker, slider, color picker.
  - List with checklist / radio / editable modes.
  - Text viewer with `--checkbox`, progress bar fed from `stdin`.
- **Flexible general options** — title, size, timeout, button labels, extra buttons, modal hint, icons, language (`--lang=en|id`), `--json` output, and more.
- **Clean CLI contract:**
  - `stdout` = dialog result data only (text, file paths, numbers, …).
  - `stderr` = diagnostics / error messages.
  - Exactly **one** dialog type per invocation (two or more → exit `255`).
- **Follows the COSMIC theme** — light and dark follow `cosmic::Theme` (the COSMIC system theme).
- **Frosted glass** — the window uses `enable_blur(true)`, blurring behind the window like native COSMIC dialogs. No synthetic shadow.
- **Floating, content-sized windows** — not a tiled application window, no compositor header, no double title.
- **Clear exit codes** — `0` OK/help/`--version`, `1` Cancel/close, `5` timeout, `255` error (full table below).

---

## Requirements

- A system running a **COSMIC** session (Wayland).
- To build from source:
  - `cargo 1.99.1` or newer, Rust Edition 2024, `rust-version = 1.87`.
  - `just` (optional, for the shortcuts in `justfile`).
  - `libcosmic` git-pinned at `dd108aa99d7306110eef06da43eafc109e1ab151` (features `about,wgpu,xdg-portal`) — already locked in `Cargo.lock`, no manual install needed.
  - `clap 4.6.7` (derive) as the CLI boundary.

---

## Installation

### A. Local install (user, no root)

```bash
cargo build --release -p cli      # release binary: LTO + strip
just install                      # -> ~/.local/bin/cosmic-wicara
```

Or manually:

```bash
install -Dm755 target/release/cosmic-wicara "$HOME/.local/bin/cosmic-wicara"
```

Make sure `~/.local/bin` is on your `PATH`.

### B. System-wide install (Arch Linux)

```bash
cd packaging && makepkg -sif      # see packaging/PKGBUILD
```

This installs:

- `/usr/bin/cosmic-wicara`
- `/usr/share/man/man1/cosmic-wicara.1`
- `/usr/share/applications/cosmic-wicara.desktop`

### C. Verify the install

```bash
cosmic-wicara --version
cosmic-wicara --help
man cosmic-wicara
```

---

## Usage

### General syntax

```bash
cosmic-wicara <one-dialog-type> [general-options] [dialog-specific-options]
```

Important rules:

1. **Exactly one** `--dialog-type` per invocation (e.g. `--question`, `--entry`, `--info`).
2. **No dialog type** → error, exit `255`, message on `stderr`.
3. **Two or more dialog types** → error, exit `255`.
4. Result data goes to `stdout`. Diagnostics go to `stderr`.
5. Use `--help-<dialog>` for dialog-specific help, e.g. `cosmic-wicara --help-question`.
6. Use `--help-general` / `--help` / `--help-all` for general help.

> Do **not** launch this dialog from `cosmic-term -e`. It is a dialog, not a terminal — its `.desktop` file uses `Terminal=false` and `Exec=cosmic-wicara`.

### Examples

```bash
# Ask for confirmation — exit 0 on OK, 1 on Cancel
cosmic-wicara --question --title="Confirm" --text="Continue?"

# Request text input — the typed text is printed to stdout
cosmic-wicara --entry --title="Login" --text="Enter your name:"

# Pick folders (multiple allowed) — paths are printed, one per line
cosmic-wicara --file-selection --directory --multiple

# Progress bar — reads percentages from stdin
cosmic-wicara --progress --pulsate --title="Working"

# Completion notification
cosmic-wicara --notification --text="Done" --icon=dialog-information

# Per-dialog help
cosmic-wicara --help-question

# Full manual
man cosmic-wicara
```

### Common shell script pattern

```bash
#!/usr/bin/env bash

if cosmic-wicara --question --title="Backup" --text="Back up now?"; then
  echo "OK pressed"
else
  code=$?
  case $code in
    1) echo "User cancelled" ;;
    5) echo "Timed out" ;;
    255) echo "Error: dialog could not be shown" ;;
  esac
fi

# Capture user input from stdout
name=$(cosmic-wicara --entry --title="Name" --text="Your name:")
echo "Hello, $name"
```

Use the same flags across your scripts; keep `stdout` clean for data and write diagnostics to `stderr`.

---

## CLI Contract: stdout, stderr & Exit Codes

### stdout vs stderr

| Stream | Contents | Purpose |
|---|---|---|
| `stdout` | **Dialog result data only** — typed text, file paths, slider values, dates, … | Captured by scripts (`$(...)`, pipes, …) |
| `stderr` | Diagnostics, error messages, help errors | Read by humans, never pollutes data |

### Exit codes

| Code | Meaning | When it happens |
|---|---|---|
| `0` | OK / help / `--version` | The user pressed OK, or you called `--help` / `--version` |
| `1` | Cancel / window closed | The user pressed Cancel, closed the window, or the dialog was dismissed |
| `5` | Timeout | `--timeout` expired before the user answered |
| `255` | Error | Unknown / missing / invalid dialog option, or more than one dialog type |

### Output and language options

| Option | Effect |
|---|---|
| `--json` | Print the dialog result as JSON (machine-readable) |
| `--lang=en\|id` | Language for built-in labels and text (`en` = English, `id` = Indonesian) |

---

## Dialog Types

Exactly one of these per invocation. Use `--help-<dialog-name>` for the full option list, or `man cosmic-wicara`.

| # | Flag | Dialog | When to use |
|---|---|---|---|
| 1 | `--calendar` | Calendar | Pick a date (day/month/year; output format via `--date-format`) |
| 2 | `--entry` | Text entry | Ask the user for a short string (initial text via `--entry-text`, hide it via `--hide-text`) |
| 3 | `--error` | Error | Show an error message (icon, wrapping, markup, ellipsize) |
| 4 | `--info` | Info | Show an informational message |
| 5 | `--file-selection` | File/folder picker | Native COSMIC file chooser via `xdg-portal` (multiple, directory, save, separator, filters) |
| 6 | `--list` | List | Choose from a list (checklist, radiolist, imagelist, editable, print-column) |
| 7 | `--notification` | Notification | COSMIC notification bubble (icon, `--listen` reads commands from stdin) |
| 8 | `--progress` | Progress bar | Progress bar updated from stdin (`--pulsate`, `--auto-close`, `--no-cancel`, …) |
| 9 | `--question` | Question | Ask OK/Cancel (icon, default-cancel, switch) |
| 10 | `--warning` | Warning | Show a warning |
| 11 | `--scale` | Numeric slider | Pick a number (value, min/max, step, hide-value) |
| 12 | `--text-info` | Text info | Show a file or text (editable, font, agreement checkbox) |
| 13 | `--color-selection` | Color picker | Pick a color (initial color, palette) |
| 14 | `--password` | Password | Password input (`--username` adds a user name field) |
| 15 | `--forms` | Forms | A single dialog with many fields (entry, password, calendar, list, combo, …) |
| 16 | `--about` | About | Show information about cosmic-wicara |

**Examples by group:**

```bash
# Messages: info / error / warning / question
cosmic-wicara --info --title="Info" --text="Backup complete."
cosmic-wicara --error --text="Failed to write the file."
cosmic-wicara --question --text="Continue?" --default-cancel

# Input: entry / password / scale / calendar / color
cosmic-wicara --entry --text="Name:" --entry-text="Alice"
cosmic-wicara --password --username
cosmic-wicara --scale --min-value=0 --max-value=100 --value=50
cosmic-wicara --calendar --date-format="%Y-%m-%d"
cosmic-wicara --color-selection --show-palette

# Selecting: file / list / forms
cosmic-wicara --file-selection --filename="$HOME" --directory
cosmic-wicara --list --column="Name" --column="Size" --checklist
cosmic-wicara --forms --add-entry="Name" --add-password="Password"

# Displaying: text-info / progress / notification
cosmic-wicara --text-info --filename=LICENSE --checkbox="I have read and agree"
echo 50 | cosmic-wicara --progress --title="Copying"
cosmic-wicara --notification --text="Done" --icon=dialog-information
```

---

## General Options

These options apply to every dialog where relevant. See `cosmic-wicara --help-general` for the full list.

| Option | Meaning |
|---|---|
| `--title` | Dialog window title |
| `--width` | Window width (`0` = default 400px) |
| `--height` | Window height (`0` = fit the content) |
| `--timeout` | Timeout in seconds (exit `5` on timeout) |
| `--ok-label` | OK button label (default: `OK`) |
| `--cancel-label` | Cancel button label (default: `Cancel`) |
| `--extra-button` | Add an extra button (may be repeated) |
| `--modal` | Set the modal hint |
| `--attach` | *Deprecated, no-op* — kept for script compatibility |
| `--json` | Print the result as JSON |
| `--lang=en\|id` | Dialog language (`en` / `id`) |
| `--window-icon` | Window icon |
| `--icon` | Per-dialog icon |
| `--ellipsize` | Enable text ellipsizing |
| `--no-wrap` | Disable text wrapping |
| `--switch` | Switch mode (suppress OK/Cancel) |
| `--default-cancel` | Give the Cancel button initial focus |

**Help & version:**

| Option | Meaning |
|---|---|
| `--help`, `-h` | Show help for all dialog types |
| `--help-general` | Help for general options |
| `--help-all` | All help options |
| `--help-<dialog>` | Help for one dialog, e.g. `--help-question`, `--help-file-selection` |
| `--version` | Show version information |

---

## Project Structure

```text
cosmic-wicara/
├── Cargo.toml              # [workspace] + [profile.release] lto/strip
├── README.md               # main documentation (this file)
├── CHANGELOG.md            # change history
├── LICENSE                 # GPL-2.0-or-later
├── justfile                # build/test/clippy/fmt shortcuts
├── cosmic-wicara.desktop   # Exec=cosmic-wicara, Terminal=false, Categories=Utility
├── man/
│   └── cosmic-wicara.1     # man page
├── packaging/
│   └── PKGBUILD            # Arch packaging (cargo build --release)
└── crates/
    ├── cli/                # package `cli`, binary `cosmic-wicara`
    │   ├── src/main.rs     # thin entry: parse -> count -> dispatch
    │   ├── src/args/       # CLI flags (general + per-dialog, via flatten)
    │   ├── src/builders/   # per-dialog builders (Cli -> DialogRequest)
    │   └── tests/          # CLI tests (headless + exit codes)
    ├── core/               # Dialog/Exit/theme/icons + notify (bus/listen)
    ├── app/                # libcosmic window shell (all dialogs)
        ├── src/runner/     # config/state/output/views (+file_task): dialog runtime
        ├── src/window/     # shell state + geometry + blur
        └── src/dialogs/    # one file per dialog
```

### How it works (data flow)

```text
Script (bash/installer)
        │
        ▼   cosmic-wicara --question --text="Continue?"
crates/cli (main.rs)
        │   parse clap -> count dialogs (0/1/>1) -> dispatch
        ▼
crates/cli builders/          # turn CLI flags into a DialogRequest
        │
        ▼
crates/app runner/            # run the libcosmic COSMIC window
        │   (theme, blur, geometry, focus, timeout)
        ▼
COSMIC dialog appears ──► user clicks / types / selects
        │
        ▼
stdout = result data      stderr = diagnostics
exit 0 / 1 / 5 / 255
        │
        ▼
Script reads the result and exit code
```

---

## Build & Development

| Command | Purpose |
|---|---|
| `cargo build --release -p cli` | Release build (`LTO + strip` from `[profile.release]`) |
| `just build` | Debug build of the whole workspace |
| `just build-release` | Same as `cargo build --release -p cli` |
| `just test` | `cargo test --workspace` |
| `cargo test -p cli` | CLI tests only (scaffold proof, must stay green) |
| `just test-cli` | Same as above |
| `just install` | Install the release binary to `~/.local/bin/cosmic-wicara` |
| `just fmt` | `cargo fmt --check` |
| `just clippy` | `cargo clippy --workspace -- -D warnings` |
| `just clean` | `cargo clean` |
| `just list` | List all `just` recipes |

### Current status

- Builds with `cargo build --release -p cli` (LTO + strip).
- Workspace tests, clippy, and fmt are green in CI.
- Dialog windows float and size to their content: no tiled application window, no compositor header, no double title.
- **Recommended:** do a visual check inside a live COSMIC session before a release. Headless validation (exit `255` on bad input) is covered by tests.

### Design notes

- `.desktop`: `Exec=cosmic-wicara`, `Terminal=false`, `Categories=Utility;`, `Keywords=dialog;wicara;cosmic;`, `Icon=dialog-information`.
- Light/dark follows the COSMIC system theme.
- Frosted glass is the `enable_blur(true)` window hint on every dialog.
- Dialog windows are floating, content-sized surfaces: fixed `min == max` size, client-side decorations, and focus requested on map.

---

## Troubleshooting

| Symptom | Likely cause | Fix |
|---|---|---|
| `You must specify a dialog type` (exit `255`) | No `--dialog-type` given | Add one, e.g. `--question` |
| `Two or more dialog options specified` (exit `255`) | More than one dialog type | Choose exactly one dialog type |
| Unknown option (exit `255`) | Typo or unsupported flag | Check `--help` or `--help-<dialog>` |
| No dialog appears at all | Not in a COSMIC session / no display | Run inside a COSMIC session (Wayland) |
| Repeated timeout (exit `5`) | `--timeout` too short | Increase `--timeout` or drop it |
| File picker empty / fails | `xdg-portal` / COSMIC portal unavailable | Make sure the COSMIC portal is active in your session |
| Script output is "dirty" | Diagnostics mixed into `stdout` | Separate them: data on `stdout`, diagnostics on `stderr` |
| Binary not found after `just install` | `~/.local/bin` not on `PATH` | Add it to `PATH` or use the absolute path |
| Want to see every option | — | `cosmic-wicara --help-all` or `man cosmic-wicara` |

---

## Contributing

Contributions, bug reports, and COSMIC UX feedback are welcome. When opening an issue or pull request, please include:

- The dialog type and the exact CLI invocation.
- The COSMIC/`libcosmic` version and session type (Wayland).
- The expected vs. actual exit code / `stdout` / `stderr`.

For COSMIC-wide design questions, prefer the upstream COSMIC/`libcosmic` projects; this repo focuses on dialog behaviour.

---

## License

GPL-2.0-or-later. See [LICENSE](LICENSE).

---

*Built for the COSMIC desktop — native, lightweight, and easy to call from shell scripts.*
