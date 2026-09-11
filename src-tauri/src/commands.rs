use std::sync::Mutex;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Manager, State};

use crate::scheduler::{snooze_duration, SchedulerState};
use crate::settings::Settings;

/// Set by `complete_reminder`/`snooze_reminder` just before the popup is
/// closed; the CloseRequested handler only lets the window die when true.
pub struct PopupComplete(pub Mutex<bool>);

fn close_popup(app: &AppHandle, flag: &State<PopupComplete>) {
    *flag.0.lock().unwrap() = true;
    if let Some(popup) = app.get_webview_window(crate::popup::POPUP_LABEL) {
        let _ = popup.close();
    }
}

#[tauri::command]
pub fn complete_reminder(
    app: AppHandle,
    sched: State<SchedulerState>,
    flag: State<PopupComplete>,
    kinds: Vec<String>,
) {
    let now = Instant::now();
    for r in sched
        .reminders
        .lock()
        .unwrap()
        .iter_mut()
        .filter(|r| kinds.iter().any(|k| k == r.name))
    {
        r.next_due = now + r.interval;
    }
    crate::stats::bump(&app, "done", kinds.len() as u64);
    close_popup(&app, &flag);
}

#[tauri::command]
pub fn snooze_reminder(
    app: AppHandle,
    sched: State<SchedulerState>,
    flag: State<PopupComplete>,
    kinds: Vec<String>,
) {
    let now = Instant::now();
    for r in sched
        .reminders
        .lock()
        .unwrap()
        .iter_mut()
        .filter(|r| kinds.iter().any(|k| k == r.name))
    {
        r.next_due = now + snooze_duration();
    }
    println!("snoozed: {}", kinds.join(", "));
    crate::stats::bump(&app, "snoozed", kinds.len() as u64);
    close_popup(&app, &flag);
}

#[tauri::command]
pub fn get_stats(app: AppHandle) -> crate::stats::DayStats {
    crate::stats::today_stats(&app)
}

#[tauri::command]
pub fn break_now(app: AppHandle) {
    crate::scheduler::reset_all_timers(&app);
}

#[tauri::command]
pub fn pause_reminders(app: AppHandle) -> String {
    crate::scheduler::pause(&app).format("%H:%M").to_string()
}

#[tauri::command]
pub fn resume_reminders(app: AppHandle) {
    crate::scheduler::resume(&app);
}

/// Wall-clock time the current pause ends ("HH:MM"), or None when running.
#[tauri::command]
pub fn get_pause_state(sched: State<SchedulerState>) -> Option<String> {
    sched.paused_until.lock().unwrap().and_then(|until| {
        let remaining = until.checked_duration_since(Instant::now())?;
        let until_wall = chrono::Local::now() + chrono::Duration::from_std(remaining).ok()?;
        Some(until_wall.format("%H:%M").to_string())
    })
}

#[tauri::command]
pub fn get_settings(app: AppHandle) -> Settings {
    Settings::load(&app)
}

#[tauri::command]
pub fn save_settings(
    app: AppHandle,
    sched: State<SchedulerState>,
    settings: Settings,
) -> Result<(), String> {
    settings.save(&app).map_err(|e| e.to_string())?;
    crate::settings::apply_autostart(&app, settings.autostart);

    *sched.work_hours.lock().unwrap() = settings.work_hours.clone();

    // Apply live: new intervals count from now.
    let now = Instant::now();
    for r in sched.reminders.lock().unwrap().iter_mut() {
        if let Some(s) = settings.get(r.name) {
            r.enabled = s.enabled;
            r.interval = Duration::from_secs(s.interval_min * 60);
            r.next_due = now + r.interval;
        }
    }
    Ok(())
}
