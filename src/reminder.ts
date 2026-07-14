import { invoke } from "@tauri-apps/api/core";
import { randomQuip } from "./quips";

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

function arm() {
  doneBtn.disabled = false;
  doneBtn.textContent = "Done ✓";
}

if (remaining > 0) {
  doneBtn.disabled = true;
  doneBtn.textContent = `${remaining}s`;
  const timer = setInterval(() => {
    remaining -= 1;
    if (remaining <= 0) {
      clearInterval(timer);
      arm();
    } else {
      doneBtn.textContent = `${remaining}s`;
    }
  }, 1000);
} else {
  arm();
}

doneBtn.addEventListener("click", () => {
  if (!doneBtn.disabled) invoke("complete_reminder", { kinds });
});

// Escape hatch for calls/meetings — usable even during the countdown.
snoozeBtn.addEventListener("click", () => {
  invoke("snooze_reminder", { kinds });
});

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

function showQuip() {
  const { kind, text } = randomQuip();
  const el = document.querySelector("#quip")!;
  el.textContent = `${kind === "joke" ? "🎭" : "💡"} ${text}`;
  el.classList.add("show");
}

invoke<{ sound: boolean; quips: boolean }>("get_settings")
  .then((prefs) => {
    if (prefs.sound) playChime();
    if (prefs.quips) showQuip();
  })
  .catch(() => {});
