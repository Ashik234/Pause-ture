import { invoke } from "@tauri-apps/api/core";

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

const kind = new URLSearchParams(window.location.search).get("type") ?? "eyes";
const copy = COPY[kind] ?? COPY.eyes;

document.querySelector("#emoji")!.textContent = copy.emoji;
document.querySelector("#title")!.textContent = copy.title;
document.querySelector("#message")!.textContent = copy.message;

// Seconds the dismiss button stays locked; 0 = instantly dismissable.
const GATE_SECONDS: Record<string, number> = {
  eyes: 20,
  posture: 10,
  water: 0,
  walk: 0,
};

const btn = document.querySelector<HTMLButtonElement>("#done")!;
let remaining = GATE_SECONDS[kind] ?? 0;

function arm() {
  btn.disabled = false;
  btn.textContent = "Done ✓";
}

if (remaining > 0) {
  btn.disabled = true;
  btn.textContent = `${remaining}s`;
  const timer = setInterval(() => {
    remaining -= 1;
    if (remaining <= 0) {
      clearInterval(timer);
      arm();
    } else {
      btn.textContent = `${remaining}s`;
    }
  }, 1000);
} else {
  arm();
}

btn.addEventListener("click", () => {
  if (!btn.disabled) invoke("complete_reminder");
});
