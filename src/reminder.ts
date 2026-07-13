import { getCurrentWindow } from "@tauri-apps/api/window";

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

// Plain close for now — countdown gating + complete_reminder come next commit.
document.querySelector("#done")!.addEventListener("click", () => {
  getCurrentWindow().close();
});
