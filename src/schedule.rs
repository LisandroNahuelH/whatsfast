//! When a scheduled message fires next.
//!
//! Every recurrence is a wall-clock rule resolved in the system's time zone,
//! so "every day at 09:00" stays at 09:00 across a daylight-saving change.
//! The next instant is always recomputed from the rule, never by adding a
//! day or a month to the previous occurrence: a month added to the 31st
//! degrades to the 30th and then to the 28th for good.

use jiff::Timestamp;
use jiff::civil::{Date, Weekday};
use jiff::tz::TimeZone;

/// How a scheduled message repeats.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Recurrence {
    Once,
    Daily,
    Weekly {
        weekday: Weekday,
    },
    Monthly {
        day: i8,
    },
    /// The first to fourth `weekday` of the month, like "the first Monday".
    NthWeekday {
        nth: i8,
        weekday: Weekday,
    },
}

impl Recurrence {
    /// The stored kind, as the archive keeps it.
    pub fn key(self) -> &'static str {
        match self {
            Recurrence::Once => "once",
            Recurrence::Daily => "daily",
            Recurrence::Weekly { .. } => "weekly",
            Recurrence::Monthly { .. } => "monthly",
            Recurrence::NthWeekday { .. } => "nth_weekday",
        }
    }

    /// The label the picker shows, in English like the rest of the app.
    pub fn label(self) -> String {
        match self {
            Recurrence::Once => "Once".to_owned(),
            Recurrence::Daily => "Every day".to_owned(),
            Recurrence::Weekly { weekday } => format!("Every {}", weekday_name(weekday)),
            Recurrence::Monthly { day } => format!("Day {day} of every month"),
            Recurrence::NthWeekday { nth, weekday } => {
                format!("{} {} of every month", ordinal(nth), weekday_name(weekday))
            }
        }
    }

    /// The stored weekday, for the recurrences that carry one.
    pub fn weekday(self) -> Option<i8> {
        match self {
            Recurrence::Weekly { weekday } | Recurrence::NthWeekday { weekday, .. } => {
                Some(weekday.to_monday_zero_offset())
            }
            _ => None,
        }
    }

    /// The stored day of the month, for the recurrences that carry one.
    pub fn day_of_month(self) -> Option<i8> {
        match self {
            Recurrence::Monthly { day } => Some(day),
            _ => None,
        }
    }

    /// The stored ordinal, for the recurrences that carry one.
    pub fn nth(self) -> Option<i8> {
        match self {
            Recurrence::NthWeekday { nth, .. } => Some(nth),
            _ => None,
        }
    }
}

/// The English weekday name, as WhatsApp writes it.
pub fn weekday_name(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Monday => "Monday",
        Weekday::Tuesday => "Tuesday",
        Weekday::Wednesday => "Wednesday",
        Weekday::Thursday => "Thursday",
        Weekday::Friday => "Friday",
        Weekday::Saturday => "Saturday",
        Weekday::Sunday => "Sunday",
    }
}

/// The weekday from its stored offset (0 = Monday).
pub fn weekday_from_offset(offset: i8) -> Weekday {
    match offset.rem_euclid(7) {
        0 => Weekday::Monday,
        1 => Weekday::Tuesday,
        2 => Weekday::Wednesday,
        3 => Weekday::Thursday,
        4 => Weekday::Friday,
        5 => Weekday::Saturday,
        _ => Weekday::Sunday,
    }
}

/// "1st", "2nd", "3rd", "4th".
pub fn ordinal(nth: i8) -> &'static str {
    match nth {
        1 => "1st",
        2 => "2nd",
        3 => "3rd",
        _ => "4th",
    }
}

/// What the schedule picker offers. The day the user picks supplies the
/// weekday and the day of the month, so the labels read like WhatsApp's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Repeat {
    Once,
    Daily,
    Weekly,
    Monthly,
    NthWeekday,
}

impl Repeat {
    pub const ALL: [Repeat; 5] = [
        Repeat::Once,
        Repeat::Daily,
        Repeat::Weekly,
        Repeat::Monthly,
        Repeat::NthWeekday,
    ];

    /// The label for a given day, like "Every Monday" or "2nd Tuesday of every
    /// month".
    pub fn label(self, day: Date) -> String {
        match self {
            Repeat::Once => "Once".to_owned(),
            Repeat::Daily => "Every day".to_owned(),
            Repeat::Weekly => format!("Every {}", weekday_name(day.weekday())),
            Repeat::Monthly => format!("Day {} of every month", day.day()),
            Repeat::NthWeekday => format!(
                "{} {} of every month",
                ordinal(nth_of(day)),
                weekday_name(day.weekday())
            ),
        }
    }

    /// The rule this becomes for the day picked.
    pub fn recurrence(self, day: Date) -> Recurrence {
        match self {
            Repeat::Once => Recurrence::Once,
            Repeat::Daily => Recurrence::Daily,
            Repeat::Weekly => Recurrence::Weekly {
                weekday: day.weekday(),
            },
            Repeat::Monthly => Recurrence::Monthly { day: day.day() },
            Repeat::NthWeekday => Recurrence::NthWeekday {
                nth: nth_of(day),
                weekday: day.weekday(),
            },
        }
    }
}

/// Which time of the month a day is: the 15th is the third week.
fn nth_of(day: Date) -> i8 {
    (day.day() - 1) / 7 + 1
}

/// The instant a one-off fires at: the chosen day at the chosen time, in the
/// system's time zone.
pub fn instant_of(day: Date, hour: i8, minute: i8) -> Option<i64> {
    day.at(hour, minute, 0, 0)
        .to_zoned(TimeZone::system())
        .ok()
        .map(|moment| moment.timestamp().as_second())
}

/// The recurrence a stored entry describes.
pub fn recurrence(
    kind: &str,
    weekday: Option<i8>,
    day_of_month: Option<i8>,
    nth: Option<i8>,
) -> Option<Recurrence> {
    match kind {
        "once" => Some(Recurrence::Once),
        "daily" => Some(Recurrence::Daily),
        "weekly" => Some(Recurrence::Weekly {
            weekday: weekday_from_offset(weekday?),
        }),
        "monthly" => Some(Recurrence::Monthly { day: day_of_month? }),
        "nth_weekday" => Some(Recurrence::NthWeekday {
            nth: nth?,
            weekday: weekday_from_offset(weekday?),
        }),
        _ => None,
    }
}

/// The instant, in Unix seconds, of the next occurrence strictly after `from`.
///
/// `None` for a one-off (it has its own instant) and when the calendar refuses
/// a date the rule asks for.
pub fn next_after(recurrence: Recurrence, hour: i8, minute: i8, from: i64) -> Option<i64> {
    next_after_in(&TimeZone::system(), recurrence, hour, minute, from)
}

/// Like [`next_after`], with the time zone spelled out so tests do not depend
/// on the machine.
pub fn next_after_in(
    tz: &TimeZone,
    recurrence: Recurrence,
    hour: i8,
    minute: i8,
    from: i64,
) -> Option<i64> {
    let now = Timestamp::from_second(from).ok()?.to_zoned(tz.clone());
    let at = |date: Date| {
        date.at(hour, minute, 0, 0)
            .to_zoned(tz.clone())
            .ok()
            .map(|moment| moment.timestamp().as_second())
    };
    match recurrence {
        Recurrence::Once => None,
        Recurrence::Daily => {
            let today = now.date();
            let candidate = at(today)?;
            if candidate > from {
                return Some(candidate);
            }
            at(today.tomorrow().ok()?)
        }
        Recurrence::Weekly { weekday } => {
            let today = now.date();
            let delta = (weekday.to_monday_zero_offset() - today.weekday().to_monday_zero_offset())
                .rem_euclid(7);
            let day = today
                .checked_add(jiff::Span::new().days(i64::from(delta)))
                .ok()?;
            let candidate = at(day)?;
            if candidate > from {
                return Some(candidate);
            }
            at(day.checked_add(jiff::Span::new().days(7)).ok()?)
        }
        Recurrence::Monthly { day } => {
            let mut month = first_of_month(now.date())?;
            for _ in 0..2 {
                let date = Date::new(
                    month.year(),
                    month.month(),
                    day.clamp(1, month.days_in_month()),
                )
                .ok()?;
                let candidate = at(date)?;
                if candidate > from {
                    return Some(candidate);
                }
                month = next_month(month)?;
            }
            None
        }
        Recurrence::NthWeekday { nth, weekday } => {
            let mut month = first_of_month(now.date())?;
            for _ in 0..2 {
                let date = month.nth_weekday_of_month(nth, weekday).ok()?;
                let candidate = at(date)?;
                if candidate > from {
                    return Some(candidate);
                }
                month = next_month(month)?;
            }
            None
        }
    }
}

fn first_of_month(date: Date) -> Option<Date> {
    Date::new(date.year(), date.month(), 1).ok()
}

fn next_month(date: Date) -> Option<Date> {
    let (year, month) = if date.month() == 12 {
        (date.year() + 1, 1)
    } else {
        (date.year(), date.month() + 1)
    };
    Date::new(year, month, 1).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at_utc() -> TimeZone {
        TimeZone::UTC
    }

    /// An instant, written as the date and hour a person would say.
    fn utc(year: i16, month: i8, day: i8, hour: i8) -> i64 {
        Date::new(year, month, day)
            .expect("a date")
            .at(hour, 0, 0, 0)
            .to_zoned(at_utc())
            .expect("a moment")
            .timestamp()
            .as_second()
    }

    fn next(recurrence: Recurrence, hour: i8, minute: i8, from: i64) -> Option<i64> {
        next_after_in(&at_utc(), recurrence, hour, minute, from)
    }

    /// The local date and time of an instant, for readable assertions.
    fn stamp(instant: i64) -> String {
        Timestamp::from_second(instant)
            .expect("a stamp")
            .to_zoned(at_utc())
            .strftime("%Y-%m-%d %H:%M")
            .to_string()
    }

    /// Friday 2026-09-18, noon.
    fn noon() -> i64 {
        utc(2026, 9, 18, 12)
    }

    #[test]
    fn daily_keeps_the_wall_clock_time() {
        // Later today.
        assert_eq!(
            stamp(next(Recurrence::Daily, 20, 30, noon()).expect("a next")),
            "2026-09-18 20:30"
        );
        // Already past, so tomorrow.
        assert_eq!(
            stamp(next(Recurrence::Daily, 9, 0, noon()).expect("a next")),
            "2026-09-19 09:00"
        );
    }

    #[test]
    fn weekly_lands_on_its_weekday() {
        // Today is Friday: the next Monday is the 21st.
        let monday = Recurrence::Weekly {
            weekday: Weekday::Monday,
        };
        assert_eq!(
            stamp(next(monday, 9, 0, noon()).expect("a next")),
            "2026-09-21 09:00"
        );
        // The same Friday, later in the day, still fires today.
        let friday = Recurrence::Weekly {
            weekday: Weekday::Friday,
        };
        assert_eq!(
            stamp(next(friday, 20, 0, noon()).expect("a next")),
            "2026-09-18 20:00"
        );
    }

    #[test]
    fn monthly_clamps_short_months_and_recovers() {
        // The 31st asked in January is the 31st; asked in February it lands on
        // the 28th, and March is the 31st again: the rule is recomputed, not
        // chained, so the day never degrades.
        let january = Recurrence::Monthly { day: 31 };
        assert_eq!(
            stamp(next(january, 9, 0, utc(2026, 1, 15, 12)).expect("a next")),
            "2026-01-31 09:00"
        );
        assert_eq!(
            stamp(next(january, 9, 0, utc(2026, 2, 10, 12)).expect("a next")),
            "2026-02-28 09:00"
        );
        assert_eq!(
            stamp(next(january, 9, 0, utc(2026, 3, 10, 12)).expect("a next")),
            "2026-03-31 09:00"
        );
    }

    #[test]
    fn nth_weekday_uses_the_ordinal_of_the_month() {
        // The first Monday after Friday the 18th is the 5th of October.
        let first_monday = Recurrence::NthWeekday {
            nth: 1,
            weekday: Weekday::Monday,
        };
        assert_eq!(
            stamp(next(first_monday, 9, 0, noon()).expect("a next")),
            "2026-10-05 09:00"
        );
        // The third Friday of this month is today, later in the day.
        let third_friday = Recurrence::NthWeekday {
            nth: 3,
            weekday: Weekday::Friday,
        };
        assert_eq!(
            stamp(next(third_friday, 20, 0, noon()).expect("a next")),
            "2026-09-18 20:00"
        );
    }

    #[test]
    fn a_one_off_has_no_next_occurrence() {
        assert_eq!(next(Recurrence::Once, 9, 0, noon()), None);
    }

    #[test]
    fn the_repeat_labels_read_the_picked_day() {
        let day = Date::new(2026, 9, 18).expect("a date");
        assert_eq!(Repeat::Once.label(day), "Once");
        assert_eq!(Repeat::Daily.label(day), "Every day");
        assert_eq!(Repeat::Weekly.label(day), "Every Friday");
        assert_eq!(Repeat::Monthly.label(day), "Day 18 of every month");
        assert_eq!(Repeat::NthWeekday.label(day), "3rd Friday of every month");
        // The rule keeps the day the label named.
        assert_eq!(
            Repeat::NthWeekday.recurrence(day),
            Recurrence::NthWeekday {
                nth: 3,
                weekday: Weekday::Friday
            }
        );
    }

    #[test]
    fn recurrence_labels_read_like_whatsapp() {
        assert_eq!(Recurrence::Daily.label(), "Every day");
        assert_eq!(
            Recurrence::NthWeekday {
                nth: 2,
                weekday: Weekday::Tuesday
            }
            .label(),
            "2nd Tuesday of every month"
        );
        assert_eq!(
            Recurrence::Monthly { day: 15 }.label(),
            "Day 15 of every month"
        );
    }
}
