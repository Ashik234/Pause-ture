use std::time::{Duration, Instant};

const TICK_SECS: u64 = 30;

pub struct Reminder {
    pub name: &'static str,
    pub interval_min: u64,
    pub last_fired: Instant,
}

impl Reminder {
    fn new(name: &'static str, interval_min: u64) -> Self {
        Self {
            name,
            interval_min,
            last_fired: Instant::now(),
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
        Reminder::new("eyes", eyes),
        Reminder::new("posture", posture),
        Reminder::new("water", water),
        Reminder::new("walk", walk),
    ]
}

pub fn spawn(app: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut reminders = default_reminders();
        let mut tick = tokio::time::interval(Duration::from_secs(TICK_SECS));
        loop {
            tick.tick().await;
            let now = Instant::now();
            for r in reminders.iter_mut().filter(|r| r.is_due(now)) {
                match crate::popup::show(&app, r.name) {
                    Ok(true) => {
                        println!("reminder fired: {}", r.name);
                        r.last_fired = now;
                    }
                    // Popup already on screen — retry this reminder next tick.
                    Ok(false) => {}
                    Err(e) => eprintln!("failed to open reminder popup: {e}"),
                }
            }
        }
    });
}
