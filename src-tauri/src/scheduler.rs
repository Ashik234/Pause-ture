use std::time::{Duration, Instant};

const TICK_SECS: u64 = 30;

pub struct Reminder {
    pub name: &'static str,
    pub interval_min: u64,
    pub last_fired: Instant,
    pub message: &'static str,
}

impl Reminder {
    fn new(name: &'static str, interval_min: u64, message: &'static str) -> Self {
        Self {
            name,
            interval_min,
            last_fired: Instant::now(),
            message,
        }
    }

    fn is_due(&self, now: Instant) -> bool {
        now.duration_since(self.last_fired) >= Duration::from_secs(self.interval_min * 60)
    }
}

pub fn default_reminders() -> Vec<Reminder> {
    // Short intervals in dev builds so firing is observable within minutes.
    let (eyes, posture, water, walk) = if cfg!(debug_assertions) {
        (1, 2, 3, 4)
    } else {
        (20, 30, 45, 60)
    };
    vec![
        Reminder::new("eyes", eyes, "Look at something 20 feet away for 20 seconds"),
        Reminder::new("posture", posture, "Sit up straight — shoulders back"),
        Reminder::new("water", water, "Drink some water"),
        Reminder::new("walk", walk, "Stand up and take a short walk"),
    ]
}

pub fn spawn(_app: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut reminders = default_reminders();
        let mut tick = tokio::time::interval(Duration::from_secs(TICK_SECS));
        loop {
            tick.tick().await;
            let now = Instant::now();
            for r in reminders.iter_mut().filter(|r| r.is_due(now)) {
                // Popup window replaces this in the next commit.
                println!("reminder due: {} — {}", r.name, r.message);
                r.last_fired = now;
            }
        }
    });
}
