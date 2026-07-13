use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

pub const POPUP_LABEL: &str = "reminder";

/// Opens the fullscreen reminder popup. Returns `false` without doing
/// anything if a popup is already on screen.
pub fn show(app: &AppHandle, kind: &str) -> tauri::Result<bool> {
    if app.get_webview_window(POPUP_LABEL).is_some() {
        return Ok(false);
    }

    WebviewWindowBuilder::new(
        app,
        POPUP_LABEL,
        WebviewUrl::App(format!("reminder.html?type={kind}").into()),
    )
    .title("Pause-ture")
    .fullscreen(true)
    .always_on_top(true)
    .decorations(false)
    .skip_taskbar(true)
    .focused(true)
    .build()?;

    Ok(true)
}
