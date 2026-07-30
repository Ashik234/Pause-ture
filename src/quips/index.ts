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
  const keys = (enabled ?? []).filter((k): k is Category => k in CATEGORIES);
  const pool = keys.length > 0 ? keys : ALL_CATEGORIES;
  const flat = pool.flatMap((category) =>
    CATEGORIES[category].entries.map((text) => ({ category, text })),
  );
  return flat[Math.floor(Math.random() * flat.length)];
}
