import { invoke } from "@tauri-apps/api/core";

type ReminderSetting = { enabled: boolean; interval_min: number };
type Kind = "eyes" | "posture" | "water" | "walk";
type Settings = Record<Kind, ReminderSetting> & { autostart: boolean };

const LABELS: Record<Kind, { emoji: string; name: string }> = {
  eyes: { emoji: "👀", name: "Look away (20-20-20)" },
  posture: { emoji: "🪑", name: "Posture check" },
  water: { emoji: "💧", name: "Drink water" },
  walk: { emoji: "🚶", name: "Take a walk" },
};

const KINDS = Object.keys(LABELS) as Kind[];

const rowsEl = document.querySelector("#rows")!;
const statusEl = document.querySelector("#status")!;
const autostartEl = document.querySelector<HTMLInputElement>("#autostart")!;
const inputs = {} as Record<
  Kind,
  { enabled: HTMLInputElement; interval: HTMLInputElement }
>;

for (const kind of KINDS) {
  const row = document.createElement("div");
  row.className = "row";

  const emoji = document.createElement("span");
  emoji.className = "emoji";
  emoji.textContent = LABELS[kind].emoji;

  const name = document.createElement("label");
  name.className = "name";
  name.textContent = LABELS[kind].name;

  const interval = document.createElement("input");
  interval.type = "number";
  interval.min = "1";
  interval.max = "480";

  const unit = document.createElement("span");
  unit.className = "unit";
  unit.textContent = "min";

  const enabled = document.createElement("input");
  enabled.type = "checkbox";

  row.append(emoji, name, interval, unit, enabled);
  rowsEl.appendChild(row);
  inputs[kind] = { enabled, interval };
}

async function loadCurrent() {
  const current = await invoke<Settings>("get_settings");
  autostartEl.checked = current.autostart;
  for (const kind of KINDS) {
    inputs[kind].enabled.checked = current[kind].enabled;
    inputs[kind].interval.value = String(current[kind].interval_min);
  }
}
loadCurrent();

document.querySelector("#save")!.addEventListener("click", async () => {
  const settings = { autostart: autostartEl.checked } as Settings;
  for (const kind of KINDS) {
    const interval_min = Math.max(
      1,
      Math.min(480, Math.round(Number(inputs[kind].interval.value) || 1)),
    );
    inputs[kind].interval.value = String(interval_min);
    settings[kind] = { enabled: inputs[kind].enabled.checked, interval_min };
  }
  try {
    await invoke("save_settings", { settings });
    statusEl.textContent = "Saved ✓ — intervals restart from now";
    setTimeout(() => (statusEl.textContent = ""), 2500);
  } catch (e) {
    statusEl.textContent = `Save failed: ${e}`;
  }
});
