//! Daily done-vs-snoozed counters, keyed by local date in stats.json.
//! The honesty stat: how many breaks you actually took today.

use serde::Serialize;
use serde_json::json;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const STORE_FILE: &str = "stats.json";

fn today() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

pub fn bump(app: &AppHandle, key: &str, by: u64) {
    let Ok(store) = app.store(STORE_FILE) else {
        return;
    };
    let day = today();
    let mut entry = store.get(&day).unwrap_or_else(|| json!({}));
    let current = entry.get(key).and_then(|v| v.as_u64()).unwrap_or(0);
    entry[key] = json!(current + by);
    store.set(day, entry);
    let _ = store.save();
}

#[derive(Serialize, Default)]
pub struct DayStats {
    pub done: u64,
    pub snoozed: u64,
    pub locked_secs: u64,
}

pub fn today_stats(app: &AppHandle) -> DayStats {
    let Ok(store) = app.store(STORE_FILE) else {
        return DayStats::default();
    };
    let Some(entry) = store.get(today()) else {
        return DayStats::default();
    };
    let get = |key: &str| entry.get(key).and_then(|v| v.as_u64()).unwrap_or(0);
    DayStats {
        done: get("done"),
        snoozed: get("snoozed"),
        locked_secs: get("locked_secs"),
    }
}
