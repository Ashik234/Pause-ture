//! Work hours guard: outside the configured window reminders freeze instead
//! of firing, so the app stays quiet on evenings and weekends. Disabled by
//! default — an install that never opens the setting behaves as it always has.

use chrono::{Datelike, Local, NaiveTime, Timelike};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkHours {
    pub enabled: bool,
    /// "HH:MM", local time.
    pub start: String,
    pub end: String,
    /// ISO weekdays: 1 = Monday … 7 = Sunday.
    pub days: Vec<u8>,
}

impl Default for WorkHours {
    fn default() -> Self {
        Self {
            enabled: false,
            start: "09:00".into(),
            end: "18:00".into(),
            days: vec![1, 2, 3, 4, 5],
        }
    }
}

/// Minutes since midnight, or None if the string isn't "HH:MM".
fn minutes(hhmm: &str) -> Option<u32> {
    let t = NaiveTime::parse_from_str(hhmm.trim(), "%H:%M").ok()?;
    Some(t.hour() * 60 + t.minute())
}

impl WorkHours {
    /// True when reminders may fire. A malformed or empty config allows
    /// everything — a typo in the store must never silence the app for good.
    pub fn allows(&self, weekday: u8, minute_of_day: u32) -> bool {
        if !self.enabled || self.days.is_empty() {
            return true;
        }
        let (Some(start), Some(end)) = (minutes(&self.start), minutes(&self.end)) else {
            return true;
        };
        if start == end {
            return true;
        }

        if start < end {
            // Same-day window: 09:00–18:00 on a selected day.
            self.days.contains(&weekday) && (start..end).contains(&minute_of_day)
        } else {
            // Overnight window: 22:00–06:00. The evening half belongs to the
            // selected day; the morning half belongs to the day after it, so
            // a Friday night shift runs into Saturday morning.
            if minute_of_day >= start {
                self.days.contains(&weekday)
            } else if minute_of_day < end {
                self.days.contains(&prev_weekday(weekday))
            } else {
                false
            }
        }
    }

    pub fn allows_now(&self) -> bool {
        let now = Local::now();
        let weekday = now.weekday().number_from_monday() as u8;
        self.allows(weekday, now.hour() * 60 + now.minute())
    }
}

fn prev_weekday(weekday: u8) -> u8 {
    if weekday <= 1 {
        7
    } else {
        weekday - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn weekdays_9_to_6() -> WorkHours {
        WorkHours {
            enabled: true,
            start: "09:00".into(),
            end: "18:00".into(),
            days: vec![1, 2, 3, 4, 5],
        }
    }

    const MON: u8 = 1;
    const FRI: u8 = 5;
    const SAT: u8 = 6;
    const SUN: u8 = 7;

    fn at(h: u32, m: u32) -> u32 {
        h * 60 + m
    }

    #[test]
    fn disabled_allows_everything() {
        let wh = WorkHours::default();
        assert!(wh.allows(SAT, at(23, 0)));
        assert!(wh.allows(SUN, at(3, 0)));
    }

    #[test]
    fn inside_working_hours_allows() {
        let wh = weekdays_9_to_6();
        assert!(wh.allows(MON, at(9, 0)));
        assert!(wh.allows(MON, at(13, 30)));
        assert!(wh.allows(FRI, at(17, 59)));
    }

    #[test]
    fn outside_working_hours_blocks() {
        let wh = weekdays_9_to_6();
        assert!(!wh.allows(MON, at(8, 59)));
        assert!(!wh.allows(MON, at(18, 0)), "end is exclusive");
        assert!(!wh.allows(MON, at(23, 0)));
        assert!(!wh.allows(MON, at(0, 0)));
    }

    #[test]
    fn unselected_days_block() {
        let wh = weekdays_9_to_6();
        assert!(!wh.allows(SAT, at(13, 0)));
        assert!(!wh.allows(SUN, at(13, 0)));
    }

    #[test]
    fn overnight_window_wraps_past_midnight() {
        let wh = WorkHours {
            enabled: true,
            start: "22:00".into(),
            end: "06:00".into(),
            days: vec![FRI],
        };
        // Friday evening, and the small hours of Saturday that follow it.
        assert!(wh.allows(FRI, at(23, 0)));
        assert!(wh.allows(SAT, at(2, 0)));
        // Neither the Friday daytime before the shift nor Saturday evening.
        assert!(!wh.allows(FRI, at(12, 0)));
        assert!(!wh.allows(SAT, at(23, 0)));
        // Sunday morning would belong to a Saturday shift, which isn't set.
        assert!(!wh.allows(SUN, at(2, 0)));
    }

    #[test]
    fn overnight_window_wraps_from_sunday_into_monday() {
        let wh = WorkHours {
            enabled: true,
            start: "22:00".into(),
            end: "06:00".into(),
            days: vec![SUN],
        };
        assert!(wh.allows(SUN, at(22, 30)));
        assert!(wh.allows(MON, at(1, 0)), "Sunday shift runs into Monday");
    }

    #[test]
    fn malformed_config_allows_rather_than_silencing() {
        let bad = WorkHours {
            enabled: true,
            start: "not a time".into(),
            end: "18:00".into(),
            days: vec![MON],
        };
        assert!(bad.allows(SAT, at(23, 0)));

        let empty_days = WorkHours {
            enabled: true,
            days: vec![],
            ..weekdays_9_to_6()
        };
        assert!(empty_days.allows(SAT, at(23, 0)));

        let zero_length = WorkHours {
            enabled: true,
            start: "09:00".into(),
            end: "09:00".into(),
            days: vec![MON],
        };
        assert!(zero_length.allows(SAT, at(23, 0)));
    }
}
