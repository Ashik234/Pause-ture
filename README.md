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

Gated breaks complete themselves when the countdown ends — the popup fades out
on its own, so a finished eye break needs no click at all.

### Keyboard

| Key | Action |
| --- | --- |
| `Enter` / `Space` | Done — once the countdown has finished |
| `Esc` | Snooze 5 minutes — works during the countdown too |

Keys are ignored while a countdown is running and for a moment after it opens,
so a keystroke already in flight can't dismiss the break you just earned.

## Features

- **Fullscreen takeover** — translucent overlay on top of everything; your work stays visible behind it
- **Countdown-gated dismissal** — no reflex-clicking past your eye break
- **Merge logic** — reminders due within 5 minutes of each other combine into one popup
- **Snooze 5 min** — escape hatch for calls and meetings, usable even mid-countdown
- **Pause 1 hour** — from the tray, for deep-work blocks
- **Fact of the break** — every popup ends with something worth knowing (see below)
- **Autostart** — launches with Windows (toggleable)
- ~4 MB installer, near-zero idle footprint

### Guards — it stays quiet when interrupting would be wrong

| Guard | Behaviour |
| --- | --- |
| **Idle** | Away from the keyboard 5+ minutes? Reminders reschedule — you were already resting |
| **Lock** | Timers freeze while the workstation is locked, and your locked time counts as rest |
| **Meeting** | Calls and presentations hold reminders back |
| **Typing** | Mid-sentence? The popup waits for a gap, capped at 5 minutes |
| **Work hours** | Off by default. Set a daily window and which weekdays count, and the app stays silent outside them — overnight windows like 22:00–06:00 work too |

### Settings

Per-reminder intervals and on/off toggles, chime on/off, autostart, the quip
category picker and the work-hours window — all persisted to disk. The settings
window also shows today's done-vs-snoozed counts and how long your screen was
locked, plus quick actions to break now, pause, or resume.

## Fact of the break

Each popup closes with one fact, drawn from whichever categories you've enabled.
Nine bundled categories ship ~460 hand-written entries offline:

🎭 Jokes · 🩺 Health & body · 🚀 Science & space · 💻 Tech & programming ·
🐾 Animals & nature · 🏛️ History · 🧠 Mind & psychology · 🌊 Oceans · 🔢 Mathematics

Two more are fetched live from Wikimedia's free APIs at popup time:

- 📅 **On this day** — a real event from today's date, via the Wikipedia feed
- 📖 **Word of the day** — today's word and its first sense, via Wiktionary

Live categories time out after 3 seconds and fall back to a bundled fact, so the
popup works the same offline.

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
  reminder.ts         fullscreen popup: countdown gate, done/snooze, fact
  settings.ts         settings form, stats, quick actions
  quips/              fact categories: 9 bundled data files + category API
src-tauri/src/
  scheduler.rs        30s tick loop, due/merge/idle/pause logic
  guard.rs            lock, meeting and idle detection
  typing.rs           holds the popup while you're mid-sentence
  workhours.rs        daily/weekday window, freezes timers off the clock
  stats.rs            daily done/snoozed and screen-locked totals
  popup.rs            fullscreen always-on-top popup window
  commands.rs         complete, snooze, get/save settings
  settings.rs         persisted settings + autostart
  lib.rs              tray, window close-blocking, wiring
```

## Releasing

Push a tag → GitHub Actions builds Windows, macOS and Linux installers and
publishes them to a GitHub release. Shipped apps auto-update from it.

```bash
git tag v2.1.0
git push origin v2.1.0
```

## Known limitations

- `always_on_top` covers normal apps and borderless-windowed games, but not exclusive-fullscreen games
- Built and tested on Windows; CI also produces macOS and Linux bundles, which are unsigned and less exercised
- Installers are unsigned, so first-run OS warnings are expected
