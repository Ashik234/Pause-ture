import { invoke } from "@tauri-apps/api/core";
import { CATEGORIES, randomQuip, type Category } from "./quips";

type Copy = { emoji: string; title: string; message: string };

const COPY: Record<string, Copy> = {
  eyes: {
    emoji: "👀",
    title: "Look away",
    message: "Focus on something 20 feet away for 20 seconds.",
  },
  posture: {
    emoji: "🪑",
    title: "Posture check",
    message: "Sit up straight — shoulders back, screen at eye level.",
  },
  water: {
    emoji: "💧",
    title: "Drink water",
    message: "A few sips now. Your future self says thanks.",
  },
  walk: {
    emoji: "🚶",
    title: "Take a walk",
    message: "Stand up and move for a couple of minutes.",
  },
};

// Seconds the dismiss button stays locked; 0 = instantly dismissable.
const GATE_SECONDS: Record<string, number> = {
  eyes: 20,
  posture: 10,
  water: 0,
  walk: 0,
};

const kinds = (new URLSearchParams(window.location.search).get("types") ?? "eyes")
  .split(",")
  .filter((k) => k in COPY);
if (kinds.length === 0) kinds.push("eyes");

const itemsEl = document.querySelector("#items")!;
for (const k of kinds) {
  const copy = COPY[k];
  const item = document.createElement("div");
  item.className = "item";

  const emoji = document.createElement("div");
  emoji.className = "tile";
  emoji.textContent = copy.emoji;

  const title = document.createElement("h1");
  title.textContent = copy.title;

  const message = document.createElement("p");
  message.className = "message";
  message.textContent = copy.message;

  item.append(emoji, title, message);
  itemsEl.appendChild(item);
}
if (kinds.length > 1) document.body.classList.add("multi");

const doneBtn = document.querySelector<HTMLButtonElement>("#done")!;
const snoozeBtn = document.querySelector<HTMLButtonElement>("#snooze")!;

// Merged popups gate on the strictest reminder in the batch.
let remaining = Math.max(...kinds.map((k) => GATE_SECONDS[k] ?? 0));

// Keys are ignored until shortly after the gate opens: a keystroke already
// in flight (or a held key) must not dismiss the break the instant it arms.
const KEY_GRACE_MS = 400;
let keysArmedAt = Number.POSITIVE_INFINITY;

function arm() {
  doneBtn.disabled = false;
  doneBtn.textContent = "Done ✓";
  doneBtn.title = "Enter or Space";
  keysArmedAt = performance.now() + KEY_GRACE_MS;
  // Makes the focus ring visible, so the shortcut is discoverable.
  doneBtn.focus({ preventScroll: true });
}

// Countdown finished — the break was taken, so fade out and complete
// without asking for a click.
function fadeOutAndComplete() {
  doneBtn.disabled = true;
  snoozeBtn.disabled = true;
  document.body.classList.add("leave");
  setTimeout(() => invoke("complete_reminder", { kinds }), 450);
}

if (remaining > 0) {
  doneBtn.disabled = true;
  doneBtn.textContent = `${remaining}s`;
  const timer = setInterval(() => {
    remaining -= 1;
    if (remaining <= 0) {
      clearInterval(timer);
      fadeOutAndComplete();
    } else {
      doneBtn.textContent = `${remaining}s`;
    }
  }, 1000);
} else {
  arm();
}

function done() {
  if (!doneBtn.disabled) invoke("complete_reminder", { kinds });
}

doneBtn.addEventListener("click", done);

// Keyboard: Enter/Space complete the break once the gate is open, Escape
// snoozes. Held keys don't count — only a fresh press after the grace period.
window.addEventListener("keydown", (e) => {
  if (e.key === "Escape") {
    e.preventDefault();
    snooze();
    return;
  }
  if (e.key !== "Enter" && e.key !== " ") return;
  // If the user tabbed to Snooze, let the button handle its own activation.
  if (document.activeElement === snoozeBtn) return;
  e.preventDefault();
  if (e.repeat || performance.now() < keysArmedAt) return;
  done();
});

// Escape hatch for calls/meetings — usable even during the countdown.
function snooze() {
  if (!snoozeBtn.disabled) invoke("snooze_reminder", { kinds });
}

snoozeBtn.addEventListener("click", snooze);
snoozeBtn.title = "Esc";

// Gentle two-note chime, synthesized so no audio asset ships.
function playChime() {
  try {
    const ctx = new AudioContext();
    if (ctx.state === "suspended") ctx.resume();

    const note = (freq: number, at: number) => {
      const osc = ctx.createOscillator();
      const gain = ctx.createGain();
      osc.type = "sine";
      osc.frequency.value = freq;
      const t = ctx.currentTime + at;
      gain.gain.setValueAtTime(0, t);
      gain.gain.linearRampToValueAtTime(0.1, t + 0.06);
      gain.gain.exponentialRampToValueAtTime(0.001, t + 1.1);
      osc.connect(gain).connect(ctx.destination);
      osc.start(t);
      osc.stop(t + 1.2);
    };
    note(659.25, 0); // E5
    note(987.77, 0.18); // B5
  } catch {
    // No audio device or blocked autoplay — popup works fine silent.
  }
}

function renderQuip(icon: string, text: string) {
  const el = document.querySelector("#quip")!;
  el.textContent = `${icon} ${text}`;
  el.classList.add("show");
}

// Wikimedia's free "on this day" feed — real events for today's date.
async function fetchOnThisDay(): Promise<string> {
  const now = new Date();
  const url = `https://api.wikimedia.org/feed/v1/wikipedia/en/onthisday/selected/${now.getMonth() + 1}/${now.getDate()}`;
  const res = await fetch(url, { signal: AbortSignal.timeout(3000) });
  if (!res.ok) throw new Error(`onthisday ${res.status}`);
  const data: { selected: { year: number; text: string }[] } = await res.json();
  const pick = data.selected[Math.floor(Math.random() * data.selected.length)];
  if (!pick) throw new Error("onthisday empty");
  return `On this day in ${pick.year}: ${clip(pick.text, 200)}`;
}

// Wiktionary's word-of-the-day feed. Entries embed HTML whose stable
// element ids (WOTD-rss-title/-description) are the supported way to parse it.
async function fetchWordOfTheDay(): Promise<string> {
  const url =
    "https://en.wiktionary.org/w/api.php?action=featuredfeed&feed=wotd&feedformat=atom&origin=*";
  const res = await fetch(url, { signal: AbortSignal.timeout(3000) });
  if (!res.ok) throw new Error(`wordofday ${res.status}`);
  const xml = new DOMParser().parseFromString(await res.text(), "text/xml");
  const entries = xml.querySelectorAll("entry");
  // Feed lists several days; the last entry is today's word.
  const html = entries[entries.length - 1]?.querySelector("summary")
    ?.textContent;
  const doc = new DOMParser().parseFromString(html ?? "", "text/html");
  const word = doc.querySelector("#WOTD-rss-title")?.textContent?.trim();
  const def = doc
    .querySelector("#WOTD-rss-description")
    ?.textContent?.trim()
    .replace(/\s+/g, " ");
  if (!word || !def) throw new Error("wordofday parse");
  return `Word of the day: ${word} — ${firstSense(def)}`;
}

// Trim to a whole word so the popup never shows a clipped fragment.
function clip(text: string, max: number): string {
  const t = text.trim();
  if (t.length <= max) return t;
  const cut = t.lastIndexOf(" ", max);
  return `${t.slice(0, cut > max / 2 ? cut : max).replace(/[,;:.]$/, "")}…`;
}

// Wiktionary descriptions stack every sense of the word into one blob, often
// hundreds of characters. Keep the first sense and end on a whole word so the
// popup never shows a clipped fragment.
function firstSense(def: string): string {
  // Numbered senses ("1. ... 2. ...") or a leading part-of-speech gloss.
  let text = def.replace(/^\s*\d+[.)]\s*/, "").split(/\s+\d+[.)]\s+/)[0]!;
  // Semicolons separate sub-senses; a full stop ends the sentence.
  const stop = text.search(/[.;](\s|$)/);
  if (stop > 40) text = text.slice(0, stop + 1);
  text = text.trim().replace(/;$/, ".");
  return clip(text, 150);
}

// Live categories are fetched at popup time, so they can't sit in
// randomQuip's static pool — they get an even share among enabled ones here.
const LIVE_QUIPS: Partial<Record<Category, () => Promise<string>>> = {
  onthisday: fetchOnThisDay,
  wordofday: fetchWordOfTheDay,
};

async function showQuip(categories: string[]) {
  const enabled = categories.length > 0 ? categories : Object.keys(CATEGORIES);
  const live = enabled.filter((c): c is Category => c in LIVE_QUIPS);
  if (live.length > 0 && Math.random() < live.length / enabled.length) {
    const pick = live[Math.floor(Math.random() * live.length)];
    try {
      renderQuip(CATEGORIES[pick].icon, await LIVE_QUIPS[pick]!());
      return;
    } catch {
      // Offline or slow — fall through to a bundled quip.
    }
  }
  const { category, text } = randomQuip(
    categories.filter((c) => !(c in LIVE_QUIPS)),
  );
  renderQuip(CATEGORIES[category].icon, text);
}

invoke<{ sound: boolean; quips: boolean; quip_categories?: string[] }>(
  "get_settings",
)
  .then((prefs) => {
    if (prefs.sound) playChime();
    if (prefs.quips) showQuip(prefs.quip_categories ?? []);
  })
  .catch(() => {});
