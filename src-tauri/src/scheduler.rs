use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const TICK_SECS: u64 = 30;

/// Reminders due within this window of a firing popup ride along in it.
pub fn merge_window() -> Duration {
    if cfg!(debug_assertions) {
        Duration::from_secs(30)
    } else {
        Duration::from_secs(5 * 60)
    }
}

pub fn snooze_duration() -> Duration {
    if cfg!(debug_assertions) {
        Duration::from_secs(60)
    } else {
        Duration::from_secs(5 * 60)
    }
}

pub fn pause_duration() -> Duration {
    if cfg!(debug_assertions) {
        Duration::from_secs(2 * 60)
    } else {
        Duration::from_secs(60 * 60)
    }
}

pub struct Reminder {
    pub name: &'static str,
    pub enabled: bool,
    pub interval: Duration,
    pub next_due: Instant,
}

impl Reminder {
    fn new(name: &'static str, setting: crate::settings::ReminderSetting) -> Self {
        let interval = Duration::from_secs(setting.interval_min * 60);
        Self {
            name,
            enabled: setting.enabled,
            interval,
            next_due: Instant::now() + interval,
        }
    }

    fn is_due(&self, now: Instant) -> bool {
        self.enabled && now >= self.next_due
    }
}

/// Shared between the tick loop (fires popups), commands (complete
/// resets next_due, snooze pushes it forward) and the tray (pause).
pub struct SchedulerState {
    pub reminders: Arc<Mutex<Vec<Reminder>>>,
    pub paused_until: Arc<Mutex<Option<Instant>>>,
}

impl SchedulerState {
    pub fn new(reminders: Vec<Reminder>) -> Self {
        Self {
            reminders: Arc::new(Mutex::new(reminders)),
            paused_until: Arc::new(Mutex::new(None)),
        }
    }
}

pub fn reminders_from(settings: &crate::settings::Settings) -> Vec<Reminder> {
    vec![
        Reminder::new("eyes", settings.eyes),
        Reminder::new("posture", settings.posture),
        Reminder::new("water", settings.water),
        Reminder::new("walk", settings.walk),
    ]
}

pub fn spawn(app: tauri::AppHandle, state: &SchedulerState) {
    let reminders = state.reminders.clone();
    let paused_until = state.paused_until.clone();
    tauri::async_runtime::spawn(async move {
        let mut tick = tokio::time::interval(Duration::from_secs(TICK_SECS));
        loop {
            tick.tick().await;
            let now = Instant::now();

            {
                let mut paused = paused_until.lock().unwrap();
                if let Some(until) = *paused {
                    if now < until {
                        continue;
                    }
                    *paused = None;
                    println!("pause ended");
                }
            }

            // next_due is only advanced by complete/snooze commands, so an
            // unanswered popup keeps these reminders due; show() just no-ops
            // while it is still on screen.
            let kinds: Vec<&'static str> = {
                let rs = reminders.lock().unwrap();
                if rs.iter().any(|r| r.is_due(now)) {
                    rs.iter()
                        .filter(|r| r.enabled && r.next_due <= now + merge_window())
                        .map(|r| r.name)
                        .collect()
                } else {
                    Vec::new()
                }
            };

            if kinds.is_empty() {
                continue;
            }
            match crate::popup::show(&app, &kinds.join(",")) {
                Ok(true) => println!("reminder fired: {}", kinds.join(", ")),
                Ok(false) => {}
                Err(e) => eprintln!("failed to open reminder popup: {e}"),
            }
        }
    });
}
