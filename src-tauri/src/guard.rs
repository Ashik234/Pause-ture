//! Meeting guard: holds reminders while the user is presenting or on a call
//! so a fullscreen popup never lands mid-demo. Held reminders stay due and
//! fire on the first tick after the guard clears.

use std::sync::atomic::{AtomicBool, Ordering};

static HOLDING: AtomicBool = AtomicBool::new(false);
static LOCKED: AtomicBool = AtomicBool::new(false);

/// True while the workstation is locked (Win+L / lock screen). The input
/// desktop switches to the secure Winlogon desktop, which this process
/// cannot open.
pub fn is_locked() -> bool {
    let locked = workstation_locked();
    if locked != LOCKED.swap(locked, Ordering::Relaxed) {
        if locked {
            println!("workstation locked — timers frozen");
        } else {
            println!("workstation unlocked — timers running");
        }
    }
    locked
}

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
#[cfg(windows)]
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
#[cfg(windows)]
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

#[cfg(windows)]
fn workstation_locked() -> bool {
    use windows::Win32::System::StationsAndDesktops::{
        CloseDesktop, OpenInputDesktop, DESKTOP_ACCESS_FLAGS, DESKTOP_CONTROL_FLAGS,
        DESKTOP_SWITCHDESKTOP,
    };
    match unsafe {
        OpenInputDesktop(
            DESKTOP_CONTROL_FLAGS(0),
            false,
            DESKTOP_ACCESS_FLAGS(DESKTOP_SWITCHDESKTOP.0 as u32),
        )
    } {
        Ok(desktop) => {
            let _ = unsafe { CloseDesktop(desktop) };
            false
        }
        Err(_) => true,
    }
}

// macOS/Linux: no guard signals implemented yet — never hold.
#[cfg(not(windows))]
fn fullscreen_or_presentation() -> bool {
    false
}

#[cfg(not(windows))]
fn workstation_locked() -> bool {
    false
}

#[cfg(not(windows))]
fn mic_in_use() -> bool {
    false
}
