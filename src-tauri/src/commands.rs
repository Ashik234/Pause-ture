use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

/// Set by `complete_reminder` just before the popup is closed; the
/// CloseRequested handler only lets the window die when this is true.
pub struct PopupComplete(pub Mutex<bool>);

#[tauri::command]
pub fn complete_reminder(app: AppHandle, state: State<PopupComplete>) {
    *state.0.lock().unwrap() = true;
    if let Some(popup) = app.get_webview_window(crate::popup::POPUP_LABEL) {
        let _ = popup.close();
    }
}
