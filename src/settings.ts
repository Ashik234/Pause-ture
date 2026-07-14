import { invoke } from "@tauri-apps/api/core";

type ReminderSetting = { enabled: boolean; interval_min: number };
type Kind = "eyes" | "posture" | "water" | "walk";
type Settings = Record<Kind, ReminderSetting> & {
  autostart: boolean;
  sound: boolean;
};

const LABELS: Record<Kind, { emoji: string; name: string; sub: string }> = {
  eyes: { emoji: "👀", name: "Look away", sub: "20-20-20 rule for your eyes" },
  posture: { emoji: "🪑", name: "Posture check", sub: "Shoulders back, unslouch" },
  water: { emoji: "💧", name: "Drink water", sub: "Stay hydrated" },
  walk: { emoji: "🚶", name: "Take a walk", sub: "Get up and move" },
};

const KINDS = Object.keys(LABELS) as Kind[];
const STEP = 5;
const clamp = (n: number) => Math.max(1, Math.min(480, Math.round(n)));

const rowsEl = document.querySelector("#rows")!;
const inputs = {} as Record<
  Kind,
  { enabled: HTMLInputElement; interval: HTMLInputElement }
>;

function makeSwitch(): { wrap: HTMLLabelElement; input: HTMLInputElement } {
  const wrap = document.createElement("label");
  wrap.className = "switch";
  const input = document.createElement("input");
  input.type = "checkbox";
  const knob = document.createElement("span");
  knob.className = "knob";
  wrap.append(input, knob);
  return { wrap, input };
}

function makeRow(kind: Kind) {
  const { emoji, name, sub } = LABELS[kind];

  const row = document.createElement("div");
  row.className = "row";

  const tile = document.createElement("div");
  tile.className = "tile";
  tile.textContent = emoji;

  const info = document.createElement("div");
  const nameEl = document.createElement("div");
  nameEl.className = "name";
  nameEl.textContent = name;
  const subEl = document.createElement("div");
  subEl.className = "sub";
  subEl.textContent = sub;

  const stepper = document.createElement("div");
  stepper.className = "stepper";
  const minus = document.createElement("button");
  minus.textContent = "−";
  minus.setAttribute("aria-label", `Less often: ${name}`);
  const interval = document.createElement("input");
  interval.type = "number";
  interval.min = "1";
  interval.max = "480";
  const unit = document.createElement("span");
  unit.className = "unit";
  unit.textContent = "min";
  const plus = document.createElement("button");
  plus.textContent = "+";
  plus.setAttribute("aria-label", `More often: ${name}`);

  minus.addEventListener("click", () => {
    interval.value = String(clamp(Number(interval.value) - STEP));
  });
  plus.addEventListener("click", () => {
    interval.value = String(clamp(Number(interval.value) + STEP));
  });
  interval.addEventListener("change", () => {
    interval.value = String(clamp(Number(interval.value) || 1));
  });

  stepper.append(minus, interval, unit, plus);
  info.append(nameEl, subEl, stepper);

  const { wrap, input: enabled } = makeSwitch();
  enabled.addEventListener("change", () => {
    row.classList.toggle("off", !enabled.checked);
  });

  row.append(tile, info, wrap);
  rowsEl.appendChild(row);
  inputs[kind] = { enabled, interval };
}

for (const kind of KINDS) makeRow(kind);

// switch-only rows (no stepper)
function makeToggleRow(emoji: string, name: string, sub: string): HTMLInputElement {
  const row = document.createElement("div");
  row.className = "row";
  const tile = document.createElement("div");
  tile.className = "tile";
  tile.textContent = emoji;
  const info = document.createElement("div");
  const nameEl = document.createElement("div");
  nameEl.className = "name";
  nameEl.textContent = name;
  const subEl = document.createElement("div");
  subEl.className = "sub";
  subEl.textContent = sub;
  info.append(nameEl, subEl);
  const { wrap, input } = makeSwitch();
  row.append(tile, info, wrap);
  rowsEl.appendChild(row);
  return input;
}

const soundEl = makeToggleRow("🔔", "Popup sound", "Gentle chime when a break appears");
const autostartEl = makeToggleRow("🚀", "Start on boot", "Launch with Windows");

async function loadCurrent() {
  const current = await invoke<Settings>("get_settings");
  autostartEl.checked = current.autostart;
  soundEl.checked = current.sound;
  for (const kind of KINDS) {
    inputs[kind].enabled.checked = current[kind].enabled;
    inputs[kind].interval.value = String(current[kind].interval_min);
    inputs[kind].enabled.dispatchEvent(new Event("change"));
  }
}
loadCurrent();

const saveBtn = document.querySelector<HTMLButtonElement>("#save")!;
let revert: number | undefined;

function flash(label: string, cls: "saved" | "error") {
  clearTimeout(revert);
  saveBtn.textContent = label;
  saveBtn.classList.remove("saved", "error");
  saveBtn.classList.add(cls);
  revert = window.setTimeout(() => {
    saveBtn.textContent = "Save changes";
    saveBtn.classList.remove("saved", "error");
  }, 1800);
}

saveBtn.addEventListener("click", async () => {
  const settings = {
    autostart: autostartEl.checked,
    sound: soundEl.checked,
  } as Settings;
  for (const kind of KINDS) {
    const interval_min = clamp(Number(inputs[kind].interval.value) || 1);
    inputs[kind].interval.value = String(interval_min);
    settings[kind] = { enabled: inputs[kind].enabled.checked, interval_min };
  }
  try {
    await invoke("save_settings", { settings });
    flash("Saved ✓", "saved");
  } catch {
    flash("Save failed — try again", "error");
  }
});
