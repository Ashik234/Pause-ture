//! Typing guard: a due reminder waits for a pause in typing so the popup
//! never lands mid-sentence. Capped — nonstop typing can't postpone a
//! break forever.

use std::time::Duration;

/// No keystroke for this long counts as a pause in typing.
const TYPING_GAP: Duration = Duration::from_secs(10);

/// Longest a due reminder can be held for typing before firing anyway.
pub fn hold_cap() -> Duration {
    if cfg!(debug_assertions) {
        Duration::from_secs(60)
    } else {
        Duration::from_secs(5 * 60)
    }
}

pub fn spawn_hook() {
    imp::spawn();
}

/// True while the user is mid-typing — a keystroke landed within the gap.
pub fn typing_recently() -> bool {
    imp::last_key_within(TYPING_GAP)
}

#[cfg(windows)]
mod imp {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::Duration;
    use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
    use windows::Win32::System::SystemInformation::GetTickCount64;
    use windows::Win32::UI::WindowsAndMessaging::{
        CallNextHookEx, DispatchMessageW, GetMessageW, SetWindowsHookExW, TranslateMessage, MSG,
        WH_KEYBOARD_LL, WM_KEYDOWN, WM_SYSKEYDOWN,
    };

    /// GetTickCount64 millis of the last keydown; 0 = no key seen yet.
    static LAST_KEY_MS: AtomicU64 = AtomicU64::new(0);

    unsafe extern "system" fn on_key(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        if code >= 0 && matches!(wparam.0 as u32, WM_KEYDOWN | WM_SYSKEYDOWN) {
            LAST_KEY_MS.store(unsafe { GetTickCount64() }, Ordering::Relaxed);
        }
        unsafe { CallNextHookEx(None, code, wparam, lparam) }
    }

    pub fn spawn() {
        std::thread::spawn(|| unsafe {
            if let Err(e) = SetWindowsHookExW(WH_KEYBOARD_LL, Some(on_key), None, 0) {
                eprintln!("keyboard hook failed — typing guard off: {e}");
                return;
            }
            // Low-level hooks only run while the installing thread pumps messages.
            let mut msg = MSG::default();
            while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        });
    }

    pub fn last_key_within(gap: Duration) -> bool {
        let last = LAST_KEY_MS.load(Ordering::Relaxed);
        last != 0 && unsafe { GetTickCount64() }.saturating_sub(last) < gap.as_millis() as u64
    }
}

#[cfg(not(windows))]
mod imp {
    pub fn spawn() {}

    pub fn last_key_within(_gap: std::time::Duration) -> bool {
        false
    }
}
