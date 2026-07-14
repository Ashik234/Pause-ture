mod commands;
mod guard;
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

/// Tray pieces that change while running: tooltip and the Resume item.
struct TrayHandles {
    tray: tauri::tray::TrayIcon<tauri::Wry>,
    resume: MenuItem<tauri::Wry>,
}

/// Reflects pause state in the tray. `until = None` means running.
pub(crate) fn set_pause_ui(app: &AppHandle, until: Option<chrono::DateTime<chrono::Local>>) {
    let handles = app.state::<TrayHandles>();
    match until {
        Some(t) => {
            let _ = handles
                .tray
                .set_tooltip(Some(format!("Pause-ture — paused until {}", t.format("%H:%M"))));
            let _ = handles.resume.set_enabled(true);
        }
        None => {
            let _ = handles.tray.set_tooltip(Some("Pause-ture"));
            let _ = handles.resume.set_enabled(false);
        }
    }
}

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
    .inner_size(420.0, 760.0)
    .resizable(false)
    .build();
    if let Err(e) = result {
        eprintln!("failed to open settings window: {e}");
    }
}

fn setup_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let pause_label = if cfg!(debug_assertions) {
        "Pause 2 min (dev)"
    } else {
        "Pause 1 hour"
    };
    let break_now = MenuItem::with_id(app, "break_now", "Taking a break now", true, None::<&str>)?;
    let pause = MenuItem::with_id(app, "pause", pause_label, true, None::<&str>)?;
    let resume = MenuItem::with_id(app, "resume", "Resume", false, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&break_now, &pause, &resume, &settings, &quit])?;

    let tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip("Pause-ture")
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => app.exit(0),
            // User is stepping away voluntarily — every timer restarts from now.
            "break_now" => {
                let state = app.state::<scheduler::SchedulerState>();
                let now = std::time::Instant::now();
                for r in state.reminders.lock().unwrap().iter_mut() {
                    r.next_due = now + r.interval;
                }
                println!("break taken — all timers reset");
            }
            "pause" => {
                let state = app.state::<scheduler::SchedulerState>();
                *state.paused_until.lock().unwrap() =
                    Some(std::time::Instant::now() + scheduler::pause_duration());
                let until = chrono::Local::now()
                    + chrono::Duration::from_std(scheduler::pause_duration()).unwrap();
                set_pause_ui(app, Some(until));
                println!("paused until {}", until.format("%H:%M"));
            }
            "resume" => {
                let state = app.state::<scheduler::SchedulerState>();
                *state.paused_until.lock().unwrap() = None;
                set_pause_ui(app, None);
                println!("resumed");
            }
            "settings" => open_settings(app),
            _ => {}
        })
        .build(app)?;

    app.manage(TrayHandles { tray, resume });

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
