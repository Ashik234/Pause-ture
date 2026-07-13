use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

pub const POPUP_LABEL: &str = "reminder";

/// Opens the fullscreen reminder popup for one or more comma-separated
/// reminder kinds. Returns `false` without doing anything if a popup is
/// already on screen.
pub fn show(app: &AppHandle, kinds: &str) -> tauri::Result<bool> {
    if app.get_webview_window(POPUP_LABEL).is_some() {
        return Ok(false);
    }

    WebviewWindowBuilder::new(
        app,
        POPUP_LABEL,
        WebviewUrl::App(format!("reminder.html?types={kinds}").into()),
    )
    .title("Pause-ture")
    .fullscreen(true)
    .always_on_top(true)
    .decorations(false)
    .skip_taskbar(true)
    .focused(true)
    .transparent(true)
    .build()?;

    Ok(true)
}
