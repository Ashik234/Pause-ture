# Typing hold — don't interrupt mid-typing

Date: 2026-07-27

## Problem

The reminder popup can land while the user is actively typing (mid-sentence,
mid-test on monkeytype, mid-commit message). That is the most jarring moment
to be interrupted.

## Goal

When a reminder comes due while the user is typing, hold the popup until the
first pause in typing. Never hold forever: after a cap, fire anyway.

- Keyboard only. Mouse activity does not postpone the popup.
- Typing pause = no keystroke for 10 seconds (`TYPING_GAP`).
- Hold cap = 5 minutes past due in release, 60 seconds in debug builds
  (`typing::hold_cap()`, follows the existing `cfg!(debug_assertions)`
  pattern in scheduler.rs).

## Design

### New module `src-tauri/src/typing.rs`

Sibling of `guard.rs`, same shape (Windows impl + non-Windows stubs).

- On startup, spawn a dedicated thread that installs a global low-level
  keyboard hook (`SetWindowsHookExW(WH_KEYBOARD_LL)`) and runs a
  `GetMessageW` message loop (LL hooks require one on the installing thread).
- The hook callback records the timestamp of every `WM_KEYDOWN` /
  `WM_SYSKEYDOWN` in a `static AtomicU64` (millis via `GetTickCount64`).
- `typing_recently() -> bool` — true when the last keystroke landed within
  `TYPING_GAP`.
- If the hook fails to install, log and disable the feature (popup behavior
  falls back to today's).
- macOS/Linux: stubs, `typing_recently()` always false — same convention as
  `guard.rs`.

### Scheduler change (`scheduler.rs`)

New hold rule in the tick loop, after the AFK check and before the popup
fires:

```
due && typing_recently() && max_overdue < hold_cap  →  skip tick (hold)
```

`overdue = now - next_due`. Due reminders already stay due (next_due is only
advanced by complete/snooze), so overdue grows naturally across held ticks —
no new state. Once any due reminder is overdue past the cap, the popup fires
even mid-typing.

### Wiring

- `lib.rs`: `mod typing;` + `typing::spawn_hook()` in setup.
- `Cargo.toml`: add `windows` features `Win32_UI_WindowsAndMessaging`,
  `Win32_System_SystemInformation`. No new crates.

## Rejected alternatives

- `GetAsyncKeyState` polling: "pressed since last call" bit is racy and
  shared per-process; misses keys between polls.
- `rdev` crate: new dependency, heavier, overkill for a Windows-focused app.
- Any-input detection via existing `user_idle`: simplest, but mouse motion
  would postpone the popup — user chose keyboard-only.

## Not in scope

- Settings UI for the gap/cap constants.
- Distinguishing typing app (all keyboard activity counts).
