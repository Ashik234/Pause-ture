mod commands;
mod popup;
mod scheduler;

use std::sync::{Arc, Mutex};

use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
};

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
            // Wired up in later commits.
            "pause" => println!("tray: pause 1 hour"),
            "settings" => println!("tray: settings"),
            _ => {}
        })
        .build(app)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(commands::PopupComplete(Mutex::new(false)))
        .manage(scheduler::SchedulerState(Arc::new(Mutex::new(
            scheduler::default_reminders(),
        ))))
        .invoke_handler(tauri::generate_handler![
            commands::complete_reminder,
            commands::snooze_reminder
        ])
        .setup(|app| {
            setup_tray(app)?;
            let reminders = app.state::<scheduler::SchedulerState>().0.clone();
            scheduler::spawn(app.app_handle().clone(), reminders);
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
