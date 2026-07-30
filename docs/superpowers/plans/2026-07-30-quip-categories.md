# Quip Categories Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Grow the quip pool to ~350 entries across 7 categories and let users pick which categories appear on break popups.

**Architecture:** Data moves from a single `src/quips.ts` into a `src/quips/` folder (one file per category plus an `index.ts` with the public API). Settings gain a backward-compatible `quip_categories: Vec<String>` field on the Rust side, rendered as a chip row in the settings window. The popup filters by the saved categories.

**Tech Stack:** Tauri v2 (Rust backend), vanilla TypeScript + Vite frontend, no test framework (manual verification per spec).

**Spec:** `docs/superpowers/specs/2026-07-30-quip-categories-design.md`

## Global Constraints

- Category keys, exactly: `jokes`, `health`, `science`, `tech`, `animals`, `history`, `mind`.
- Category icons, exactly: 🎭 🩺 🚀 💻 🐾 🏛️ 🧠 (same order as keys).
- Category labels: Jokes, Health & body, Science & space, Tech & programming, Animals & nature, History, Mind & psychology.
- ~50 entries per category. Every fact must be well-known and verifiable; no contested or obscure claims. One-liner style matching existing entries (see `src/quips.ts` for tone).
- `quips: bool` master toggle stays; old `settings.json` files must keep loading (serde default fills the new field).
- Commit messages: single-line title only, no body (user preference).
- Type-check/build with `npm run build` (runs `tsc && vite build`). Rust check with `cargo check` inside `src-tauri/`.
- There is no test framework in this repo. Do NOT add one. Verification = build passes + manual dev-run checklist in Task 5.

---

### Task 1: Category data files

**Files:**
- Create: `src/quips/jokes.ts`, `src/quips/health.ts`, `src/quips/science.ts`, `src/quips/tech.ts`, `src/quips/animals.ts`, `src/quips/history.ts`, `src/quips/mind.ts`

**Interfaces:**
- Produces: each file default-nothing, single named export `JOKES` / `HEALTH` / `SCIENCE` / `TECH` / `ANIMALS` / `HISTORY` / `MIND` of type `string[]`, ~50 entries each. Task 2's `index.ts` imports these exact names.

- [ ] **Step 1: Create `src/quips/jokes.ts`**

Start from the 18 existing joke strings in `src/quips.ts` (copy the `text` values verbatim, drop the `{ kind, text }` wrappers), then extend to ~50 total in the same style. Example shape and seed entries for the new material:

```ts
export const JOKES: string[] = [
  // ...the 18 existing joke texts from src/quips.ts, verbatim...
  "There's no place like 127.0.0.1.",
  "I changed my password to \"incorrect\", so my computer reminds me when I forget.",
  "Real programmers count from 0.",
  "An SEO expert walks into a bar, bars, pub, tavern, drinks, beer...",
  "Why was the JavaScript developer sad? Because he didn't Node how to Express himself.",
  "Software and cathedrals: first we build them, then we pray.",
  "The two hardest problems in CS: cache invalidation, naming things, and off-by-one errors.",
  "Documentation is like a love letter you write to your future self.",
  "Deleted code is debugged code.",
  "Artificial intelligence is no match for natural stupidity.",
  // ...continue in this style to ~50 total
];
```

- [ ] **Step 2: Create `src/quips/health.ts`**

Start from the 16 existing fact strings in `src/quips.ts` (copy verbatim), extend to ~50 total. All must be body/health/ergonomics facts relevant to desk work. Seed entries for the new material:

```ts
export const HEALTH: string[] = [
  // ...the 16 existing fact texts from src/quips.ts, verbatim...
  "Your eyes make about three saccades per second — roughly 170,000 tiny movements a day.",
  "Deep breathing for 60 seconds measurably lowers heart rate and blood pressure.",
  "Stretching improves blood flow to muscles in under 30 seconds.",
  "Your cornea has no blood supply — it takes oxygen directly from the air.",
  "Laughing increases blood flow by around 20%, similar to light exercise.",
  "Humans are the only animals with a chin.",
  "Your body has about 37 trillion cells, and most are replaced over your lifetime.",
  "Bones are, gram for gram, stronger than steel.",
  "The human eye can distinguish roughly 10 million colors.",
  "Regular short walks improve insulin sensitivity within days.",
  // ...continue in this style to ~50 total
];
```

- [ ] **Step 3: Create `src/quips/science.ts`**

~50 physics/astronomy/chemistry one-liners. Seed entries:

```ts
export const SCIENCE: string[] = [
  "A day on Venus is longer than its year.",
  "Sunlight takes about 8 minutes and 20 seconds to reach Earth.",
  "Neutron stars are so dense a teaspoon would weigh billions of tons.",
  "Hot water can freeze faster than cold water — the Mpemba effect.",
  "There are more stars in the universe than grains of sand on all Earth's beaches.",
  "Honey never spoils — edible honey has been found in 3,000-year-old Egyptian tombs.",
  "Lightning is about five times hotter than the surface of the Sun.",
  "Water expands about 9% when it freezes — that's why ice floats.",
  "The Moon drifts about 3.8 cm farther from Earth every year.",
  "Helium is the only element that can't be frozen solid at normal pressure.",
  // ...continue in this style to ~50 total
];
```

- [ ] **Step 4: Create `src/quips/tech.ts`**

~50 computing-history/internet one-liners. Seed entries:

```ts
export const TECH: string[] = [
  "The first computer bug was an actual moth, found in a Harvard Mark II relay in 1947.",
  "The first 1 GB hard drive (1980) weighed over 200 kg and cost $40,000.",
  "Around 90% of the world's data was created in just the last few years.",
  "The QWERTY layout was designed in the 1870s to reduce typewriter jams.",
  "The first website, info.cern.ch, is still online.",
  "Email predates the World Wide Web by about 20 years.",
  "The Apollo 11 guidance computer had less memory than a modern greeting card that plays music.",
  "CAPTCHA stands for Completely Automated Public Turing test to tell Computers and Humans Apart.",
  "The @ symbol was chosen for email in 1971 because it was unlikely to appear in names.",
  "Wi-Fi doesn't stand for anything — it's a made-up brand name.",
  // ...continue in this style to ~50 total
];
```

- [ ] **Step 5: Create `src/quips/animals.ts`**

~50 animal/nature one-liners. Seed entries:

```ts
export const ANIMALS: string[] = [
  "Octopuses have three hearts and blue blood.",
  "A group of flamingos is called a flamboyance.",
  "Sea otters hold hands while sleeping so they don't drift apart.",
  "Cows have best friends and get stressed when separated.",
  "A shrimp's heart is in its head.",
  "Bananas are berries, but strawberries aren't.",
  "Sharks existed before trees.",
  "Butterflies taste with their feet.",
  "An octopus has nine brains — one central and one per arm.",
  "Trees can communicate and share nutrients through underground fungal networks.",
  // ...continue in this style to ~50 total
];
```

- [ ] **Step 6: Create `src/quips/history.ts`**

~50 history one-liners. Seed entries:

```ts
export const HISTORY: string[] = [
  "Oxford University is older than the Aztec Empire.",
  "Cleopatra lived closer in time to the Moon landing than to the building of the Great Pyramid.",
  "The fax machine was invented in 1843 — the same era as the wagon trail west.",
  "Woolly mammoths were still alive while the Egyptian pyramids were being built.",
  "The shortest war in history, between Britain and Zanzibar in 1896, lasted under 40 minutes.",
  "Ancient Romans used crushed mouse brains as toothpaste.",
  "The Eiffel Tower was meant to be temporary — a 20-year permit.",
  "Nintendo was founded in 1889 — as a playing-card company.",
  "The Great Fire of London in 1666 officially killed only six people.",
  "Ketchup was sold as medicine in the 1830s.",
  // ...continue in this style to ~50 total
];
```

- [ ] **Step 7: Create `src/quips/mind.ts`**

~50 brain/psychology/focus one-liners. Seed entries:

```ts
export const MIND: string[] = [
  "Your brain uses about 20% of your body's energy while being 2% of its weight.",
  "Short breaks during learning improve retention — your brain replays new skills at rest.",
  "The brain can't actually multitask — it switches rapidly, losing time with every switch.",
  "After an interruption it takes about 23 minutes to fully refocus.",
  "Your brain generates roughly 12–25 watts — enough to power a dim LED bulb.",
  "Naps as short as 10 minutes measurably improve alertness.",
  "Handwriting notes improves memory more than typing them.",
  "The 'doorway effect' is real: walking through a doorway makes you forget why you came.",
  "Your brain treats to-do lists as offloaded memory — writing tasks down reduces anxiety.",
  "Dopamine spikes in anticipation of a reward, not just on receiving it.",
  // ...continue in this style to ~50 total
];
```

- [ ] **Step 8: Verify entry counts**

Run from repo root:
```powershell
Get-ChildItem src/quips/*.ts | ForEach-Object { "$($_.Name): $((Select-String '^  "' $_.FullName).Count)" }
```
Expected: each of the 7 files reports ~50 (45–55 acceptable). Note: entries must be one per line starting with two spaces and a quote for this count to work — format them that way.

- [ ] **Step 9: Commit**

```powershell
git add src/quips/
git commit -m "feat(quips): add 7 category data files, ~50 entries each"
```

---

### Task 2: Public API in `index.ts` + popup update

**Files:**
- Create: `src/quips/index.ts`
- Delete: `src/quips.ts`
- Modify: `src/reminder.ts:2` (import stays `"./quips"` — folder index resolves identically), `src/reminder.ts:135-147` (`showQuip` and the `get_settings` call)

**Interfaces:**
- Consumes: `JOKES`/`HEALTH`/`SCIENCE`/`TECH`/`ANIMALS`/`HISTORY`/`MIND` (`string[]`) from Task 1.
- Produces:
  - `type Category = "jokes" | "health" | "science" | "tech" | "animals" | "history" | "mind"`
  - `CATEGORIES: Record<Category, { icon: string; label: string; entries: string[] }>`
  - `ALL_CATEGORIES: Category[]`
  - `randomQuip(enabled?: string[]): { category: Category; text: string }`
  - Task 4 imports `CATEGORIES` and `ALL_CATEGORIES` from `./quips`.

- [ ] **Step 1: Create `src/quips/index.ts`**

```ts
import { JOKES } from "./jokes";
import { HEALTH } from "./health";
import { SCIENCE } from "./science";
import { TECH } from "./tech";
import { ANIMALS } from "./animals";
import { HISTORY } from "./history";
import { MIND } from "./mind";

export type Category =
  | "jokes"
  | "health"
  | "science"
  | "tech"
  | "animals"
  | "history"
  | "mind";

export const CATEGORIES: Record<
  Category,
  { icon: string; label: string; entries: string[] }
> = {
  jokes: { icon: "🎭", label: "Jokes", entries: JOKES },
  health: { icon: "🩺", label: "Health & body", entries: HEALTH },
  science: { icon: "🚀", label: "Science & space", entries: SCIENCE },
  tech: { icon: "💻", label: "Tech & programming", entries: TECH },
  animals: { icon: "🐾", label: "Animals & nature", entries: ANIMALS },
  history: { icon: "🏛️", label: "History", entries: HISTORY },
  mind: { icon: "🧠", label: "Mind & psychology", entries: MIND },
};

export const ALL_CATEGORIES = Object.keys(CATEGORIES) as Category[];

// Unknown keys are dropped; an empty result falls back to every category so
// the popup never renders without a quip while the master toggle is on.
export function randomQuip(enabled?: string[]): {
  category: Category;
  text: string;
} {
  const keys = (enabled ?? []).filter(
    (k): k is Category => k in CATEGORIES,
  );
  const pool = keys.length > 0 ? keys : ALL_CATEGORIES;
  const flat = pool.flatMap((category) =>
    CATEGORIES[category].entries.map((text) => ({ category, text })),
  );
  return flat[Math.floor(Math.random() * flat.length)];
}
```

- [ ] **Step 2: Delete `src/quips.ts`**

```powershell
git rm src/quips.ts
```

- [ ] **Step 3: Update `src/reminder.ts`**

The import on line 2 stays `import { randomQuip } from "./quips";` but now also needs `CATEGORIES`:

```ts
import { CATEGORIES, randomQuip } from "./quips";
```

Replace the current `showQuip` and `get_settings` block (lines 135–147):

```ts
function showQuip(categories: string[]) {
  const { category, text } = randomQuip(categories);
  const el = document.querySelector("#quip")!;
  el.textContent = `${CATEGORIES[category].icon} ${text}`;
  el.classList.add("show");
}

invoke<{ sound: boolean; quips: boolean; quip_categories?: string[] }>(
  "get_settings",
)
  .then((prefs) => {
    if (prefs.sound) playChime();
    if (prefs.quips) showQuip(prefs.quip_categories ?? []);
  })
  .catch(() => {});
```

`quip_categories` is optional in the invoke type because the Rust field ships in Task 3 — until then the popup falls back to all categories. This ordering keeps every commit working.

- [ ] **Step 4: Verify build**

Run: `npm run build`
Expected: `tsc` and `vite build` both succeed, no type errors.

- [ ] **Step 5: Commit**

```powershell
git add src/quips/index.ts src/reminder.ts
git commit -m "feat(quips): category API with filtered randomQuip, popup uses category icons"
```

---

### Task 3: Rust settings field

**Files:**
- Modify: `src-tauri/src/settings.rs` (struct at lines 18–30, `Default` impl at lines 32–54)

**Interfaces:**
- Produces: `Settings.quip_categories: Vec<String>` serialized as `quip_categories` in `settings.json` and in the `get_settings`/`save_settings` command payloads (both already round-trip the whole struct — no command changes).

- [ ] **Step 1: Add the default fn and field**

In `src-tauri/src/settings.rs`, below the existing `default_true` fn (line 14–16), add:

```rust
fn all_categories() -> Vec<String> {
    ["jokes", "health", "science", "tech", "animals", "history", "mind"]
        .iter()
        .map(|s| s.to_string())
        .collect()
}
```

In the `Settings` struct, after `pub quips: bool,` (line 25), add:

```rust
    #[serde(default = "all_categories")]
    pub quip_categories: Vec<String>,
```

In `impl Default for Settings` (line 44–53), after `quips: true,` add:

```rust
            quip_categories: all_categories(),
```

- [ ] **Step 2: Verify**

Run from `src-tauri/`: `cargo check`
Expected: compiles clean (warnings about unrelated code are fine, no errors).

- [ ] **Step 3: Commit**

```powershell
git add src-tauri/src/settings.rs
git commit -m "feat(settings): quip_categories field, defaults to all, backward compatible"
```

---

### Task 4: Settings UI chip row

**Files:**
- Modify: `src/settings.ts` (Settings type line 3–9, `makeToggleRow` lines 99–117, quips row line 120, `loadCurrent` lines 151–162, save handler lines 178–195)
- Modify: `settings.html` (add `.chips`/`.chip` CSS inside the existing `<style>` block, after the `.switch` rules ending line 291)

**Interfaces:**
- Consumes: `CATEGORIES` and `ALL_CATEGORIES` from `./quips` (Task 2); `quip_categories: string[]` in the settings payload (Task 3).
- Produces: user-facing chips; saved settings include `quip_categories` with ≥1 entry whenever saved from the UI.

- [ ] **Step 1: Add imports and extend the `Settings` type in `src/settings.ts`**

```ts
import { ALL_CATEGORIES, CATEGORIES, type Category } from "./quips";
```

Extend the type (line 5–9):

```ts
type Settings = Record<Kind, ReminderSetting> & {
  autostart: boolean;
  sound: boolean;
  quips: boolean;
  quip_categories: string[];
};
```

- [ ] **Step 2: Make `makeToggleRow` return the info container**

Change the return type of `makeToggleRow` (line 99) so callers can append into the row:

```ts
function makeToggleRow(
  emoji: string,
  name: string,
  sub: string,
): { input: HTMLInputElement; info: HTMLDivElement } {
```

and its last line from `return input;` to `return { input, info };`. Update the three call sites (lines 119–121):

```ts
const soundEl = makeToggleRow("🔔", "Popup sound", "Gentle chime when a break appears").input;
const quipsRow = makeToggleRow("🎭", "Joke of the break", "A joke or fact on each popup");
const quipsEl = quipsRow.input;
const autostartEl = makeToggleRow("🚀", "Start on boot", "Launch with Windows").input;
```

- [ ] **Step 3: Build the chip row**

Directly after the call-site block from Step 2, add:

```ts
const chipsWrap = document.createElement("div");
chipsWrap.className = "chips";
const chips = {} as Record<Category, HTMLButtonElement>;
for (const key of ALL_CATEGORIES) {
  const { icon, label } = CATEGORIES[key];
  const chip = document.createElement("button");
  chip.type = "button";
  chip.className = "chip";
  chip.textContent = `${icon} ${label}`;
  chip.setAttribute("aria-pressed", "false");
  chip.addEventListener("click", () => {
    // Never allow zero active categories while the master toggle exists.
    const isLastActive =
      chip.classList.contains("on") &&
      chipsWrap.querySelectorAll(".chip.on").length === 1;
    if (isLastActive) return;
    const on = chip.classList.toggle("on");
    chip.setAttribute("aria-pressed", String(on));
  });
  chips[key] = chip;
  chipsWrap.appendChild(chip);
}
quipsRow.info.appendChild(chipsWrap);
quipsEl.addEventListener("change", () => {
  chipsWrap.hidden = !quipsEl.checked;
});

function setActiveChips(keys: string[]) {
  const valid = keys.filter((k): k is Category => k in CATEGORIES);
  const active = valid.length > 0 ? valid : ALL_CATEGORIES;
  for (const key of ALL_CATEGORIES) {
    const on = active.includes(key);
    chips[key].classList.toggle("on", on);
    chips[key].setAttribute("aria-pressed", String(on));
  }
}
```

- [ ] **Step 4: Wire load and save**

In `loadCurrent()` after `quipsEl.checked = current.quips;` add:

```ts
  setActiveChips(current.quip_categories ?? []);
  chipsWrap.hidden = !current.quips;
```

In the save handler, add `quip_categories` to the settings object literal:

```ts
  const settings = {
    autostart: autostartEl.checked,
    sound: soundEl.checked,
    quips: quipsEl.checked,
    quip_categories: ALL_CATEGORIES.filter((k) =>
      chips[k].classList.contains("on"),
    ),
  } as Settings;
```

- [ ] **Step 5: Add chip CSS to `settings.html`**

Inside the `<style>` block, after the `.switch input:focus-visible + .knob` rule (ends line 291), add:

```css
      /* category chips */
      .chips {
        display: flex;
        flex-wrap: wrap;
        gap: 0.4rem;
        margin-top: 0.6rem;
      }

      .chips[hidden] {
        display: none;
      }

      .chip {
        padding: 0.3rem 0.7rem;
        font-size: 0.78rem;
        font-family: inherit;
        color: var(--muted);
        background: var(--bg);
        border: 1px solid var(--line);
        border-radius: 999px;
        cursor: pointer;
        transition: background 0.12s ease, color 0.12s ease,
          border-color 0.12s ease;
      }

      .chip:hover {
        border-color: var(--accent);
      }

      .chip.on {
        color: var(--accent);
        background: var(--accent-dim);
        border-color: var(--accent);
      }
```

- [ ] **Step 6: Verify build**

Run: `npm run build`
Expected: clean.

- [ ] **Step 7: Commit**

```powershell
git add src/settings.ts settings.html
git commit -m "feat(settings): category chip picker under quip toggle"
```

---

### Task 5: Manual verification (dev run)

**Files:** none (verification only)

**Interfaces:**
- Consumes: everything above.

- [ ] **Step 1: Launch dev build**

Run: `npm run tauri dev` (leave running; dev intervals are 1–4 min so popups fire fast).

- [ ] **Step 2: Walk the checklist from the spec**

1. Open Settings from the tray → chip row renders under 🎭 with all 7 chips active (fresh default).
2. Toggle 🎭 off → chips hide. Toggle on → chips reappear with prior selection.
3. Deselect chips down to one → clicking the last active chip does nothing.
4. Select only `science`, Save → "Saved ✓". Close and reopen Settings → only `science` active.
5. Wait for a popup (≤1 min for eyes in dev) → quip shows 🚀 icon and a science fact. Repeat across 2–3 popups → never a joke or other category.
6. Close the app. Open the settings store file at `%APPDATA%\com.pause-ture.app\settings.json` (check `src-tauri/tauri.conf.json` for the exact identifier if that path is missing), delete the `"quip_categories"` key, relaunch → Settings shows all 7 chips active (serde default applied).

- [ ] **Step 3: Fix anything that fails, re-verify, commit fixes**

Single-line commit titles, e.g. `fix(settings): <what>`.

---

## Self-Review Notes

- Spec coverage: data layer (T1+T2), settings schema (T3), settings UI incl. last-chip guard + hide-on-master-off (T4), popup (T2), error handling / unknown keys fallback (T2 `randomQuip`, T4 `setActiveChips`), manual test checklist (T5). Out-of-scope items untouched.
- Ordering keeps every commit green: T2 popup treats `quip_categories` as optional until T3 lands.
- Type consistency: `Category`, `CATEGORIES`, `ALL_CATEGORIES`, `randomQuip(enabled?: string[])` used identically in T2 and T4; Rust field name `quip_categories` matches the TS payloads.
