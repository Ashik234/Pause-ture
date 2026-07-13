mod commands;
mod popup;
mod scheduler;
mod settings;

use std::sync::Mutex;

use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager, WebviewUrl, WebviewWindowBuilder,
};

const SETTINGS_LABEL: &str = "settings";

fn open_settings(app: &AppHandle) {
    if let Some(win) = app.get_webview_window(SETTINGS_LABEL) {
        let _ = win.set_focus();
        return;
    }
    let result = WebviewWindowBuilder::new(
        app,
        SETTINGS_LABEL,
        WebviewUrl::App("settings.html".into()),
    )
    .title("Pause-ture Settings")
    .inner_size(420.0, 660.0)
    .resizable(false)
    .build();
    if let Err(e) = result {
        eprintln!("failed to open settings window: {e}");
    }
}

fn setup_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let pause = MenuItem::with_id(app, "pause", "Pause 1 hour", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&pause, &settings, &quit])?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip("Pause-ture")
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => app.exit(0),
            "pause" => {
                let state = app.state::<scheduler::SchedulerState>();
                *state.paused_until.lock().unwrap() =
                    Some(std::time::Instant::now() + scheduler::pause_duration());
                println!("paused for {:?}", scheduler::pause_duration());
            }
            "settings" => open_settings(app),
            _ => {}
        })
        .build(app)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .manage(commands::PopupComplete(Mutex::new(false)))
        .invoke_handler(tauri::generate_handler![
            commands::complete_reminder,
            commands::snooze_reminder,
            commands::get_settings,
            commands::save_settings
        ])
        .setup(|app| {
            setup_tray(app)?;
            let stored = settings::Settings::load(app.app_handle());
            settings::apply_autostart(app.app_handle(), stored.autostart);
            app.manage(scheduler::SchedulerState::new(scheduler::reminders_from(
                &stored,
            )));
            scheduler::spawn(
                app.app_handle().clone(),
                &app.state::<scheduler::SchedulerState>(),
            );
            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() != popup::POPUP_LABEL {
                return;
            }
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let state = window.state::<commands::PopupComplete>();
                let mut done = state.0.lock().unwrap();
                if *done {
                    // Reset the flag so the next popup starts locked.
                    *done = false;
                } else {
                    // Alt+F4 / anything that isn't complete_reminder.
                    api.prevent_close();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
