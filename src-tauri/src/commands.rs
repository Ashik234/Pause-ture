use std::sync::Mutex;
use std::time::Instant;
use tauri::{AppHandle, Manager, State};

use crate::scheduler::{snooze_duration, SchedulerState};

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
        .0
        .lock()
        .unwrap()
        .iter_mut()
        .filter(|r| kinds.iter().any(|k| k == r.name))
    {
        r.next_due = now + r.interval;
    }
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
        .0
        .lock()
        .unwrap()
        .iter_mut()
        .filter(|r| kinds.iter().any(|k| k == r.name))
    {
        r.next_due = now + snooze_duration();
    }
    println!("snoozed: {}", kinds.join(", "));
    close_popup(&app, &flag);
}
