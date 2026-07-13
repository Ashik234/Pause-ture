use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const STORE_FILE: &str = "settings.json";
const STORE_KEY: &str = "settings";

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct ReminderSetting {
    pub enabled: bool,
    pub interval_min: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub eyes: ReminderSetting,
    pub posture: ReminderSetting,
    pub water: ReminderSetting,
    pub walk: ReminderSetting,
}

impl Default for Settings {
    fn default() -> Self {
        // Short intervals in dev builds so firing is observable within minutes.
        let (eyes, posture, water, walk) = if cfg!(debug_assertions) {
            (1, 2, 3, 4)
        } else {
            (20, 30, 45, 60)
        };
        let on = |interval_min| ReminderSetting {
            enabled: true,
            interval_min,
        };
        Self {
            eyes: on(eyes),
            posture: on(posture),
            water: on(water),
            walk: on(walk),
        }
    }
}

impl Settings {
    pub fn get(&self, name: &str) -> Option<ReminderSetting> {
        match name {
            "eyes" => Some(self.eyes),
            "posture" => Some(self.posture),
            "water" => Some(self.water),
            "walk" => Some(self.walk),
            _ => None,
        }
    }

    pub fn load(app: &AppHandle) -> Self {
        let Ok(store) = app.store(STORE_FILE) else {
            return Self::default();
        };
        store
            .get(STORE_KEY)
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, app: &AppHandle) -> Result<(), tauri_plugin_store::Error> {
        let store = app.store(STORE_FILE)?;
        store.set(STORE_KEY, serde_json::to_value(self)?);
        store.save()
    }
}
