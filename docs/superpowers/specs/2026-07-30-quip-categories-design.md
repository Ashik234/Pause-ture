# Quip Categories — Design

**Date:** 2026-07-30
**Status:** Approved

## Goal

Grow the quip pool from 34 entries to ~350 across 7 categories, and let the user pick which categories appear on break popups.

## Categories

| Key | Label | Icon | Content |
|-----|-------|------|---------|
| `jokes` | Jokes | 🎭 | Programmer/office humor (existing 18, grow to ~50) |
| `health` | Health & body | 🩺 | Eyes, posture, hydration, movement (existing 16, grow to ~50) |
| `science` | Science & space | 🚀 | Physics, astronomy, chemistry (~50 new) |
| `tech` | Tech & programming | 💻 | Computing history, internet trivia (~50 new) |
| `animals` | Animals & nature | 🐾 | Animal and nature curiosities (~50 new) |
| `history` | History | 🏛️ | Historical facts (~50 new) |
| `mind` | Mind & psychology | 🧠 | Brain, memory, focus facts (~50 new) |

All facts: well-known, verifiable, one-liner style matching the existing tone. No obscure or contested claims.

## Data layer

- New folder `src/quips/` replaces `src/quips.ts`:
  - One file per category (`jokes.ts`, `health.ts`, `science.ts`, `tech.ts`, `animals.ts`, `history.ts`, `mind.ts`), each exporting `string[]`.
  - `index.ts` exports:
    - `type Category = "jokes" | "health" | "science" | "tech" | "animals" | "history" | "mind"`
    - `CATEGORIES: Record<Category, { icon: string; label: string; entries: string[] }>`
    - `randomQuip(enabled: Category[]): { category: Category; text: string }` — flattens enabled pools and picks uniformly, so bigger pools weigh proportionally. Falls back to all categories if `enabled` is empty or contains no known keys.

## Settings schema

Rust `Settings` struct ([settings.rs](../../../src-tauri/src/settings.rs)):

- Keep `quips: bool` as the master toggle (unchanged, backward compatible).
- Add `quip_categories: Vec<String>` with `#[serde(default = "all_categories")]` returning all 7 keys. Old `settings.json` files load fine — the serde default fills the field.
- No new Tauri commands: `get_settings`/`save_settings` already round-trip the whole struct.

TS `Settings` type in [settings.ts](../../../src/settings.ts) gains `quip_categories: string[]`.

## Settings UI

- Chip row rendered under the 🎭 "Joke of the break" toggle row.
- Chip = small pill button per category (icon + label), filled when active, outline when off.
- Chip row hidden while the master toggle is off; reappears with prior selection when re-enabled.
- Guard: clicking the last active chip is a no-op — at least one category stays selected while the master toggle exists.
- Save writes the active chip keys into `quip_categories`.

## Popup

[reminder.ts](../../../src/reminder.ts) `showQuip()`:

- Calls `randomQuip(prefs.quip_categories)`.
- Prefix icon comes from `CATEGORIES[category].icon` instead of the joke/fact ternary.

## Error handling

- Unknown category keys in stored settings (e.g. after a future rename) are ignored by `randomQuip`'s filter; if nothing survives, fall back to all categories rather than showing no quip.
- Empty `quip_categories` array treated the same way.

## Testing

No test infrastructure in the repo. Manual verification via dev run:

1. Settings open → chips render, reflect saved state.
2. Toggle master off → chips hide; on → reappear.
3. Deselect all but one chip → last chip refuses to turn off.
4. Save → reopen settings → selection persists.
5. Popup fires → quip only from selected categories, correct icon.
6. Delete `quip_categories` from `settings.json` → app loads with all categories on.

## Out of scope

- Remote/updatable fact feeds.
- Per-category frequency weighting controls.
- User-authored custom facts.
