<p align="center">
  <img src="icon.png" width="110" alt="Pause-ture logo" />
</p>

<h1 align="center">Pause-ture</h1>

<p align="center">
  Break reminders for Windows that actually stop you.<br />
  Rest your eyes, fix your posture, drink water, take a walk.
</p>

---

## What it does

Pause-ture lives in your system tray and interrupts you with a fullscreen, always-on-top popup when it's time for a break. The popup can't be Alt+F4'd away — eye and posture breaks are gated behind a short countdown, so you actually take them.

| Reminder | Default interval | Dismiss rule |
| --- | --- | --- |
| 👀 Look away (20-20-20) | 20 min | Unlocks after a 20s countdown |
| 🪑 Posture check | 30 min | Unlocks after a 10s countdown |
| 💧 Drink water | 45 min | Instant Done ✓ |
| 🚶 Take a walk | 60 min | Instant Done ✓ |

## Features

- **Fullscreen takeover** — translucent overlay on top of everything; your work stays visible behind it
- **Countdown-gated dismissal** — no reflex-clicking past your eye break
- **Merge logic** — reminders due within 5 minutes of each other combine into one popup
- **Snooze 5 min** — escape hatch for calls and meetings, usable even mid-countdown
- **Pause 1 hour** — from the tray, for deep-work blocks
- **Idle detection** — away from the keyboard 5+ minutes? Reminders reschedule; you were already resting
- **Settings** — per-reminder intervals and toggles, persisted to disk
- **Autostart** — launches with Windows (toggleable)
- ~4 MB installer, near-zero idle footprint

## Install

Grab the latest `Pause-ture_x64-setup.exe` from [Releases](../../releases).

> Unsigned build — Windows SmartScreen will warn on first run.
> Click **More info → Run anyway**.

## Development

Prerequisites: [Rust](https://rustup.rs), Node.js, MSVC Build Tools (Desktop development with C++).

```bash
npm install
npm run tauri dev     # dev build: short intervals (1-4 min) so reminders fire fast
npm run tauri build   # release installers → src-tauri/target/release/bundle/
```

Dev builds use compressed timings so everything is observable in minutes: 1/2/3/4-minute intervals, 1-minute snooze, 2-minute pause, 60-second idle threshold. Release builds use the real values above.

### Stack

- [Tauri v2](https://tauri.app) — Rust backend, WebView2 frontend
- Vanilla TypeScript + Vite, no frontend framework
- `tauri-plugin-store` (settings), `tauri-plugin-autostart`, `user-idle`

### Architecture

```
src/                  frontend
  reminder.ts         fullscreen popup: countdown gate, done/snooze
  settings.ts         settings form
src-tauri/src/
  scheduler.rs        30s tick loop, due/merge/idle/pause logic
  popup.rs            fullscreen always-on-top popup window
  commands.rs         complete, snooze, get/save settings
  settings.rs         persisted settings + autostart
  lib.rs              tray, window close-blocking, wiring
```

## Releasing

Push a tag → GitHub Actions builds and attaches installers to a draft release:

```bash
git tag v0.1.0
git push origin v0.1.0
```

## Known limitations

- `always_on_top` covers normal apps and borderless-windowed games, but not exclusive-fullscreen games
- Windows only (for now)
