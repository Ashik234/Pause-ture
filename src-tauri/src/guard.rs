//! Meeting guard: holds reminders while the user is presenting or on a call
//! so a fullscreen popup never lands mid-demo. Held reminders stay due and
//! fire on the first tick after the guard clears.

use std::sync::atomic::{AtomicBool, Ordering};

static HOLDING: AtomicBool = AtomicBool::new(false);

pub fn should_hold() -> bool {
    let hold = fullscreen_or_presentation() || mic_in_use();
    // Log only on state change, not every 30s tick.
    if hold != HOLDING.swap(hold, Ordering::Relaxed) {
        if hold {
            println!("meeting guard on — holding reminders");
        } else {
            println!("meeting guard off");
        }
    }
    hold
}

/// True when a fullscreen app, game, or presentation owns the screen —
/// the same signal Windows uses to suppress its own notifications.
fn fullscreen_or_presentation() -> bool {
    use windows::Win32::UI::Shell::{
        SHQueryUserNotificationState, QUNS_BUSY, QUNS_PRESENTATION_MODE,
        QUNS_RUNNING_D3D_FULL_SCREEN,
    };
    match unsafe { SHQueryUserNotificationState() } {
        Ok(state) => matches!(
            state,
            QUNS_BUSY | QUNS_RUNNING_D3D_FULL_SCREEN | QUNS_PRESENTATION_MODE
        ),
        Err(_) => false,
    }
}

/// True when any app currently holds the microphone (Teams/Meet/Zoom call).
/// Windows tracks this in the CapabilityAccessManager consent store:
/// an in-use app has LastUsedTimeStop == 0.
fn mic_in_use() -> bool {
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    const STORE: &str =
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\microphone";

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    for root in [STORE.to_string(), format!(r"{STORE}\NonPackaged")] {
        let Ok(key) = hkcu.open_subkey(&root) else {
            continue;
        };
        for name in key.enum_keys().flatten() {
            if name == "NonPackaged" {
                continue;
            }
            if let Ok(app) = key.open_subkey(&name) {
                if app.get_value::<u64, _>("LastUsedTimeStop").is_ok_and(|t| t == 0) {
                    return true;
                }
            }
        }
    }
    false
}
