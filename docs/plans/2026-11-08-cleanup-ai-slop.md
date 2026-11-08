# Plan: Verified cleanup of AI slop, redundancy, and logic bugs in cosmic-wicara

REQUIRED SUB-SKILL: plan-writing (with refactoring-craft, test-first-development, debug-methodically)

Goal: Remove verified duplication and fix verified logic bugs without changing documented CLI behavior.

Architecture: One generic `MessageDialog` in the app crate replaces the four copy-pasted message dialogs behind type aliases, so all existing call sites keep compiling. Logic fixes land as small, individually testable patches. No help-text, trait, or dependency redesign in this round.

Stack: Rust 2024, cargo workspace (cli/core/app/applet), libcosmic pinned dd108aa9.

Spec: User request 2026-11-08 — crosscheck/verify/validate every audit finding, plan, crosscheck the plan, then execute.

Global constraints (verbatim from repo):
- `rust-version = "1.85"`, `resolver = "3"`, edition 2024
- Exit codes: 0 OK/help, 1 Cancel/close, 5 timeout, 255 error; stdout=data, stderr=diagnostics
- Baseline (untouched, verified): `cargo test --workspace` all green (48+5+12+5+4+3+0+6+2+0+9+2+16+1+4 tests), `cargo clippy --workspace` exit 0, `cargo fmt --check` exit 0

## Verification results (crosscheck of prior audit)

CONFIRMED TRUE (evidence):
1. info/error/warning/question.rs are 273/273/273/292 lines; `diff` shows only struct name + DialogIcon + cancel fields differ. ~800 lines redundant.
2. List has paired setters with_/bare with zero external callers of the bare variants (grep verified).
3. window.rs vs core/theme.rs duplicate blur_supported/prefers_reduced_transparency/enable_blur/solid+glass settings.
4. `let _ = prog.no_cancel` (runner.rs:658) — --no-cancel parsed, never hides Cancel. BUG.
5. Scale::snap truncates via div_euclid: snap(8)=5 with step 5 (nearest is 10). Test codifies truncation. BUG (GTK snaps to nearest).
6. glob_match("*.txt","atxt") == true — suffix "txt" matched without dot. BUG.
7. Applet view_window panics; view() unwraps main_window_id. BUG (crash risk).
8. SCAFFOLD consts in core/lib.rs and app/lib.rs have zero references. Dead.
9. CORNER_RADIUS (window.rs) and DIALOG_RADIUS (theme.rs) unused in production. Dead.
10. body_with_countdown_public is a pure wrapper only because runner is in same crate. Redundant.
11. tracing/anyhow/thiserror declared in core/Cargo.toml, tracing in applet/Cargo.toml, zero code usage. Dead deps.
12. today() in cli/main.rs hand-rolls Gregorian conversion; jiff already used in runner.rs. Duplication (algorithm itself is correct — verified, not a bug).
13. List validate() checks rows multiple-of-columns, but cli never calls it before runner; runner view_list truncates silently. GAP.

REJECTED / OVERSTATED (do NOT fix this round):
- Calendar format_with passthrough of unknown %Q: intentional, tested. Not a bug.
- Forms List value() returning first value: ambiguous spec, no failing evidence. Defer.
- kill_parent via /proc+kill: works on Linux, fragile but no reproduced failure. Defer (would need libc dep).
- Help duplication (main print_* vs clap): real but help_snapshot tests pin exact strings; redesign is its own task. Defer, noted as follow-up.
- Dialog::show() stub: trait is misleading for message dialogs but Notification/Progress use it correctly; deleting trait is breaking. This round only removes the stub body from the unified MessageDialog and documents Runner as the real path.

## Review Focus (five untested input classes)
1. MessageDialog with timeout=Some(0) — currently immediate timeout; pin current behavior in a test, do not change semantics this round.
2. snap() with negative values below min — nearest-rounding must clamp, not wrap.
3. glob "*.txt" against ".txt" and "a.TXT" (case) — pin current: ".txt" matches, case-sensitive.
4. Applet popup before main_window_id exists — must not panic/unwrap.
5. List with rows not multiple of columns via CLI — must exit 255, not silently truncate.

## File map
- Create `crates/app/src/dialogs/message.rs` — MessageDialog + aliases Info/Error/Warning/Question.
- Modify info/error/warning/question.rs — become re-export shims (keep module paths).
- Modify scale.rs, file_selection.rs, runner.rs, applet/main.rs, window.rs, cli/main.rs, list.rs, core/lib.rs, app/lib.rs, core/Cargo.toml, applet/Cargo.toml.

## Tasks

### Task 1 — Unify message dialogs (refactor, behavior-preserving)
- [ ] RED: existing dialog tests pass before change (`cargo test -p app info:: error:: warning:: question::` — record green).
- [ ] Create message.rs with `pub struct MessageDialog { kind: DialogIcon, title, text, ok_label, cancel_label: Option<String>, default_cancel, ellipsize, icon, width, timeout }`, constructors `MessageDialog::new(kind,title,text)`, methods new/with_ok_label/with_cancel_label/with_ellipsize/with_icon/with_width/with_timeout/icon_named/icon_name/apply_ellipsize/visible_text/ellipsize_mode/display_text/view/view_with_theme`, `impl Dialog` delegating to build_window+view (same stub semantics, documented).
- [ ] info.rs/error.rs/warning.rs/question.rs become `pub use super::message::{MessageDialog as Info, ...}` plus Question-specific? No — Question needs cancel_label: MessageDialog::new(DialogIcon::Question,..) sets cancel_label=Some(DEFAULT_CANCEL_LABEL). Aliases preserve all existing tests.
- [ ] GREEN: `cargo test --workspace` all pass; `cargo clippy --workspace` 0; `cargo fmt --check` 0.

### Task 2 — Fix --no-cancel (bug, test-first)
- [ ] RED: add test in runner tests or set3.rs asserting a Progress config with no_cancel=true is distinguishable (expose `fn progress_hides_cancel(&self)->bool` or check view_input branch). Run, expect FAIL.
- [ ] GREEN: in view_input, omit secondary_action when request is Progress && prog.no_cancel. Remove `let _ = prog.no_cancel`. Run, expect PASS + full suite.

### Task 3 — Fix Scale::snap to nearest (bug, test-first)
- [ ] RED: change test expectation snap(8)==10, add negative case. Run, expect FAIL (currently 5).
- [ ] GREEN: `((offset + step/2).div_euclid(step))*step + min`, clamped. Run, expect PASS + full suite. Update the one codified test.

### Task 4 — Fix glob "*.txt" dot bug (bug, test-first)
- [ ] RED: add assert !glob_match("*.txt","atxt"). Run, expect FAIL.
- [ ] GREEN: require name ends with ".txt" and name.len() > 4 (chars). Run, expect PASS + full suite.

### Task 5 — Applet crash fixes (bug, test-first)
- [ ] RED: add test that view_window does not panic (call via a test harness or extract `popup_content` path). Expect FAIL/panic.
- [ ] GREEN: view_window returns empty text element; replace unwrap() with fallback Id::unique() or early return. Run, expect PASS.

### Task 6 — Dead code & deps cleanup (refactor, evidence-based)
- [ ] Remove SCAFFOLD from core/lib.rs + app/lib.rs (grep evidence: zero refs).
- [ ] Remove CORNER_RADIUS/DIALOG_RADIUS (grep evidence: zero prod refs); keep button_bg_color (public API, defer).
- [ ] Remove unused tracing/anyhow/thiserror from core/Cargo.toml, tracing from applet/Cargo.toml.
- [ ] Remove body_with_countdown_public; make body_with_countdown pub(crate); update runner call.
- [ ] Delete bare List setters (title/text/checklist/radiolist/imagelist/editable/print_column/hide_column/hide_header/separator) — grep evidence: zero callers.
- [ ] Replace today() with jiff::Zoned::now().date() in cli/main.rs; add jiff dep to cli if missing (check Cargo.toml first).
- [ ] Add List validate() call in cli show_dialog List branch (Review Focus 5).
- [ ] GREEN: full suite + clippy + fmt.

## Self-review (author, all five points)
- Spec coverage: PASS — every CONFIRMED item maps to a task; REJECTED items explicitly deferred with reason.
- Step scan: PASS after crosscheck — each task has RED/GREEN commands with expected outcomes; no TBD.
- Type consistency: PASS — MessageDialog fields/methods named once, aliases preserve Info/Error/Warning/Question spelling used by cli/main.rs and tests.
- Review Focus: PASS — five classes above, each pinned by a test added in its owning task (Timeout 0 in Task 1 tests, snap negative in Task 3, glob case in Task 4, applet in Task 5, List validate in Task 6).
- Proportion: PASS — Task 1 is largest but is the core redundancy; Tasks 2-6 are small single-bug cycles.

## Crosscheck of this plan (second pass)
- Verified today() replacement: cli/Cargo.toml — must check jiff present; if absent add `jiff.workspace`? jiff is not in workspace deps, runner uses its own. So add jiff to cli Cargo.toml directly (version used by app). Checked during Task 6.
- Verified aliases compile: cli/main.rs imports About/Calendar/... but NOT Info/Error/Warning/Question (uses DialogRequest::Message), so alias breakage risk is only in dialogs/mod.rs + tests. Safe.
- Verified Question cancel difference: Question has cancel_label+default_cancel extra fields; MessageDialog covers both via Option. view_with_theme must render secondary when cancel_label is Some (copy Question's branch).
- Risk: deleting bare List setters breaks external users of the library — acceptable for 0.1.0 workspace-internal crate; noted.

## Execution record (2026-11-08, second round — deferred items)

1. **Forms List**: `value()` joins all values with `|`, `set_value()` splits with `split_values` (round-trip). Test `list_value_round_trips_all_values`. Man page documents `label=` + `|`-joined values.
2. **kill_parent/libc**: added `libc = "0.2"` to app; `parent_pid()` via `libc::getppid()` captured in `Runner::new`; `kill_parent(ppid)` uses `libc::kill(ppid, SIGTERM)`, Unix-only, explicit stderr on non-Unix. Test `parent_pid_is_sane`. Man page notes Unix-only.
3. **Trait Dialog removed (Opsi A)**: deleted `pub trait Dialog` from core; stub `show()` impls deleted (validate remains); `Notification::run()` and `Progress::run()` are inherent methods; CLI calls `dialog.run()`; `core/tests/dialog_trait.rs` rewritten to exit-mapping only; tests in about/smoke/set3 updated to not call stub `show()`.
4. **Help single source**: deleted ~240 lines of manual `print_*_help` in cli/main.rs (689→424 lines); all `--help-*` flags now render `Cli::command().print_long_help()` from args.rs headings via new `Cli::wants_help()`. help_snapshot 9/9 pass unchanged.

Final verification: `cargo test --workspace` 18/18 binaries ok, `cargo clippy --workspace --all-targets` clean, `cargo fmt --check` ok.
