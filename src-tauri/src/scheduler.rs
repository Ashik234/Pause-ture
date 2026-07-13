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

pub struct Reminder {
    pub name: &'static str,
    pub interval: Duration,
    pub next_due: Instant,
}

impl Reminder {
    fn new(name: &'static str, interval_min: u64) -> Self {
        let interval = Duration::from_secs(interval_min * 60);
        Self {
            name,
            interval,
            next_due: Instant::now() + interval,
        }
    }

    fn is_due(&self, now: Instant) -> bool {
        now >= self.next_due
    }
}

/// Shared between the tick loop (fires popups) and commands
/// (complete resets next_due, snooze pushes it forward).
pub struct SchedulerState(pub Arc<Mutex<Vec<Reminder>>>);

pub fn default_reminders() -> Vec<Reminder> {
    // Short intervals in dev builds so firing is observable within minutes.
    let (eyes, posture, water, walk) = if cfg!(debug_assertions) {
        (1, 2, 3, 4)
    } else {
        (20, 30, 45, 60)
    };
    vec![
        Reminder::new("eyes", eyes),
        Reminder::new("posture", posture),
        Reminder::new("water", water),
        Reminder::new("walk", walk),
    ]
}

pub fn spawn(app: tauri::AppHandle, reminders: Arc<Mutex<Vec<Reminder>>>) {
    tauri::async_runtime::spawn(async move {
        let mut tick = tokio::time::interval(Duration::from_secs(TICK_SECS));
        loop {
            tick.tick().await;
            let now = Instant::now();

            // next_due is only advanced by complete/snooze commands, so an
            // unanswered popup keeps these reminders due; show() just no-ops
            // while it is still on screen.
            let kinds: Vec<&'static str> = {
                let rs = reminders.lock().unwrap();
                if rs.iter().any(|r| r.is_due(now)) {
                    rs.iter()
                        .filter(|r| r.next_due <= now + merge_window())
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
