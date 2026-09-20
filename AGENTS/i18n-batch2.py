"""Lote 2: fechas (util), schedule, ui/schedule, ui/pane, ui/chats.when_label."""
import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
DATA = ROOT / "AGENTS" / "i18n-strings.json"
data = json.loads(DATA.read_text(encoding="utf-8"))

rows = [
    # Semana y meses (tablas únicas: src/schedule.rs las publica, util.rs las usa).
    ("DateWeekdayMonday", "Monday", "lunes"),
    ("DateWeekdayTuesday", "Tuesday", "martes"),
    ("DateWeekdayWednesday", "Wednesday", "miércoles"),
    ("DateWeekdayThursday", "Thursday", "jueves"),
    ("DateWeekdayFriday", "Friday", "viernes"),
    ("DateWeekdaySaturday", "Saturday", "sábado"),
    ("DateWeekdaySunday", "Sunday", "domingo"),
    ("DateWeekdayAbbrMonday", "Mon", "lun"),
    ("DateWeekdayAbbrTuesday", "Tue", "mar"),
    ("DateWeekdayAbbrWednesday", "Wed", "mié"),
    ("DateWeekdayAbbrThursday", "Thu", "jue"),
    ("DateWeekdayAbbrFriday", "Fri", "vie"),
    ("DateWeekdayAbbrSaturday", "Sat", "sáb"),
    ("DateWeekdayAbbrSunday", "Sun", "dom"),
    ("DateMonthJanuary", "January", "enero"),
    ("DateMonthFebruary", "February", "febrero"),
    ("DateMonthMarch", "March", "marzo"),
    ("DateMonthApril", "April", "abril"),
    ("DateMonthMay", "May", "mayo"),
    ("DateMonthJune", "June", "junio"),
    ("DateMonthJuly", "July", "julio"),
    ("DateMonthAugust", "August", "agosto"),
    ("DateMonthSeptember", "September", "septiembre"),
    ("DateMonthOctober", "October", "octubre"),
    ("DateMonthNovember", "November", "noviembre"),
    ("DateMonthDecember", "December", "diciembre"),
    ("DateMonthAbbrJanuary", "Jan", "ene"),
    ("DateMonthAbbrFebruary", "Feb", "feb"),
    ("DateMonthAbbrMarch", "Mar", "mar"),
    ("DateMonthAbbrApril", "Apr", "abr"),
    ("DateMonthAbbrMay", "May", "may"),
    ("DateMonthAbbrJune", "Jun", "jun"),
    ("DateMonthAbbrJuly", "Jul", "jul"),
    ("DateMonthAbbrAugust", "Aug", "ago"),
    ("DateMonthAbbrSeptember", "Sep", "sep"),
    ("DateMonthAbbrOctober", "Oct", "oct"),
    ("DateMonthAbbrNovember", "Nov", "nov"),
    ("DateMonthAbbrDecember", "Dec", "dic"),
    # Etiquetas relativas y plantillas.
    ("DateToday", "Today", "Hoy"),
    ("DateYesterday", "Yesterday", "Ayer"),
    ("DateYesterdayAt", "Yesterday at {time}", "Ayer a las {time}"),
    ("DateAt", "{moment} at {time}", "{moment} a las {time}"),
    ("DateShort", "{day} {month} {year}", "{day} {month} {year}"),
    ("DateLong", "{weekday}, {day} {month} {year}", "{weekday}, {day} de {month} de {year}"),
    ("DateStampShort", "{weekday} {day} {month}, {time}", "{weekday} {day} {month}, {time}"),
    ("DateShortWeekday", "{weekday} {day} {month}", "{weekday} {day} {month}"),
    ("MonthYear", "{month} {year}", "{month} de {year}"),
    ("CopyStamp", "{time}, {month}/{day}/{year}", "{time}, {day}/{month}/{year}"),
    # Programar mensaje.
    ("ScheduleOnce", "Once", "Una vez"),
    ("ScheduleEveryDay", "Every day", "Todos los días"),
    ("ScheduleEveryWeekday", "Every {weekday}", "Todos los {weekday}"),
    ("ScheduleDayOfMonth", "Day {day} of every month", "El día {day} de cada mes"),
    (
        "ScheduleNthWeekdayOfMonth",
        "{ordinal} {weekday} of every month",
        "El {ordinal} {weekday} de cada mes",
    ),
    ("ScheduleOrdinal1", "1st", "primer"),
    ("ScheduleOrdinal2", "2nd", "segundo"),
    ("ScheduleOrdinal3", "3rd", "tercero"),
    ("ScheduleOrdinal4", "4th", "cuarto"),
    ("ScheduleTitle", "Schedule message", "Programar mensaje"),
    ("ScheduleTimeLabel", "Time", "Hora"),
    ("ScheduleRepeatLabel", "Repeat", "Repetir"),
    ("ScheduleEarlierHour", "Earlier", "Una hora antes"),
    ("ScheduleLaterHour", "Later", "Una hora después"),
    ("ScheduleEarlierMinutes", "Earlier minutes", "Cinco minutos antes"),
    ("ScheduleLaterMinutes", "Later minutes", "Cinco minutos después"),
    ("ScheduleSend", "Schedule", "Programar"),
    ("SchedulePreviousMonth", "Previous month", "Mes anterior"),
    ("ScheduleNextMonth", "Next month", "Mes siguiente"),
    (
        "ScheduleNeedMessage",
        "Write the message first; it goes out at the time you pick.",
        "Escribe el mensaje primero; se envía a la hora que elijas.",
    ),
    ("ScheduleFrom", "{repeat}, from {when}", "{repeat}, desde el {when}"),
    ("DialogCancel", "Cancel", "Cancelar"),
]

existing = {r["key"] for r in data["strings"]}
for key, en, es in rows:
    assert key not in existing, f"duplicate key {key}"
    data["strings"].append({"key": key, "en": en, "es": es})

sites = [
    # ---- src/util.rs
    {
        "file": "src/util.rs",
        "old": "use jiff::civil::Date;\nuse jiff::{Timestamp, Zoned};",
        "new": "use jiff::civil::Date;\nuse jiff::{Timestamp, Zoned};\n\nuse crate::i18n::{self, Key};\nuse crate::schedule::{month_abbr, month_name, weekday_abbr, weekday_name};",
    },
    {
        "file": "src/util.rs",
        "old": """fn weekday_name(weekday: jiff::civil::Weekday) -> &'static str {
    match weekday {
        jiff::civil::Weekday::Monday => "Monday",
        jiff::civil::Weekday::Tuesday => "Tuesday",
        jiff::civil::Weekday::Wednesday => "Wednesday",
        jiff::civil::Weekday::Thursday => "Thursday",
        jiff::civil::Weekday::Friday => "Friday",
        jiff::civil::Weekday::Saturday => "Saturday",
        jiff::civil::Weekday::Sunday => "Sunday",
    }
}

fn month_name(month: i8) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        _ => "December",
    }
}

""",
        "new": "",
    },
    {
        "file": "src/util.rs",
        "old": """            format!(
                "{}:{:02}, {}/{}/{}",
                when.hour(),
                when.minute(),
                when.month(),
                when.day(),
                when.year()
            )""",
        "new": """            let time = format!("{:02}:{:02}", when.hour(), when.minute());
            i18n::f(
                Key::CopyStamp,
                &[
                    ("time", &time),
                    ("day", &when.date().day().to_string()),
                    ("month", &when.date().month().to_string()),
                    ("year", &when.date().year().to_string()),
                ],
            )""",
    },
    {
        "file": "src/util.rs",
        "old": """        1 => format!("Yesterday at {time}"),
        2..=6 => format!("{} at {time}", weekday_name(when.date().weekday())),
        _ => format!("{} at {time}", short_date(when.date())),""",
        "new": """        1 => i18n::f(Key::DateYesterdayAt, &[("time", time.as_str())]),
        2..=6 => i18n::f(
            Key::DateAt,
            &[
                ("moment", weekday_name(when.date().weekday())),
                ("time", time.as_str()),
            ],
        ),
        _ => i18n::f(
            Key::DateAt,
            &[
                ("moment", short_date(when.date()).as_str()),
                ("time", time.as_str()),
            ],
        ),""",
    },
    {
        "file": "src/util.rs",
        "old": "        0 => format!(\"{:02}:{:02}\", when.hour(), when.minute()),\n        1 => \"Yesterday\".to_owned(),",
        "new": "        0 => format!(\"{:02}:{:02}\", when.hour(), when.minute()),\n        1 => i18n::t(Key::DateYesterday).to_owned(),",
    },
    {
        "file": "src/util.rs",
        "old": "        0 => \"Today\".to_owned(),\n        1 => \"Yesterday\".to_owned(),",
        "new": "        0 => i18n::t(Key::DateToday).to_owned(),\n        1 => i18n::t(Key::DateYesterday).to_owned(),",
    },
    {
        "file": "src/util.rs",
        "old": """/// Conversation day-separator label.""",
        "new": """/// Short stamp for lists, such as \"Fri 18 Sep, 21:00\".
pub fn short_stamp(unix_seconds: i64) -> String {
    let Some(when) = zoned(unix_seconds) else {
        return String::new();
    };
    i18n::f(
        Key::DateStampShort,
        &[
            ("weekday", weekday_abbr(when.date().weekday())),
            ("day", &when.date().day().to_string()),
            ("month", month_abbr(when.date().month())),
            ("time", &format!("{:02}:{:02}", when.hour(), when.minute())),
        ],
    )
}

/// Conversation day-separator label.""",
    },
    {
        "file": "src/util.rs",
        "old": """fn short_date(date: Date) -> String {
    format!(
        "{} {} {}",
        date.day(),
        &month_name(date.month())[..3],
        date.year()
    )
}""",
        "new": """fn short_date(date: Date) -> String {
    i18n::f(
        Key::DateShort,
        &[
            ("day", &date.day().to_string()),
            ("month", month_abbr(date.month())),
            ("year", &date.year().to_string()),
        ],
    )
}""",
    },
    {
        "file": "src/util.rs",
        "old": """fn long_date(date: Date) -> String {
    format!(
        "{}, {} {} {}",
        weekday_name(date.weekday()),
        date.day(),
        month_name(date.month()),
        date.year()
    )
}""",
        "new": """fn long_date(date: Date) -> String {
    i18n::f(
        Key::DateLong,
        &[
            ("weekday", weekday_name(date.weekday())),
            ("day", &date.day().to_string()),
            ("month", month_name(date.month())),
            ("year", &date.year().to_string()),
        ],
    )
}""",
    },
    # ---- src/schedule.rs
    {
        "file": "src/schedule.rs",
        "old": "use jiff::Timestamp;\nuse jiff::civil::{Date, Weekday};",
        "new": "use jiff::Timestamp;\nuse jiff::civil::{Date, Weekday};\n\nuse crate::i18n::{self, Key};",
    },
    {
        "file": "src/schedule.rs",
        "old": """/// The English weekday name, as WhatsApp writes it.
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
}""",
        "new": """/// The weekday name in the interface language.
pub fn weekday_name(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Monday => i18n::t(Key::DateWeekdayMonday),
        Weekday::Tuesday => i18n::t(Key::DateWeekdayTuesday),
        Weekday::Wednesday => i18n::t(Key::DateWeekdayWednesday),
        Weekday::Thursday => i18n::t(Key::DateWeekdayThursday),
        Weekday::Friday => i18n::t(Key::DateWeekdayFriday),
        Weekday::Saturday => i18n::t(Key::DateWeekdaySaturday),
        Weekday::Sunday => i18n::t(Key::DateWeekdaySunday),
    }
}

/// The weekday abbreviation, such as \"Fri\" or \"vie\".
pub fn weekday_abbr(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Monday => i18n::t(Key::DateWeekdayAbbrMonday),
        Weekday::Tuesday => i18n::t(Key::DateWeekdayAbbrTuesday),
        Weekday::Wednesday => i18n::t(Key::DateWeekdayAbbrWednesday),
        Weekday::Thursday => i18n::t(Key::DateWeekdayAbbrThursday),
        Weekday::Friday => i18n::t(Key::DateWeekdayAbbrFriday),
        Weekday::Saturday => i18n::t(Key::DateWeekdayAbbrSaturday),
        Weekday::Sunday => i18n::t(Key::DateWeekdayAbbrSunday),
    }
}

/// The month name in the interface language.
pub fn month_name(month: i8) -> &'static str {
    match month {
        1 => i18n::t(Key::DateMonthJanuary),
        2 => i18n::t(Key::DateMonthFebruary),
        3 => i18n::t(Key::DateMonthMarch),
        4 => i18n::t(Key::DateMonthApril),
        5 => i18n::t(Key::DateMonthMay),
        6 => i18n::t(Key::DateMonthJune),
        7 => i18n::t(Key::DateMonthJuly),
        8 => i18n::t(Key::DateMonthAugust),
        9 => i18n::t(Key::DateMonthSeptember),
        10 => i18n::t(Key::DateMonthOctober),
        11 => i18n::t(Key::DateMonthNovember),
        _ => i18n::t(Key::DateMonthDecember),
    }
}

/// The month abbreviation, such as \"Sep\" or \"sep\".
pub fn month_abbr(month: i8) -> &'static str {
    match month {
        1 => i18n::t(Key::DateMonthAbbrJanuary),
        2 => i18n::t(Key::DateMonthAbbrFebruary),
        3 => i18n::t(Key::DateMonthAbbrMarch),
        4 => i18n::t(Key::DateMonthAbbrApril),
        5 => i18n::t(Key::DateMonthAbbrMay),
        6 => i18n::t(Key::DateMonthAbbrJune),
        7 => i18n::t(Key::DateMonthAbbrJuly),
        8 => i18n::t(Key::DateMonthAbbrAugust),
        9 => i18n::t(Key::DateMonthAbbrSeptember),
        10 => i18n::t(Key::DateMonthAbbrOctober),
        11 => i18n::t(Key::DateMonthAbbrNovember),
        _ => i18n::t(Key::DateMonthAbbrDecember),
    }
}""",
    },
    {
        "file": "src/schedule.rs",
        "old": """/// \"1st\", \"2nd\", \"3rd\", \"4th\".
pub fn ordinal(nth: i8) -> &'static str {
    match nth {
        1 => \"1st\",
        2 => \"2nd\",
        3 => \"3rd\",
        _ => \"4th\",
    }
}""",
        "new": """/// \"1st\", \"2nd\", \"3rd\", \"4th\" (\"primer\", \"segundo\", ... in Spanish).
pub fn ordinal(nth: i8) -> &'static str {
    match nth {
        1 => i18n::t(Key::ScheduleOrdinal1),
        2 => i18n::t(Key::ScheduleOrdinal2),
        3 => i18n::t(Key::ScheduleOrdinal3),
        _ => i18n::t(Key::ScheduleOrdinal4),
    }
}""",
    },
    {
        "file": "src/schedule.rs",
        "old": """            Recurrence::Once => "Once".to_owned(),
            Recurrence::Daily => "Every day".to_owned(),
            Recurrence::Weekly { weekday } => format!("Every {}", weekday_name(weekday)),
            Recurrence::Monthly { day } => format!("Day {day} of every month"),
            Recurrence::NthWeekday { nth, weekday } => {
                format!("{} {} of every month", ordinal(nth), weekday_name(weekday))
            }""",
        "new": """            Recurrence::Once => i18n::t(Key::ScheduleOnce).to_owned(),
            Recurrence::Daily => i18n::t(Key::ScheduleEveryDay).to_owned(),
            Recurrence::Weekly { weekday } => i18n::f(
                Key::ScheduleEveryWeekday,
                &[("weekday", weekday_name(weekday))],
            ),
            Recurrence::Monthly { day } => {
                i18n::f(Key::ScheduleDayOfMonth, &[("day", &day.to_string())])
            }
            Recurrence::NthWeekday { nth, weekday } => i18n::f(
                Key::ScheduleNthWeekdayOfMonth,
                &[
                    ("ordinal", ordinal(nth)),
                    ("weekday", weekday_name(weekday)),
                ],
            ),""",
    },
    {
        "file": "src/schedule.rs",
        "old": """            Repeat::Once => "Once".to_owned(),
            Repeat::Daily => "Every day".to_owned(),
            Repeat::Weekly => format!("Every {}", weekday_name(day.weekday())),
            Repeat::Monthly => format!("Day {} of every month", day.day()),
            Repeat::NthWeekday => format!(
                "{} {} of every month",
                ordinal(nth_of(day)),
                weekday_name(day.weekday())
            ),""",
        "new": """            Repeat::Once => i18n::t(Key::ScheduleOnce).to_owned(),
            Repeat::Daily => i18n::t(Key::ScheduleEveryDay).to_owned(),
            Repeat::Weekly => i18n::f(
                Key::ScheduleEveryWeekday,
                &[("weekday", weekday_name(day.weekday()))],
            ),
            Repeat::Monthly => {
                i18n::f(Key::ScheduleDayOfMonth, &[("day", &day.day().to_string())])
            }
            Repeat::NthWeekday => i18n::f(
                Key::ScheduleNthWeekdayOfMonth,
                &[
                    ("ordinal", ordinal(nth_of(day))),
                    ("weekday", weekday_name(day.weekday())),
                ],
            ),""",
    },
    # ---- src/ui/schedule.rs
    {
        "file": "src/ui/schedule.rs",
        "old": "use crate::app::App;\nuse crate::model::Action;\nuse crate::schedule::{Repeat, instant_of, next_after, weekday_from_offset, weekday_name};",
        "new": "use crate::app::App;\nuse crate::i18n::{self, Key};\nuse crate::model::Action;\nuse crate::schedule::{\n    Repeat, instant_of, month_abbr, month_name, next_after, weekday_abbr, weekday_from_offset,\n    weekday_name,\n};",
    },
    {
        "file": "src/ui/schedule.rs",
        "old": "super::dialogs::title(ui, app, \"Schedule message\");",
        "new": "super::dialogs::title(ui, app, i18n::t(Key::ScheduleTitle));",
    },
    { "file": "src/ui/schedule.rs", "old": "\"Previous month\",", "new": "i18n::t(Key::SchedulePreviousMonth)," },
    { "file": "src/ui/schedule.rs", "old": "\"Next month\",", "new": "i18n::t(Key::ScheduleNextMonth)," },
    {
        "file": "src/ui/schedule.rs",
        "old": "theme::text(ui, \"Time\", theme::medium(13.5), palette.text);",
        "new": "theme::text(ui, i18n::t(Key::ScheduleTimeLabel), theme::medium(13.5), palette.text);",
    },
    { "file": "src/ui/schedule.rs", "old": "            \"Earlier\",", "new": "            i18n::t(Key::ScheduleEarlierHour)," },
    { "file": "src/ui/schedule.rs", "old": "            \"Later\",", "new": "            i18n::t(Key::ScheduleLaterHour)," },
    { "file": "src/ui/schedule.rs", "old": "            \"Earlier minutes\",", "new": "            i18n::t(Key::ScheduleEarlierMinutes)," },
    { "file": "src/ui/schedule.rs", "old": "            \"Later minutes\",", "new": "            i18n::t(Key::ScheduleLaterMinutes)," },
    {
        "file": "src/ui/schedule.rs",
        "old": "theme::text(ui, \"Repeat\", theme::medium(13.5), palette.text);",
        "new": "theme::text(ui, i18n::t(Key::ScheduleRepeatLabel), theme::medium(13.5), palette.text);",
    },
    {
        "file": "src/ui/schedule.rs",
        "old": "theme::pill_button(ui, palette, \"Schedule\", true)",
        "new": "theme::pill_button(ui, palette, i18n::t(Key::ScheduleSend), true)",
    },
    {
        "file": "src/ui/schedule.rs",
        "old": "theme::soft_button(ui, palette, None, \"Cancel\", false)",
        "new": "theme::soft_button(ui, palette, None, i18n::t(Key::DialogCancel), false)",
    },
    {
        "file": "src/ui/schedule.rs",
        "old": "        \"Write the message first; it goes out at the time you pick.\",",
        "new": "        i18n::t(Key::ScheduleNeedMessage),",
    },
    {
        "file": "src/ui/schedule.rs",
        "old": """    let when = format!("{} at {:02}:{:02}", day_label(day), hour, minute);
    match repeat {
        Repeat::Once => when,
        repeat => format!("{}, from {when}", repeat.label(day)),
    }""",
        "new": """    let when = i18n::f(
        Key::DateAt,
        &[
            ("moment", &day_label(day)),
            ("time", &format!("{hour:02}:{minute:02}")),
        ],
    );
    match repeat {
        Repeat::Once => when,
        repeat => i18n::f(
            Key::ScheduleFrom,
            &[("repeat", &repeat.label(day)), ("when", &when)],
        ),
    }""",
    },
    {
        "file": "src/ui/schedule.rs",
        "old": """/// \"Fri 18 Sep\" for the picked day.
fn day_label(day: Date) -> String {
    day.strftime(\"%a %d %b\").to_string()
}""",
        "new": """/// \"Fri 18 Sep\" for the picked day.
fn day_label(day: Date) -> String {
    i18n::f(
        Key::DateShortWeekday,
        &[
            ("weekday", weekday_abbr(day.weekday())),
            ("day", &day.day().to_string()),
            ("month", month_abbr(day.month())),
        ],
    )
}""",
    },
    {
        "file": "src/ui/schedule.rs",
        "old": """fn month_label(month: Date) -> String {
    const NAMES: [&str; 12] = [
        \"January\",
        \"February\",
        \"March\",
        \"April\",
        \"May\",
        \"June\",
        \"July\",
        \"August\",
        \"September\",
        \"October\",
        \"November\",
        \"December\",
    ];
    let name = NAMES
        .get(usize::try_from(month.month() - 1).unwrap_or(0))
        .copied()
        .unwrap_or(\"January\");
    format!(\"{name} {}\", month.year())
}""",
        "new": """fn month_label(month: Date) -> String {
    i18n::f(
        Key::MonthYear,
        &[
            (\"month\", month_name(month.month())),
            (\"year\", &month.year().to_string()),
        ],
    )
}""",
    },
    # ---- src/ui/pane.rs
    {
        "file": "src/ui/pane.rs",
        "old": "use crate::schedule::{weekday_from_offset, weekday_name};",
        "new": "use crate::i18n::{self, Key};\nuse crate::schedule::{month_name, weekday_from_offset, weekday_name};",
    },
    {
        "file": "src/ui/pane.rs",
        "old": """fn month_label(month: Date) -> String {
    const NAMES: [&str; 12] = [
        \"January\",
        \"February\",
        \"March\",
        \"April\",
        \"May\",
        \"June\",
        \"July\",
        \"August\",
        \"September\",
        \"October\",
        \"November\",
        \"December\",
    ];
    let name = NAMES
        .get(usize::try_from(month.month() - 1).unwrap_or(0))
        .copied()
        .unwrap_or(\"January\");
    format!(\"{name} {}\", month.year())
}""",
        "new": """fn month_label(month: Date) -> String {
    i18n::f(
        Key::MonthYear,
        &[
            (\"month\", month_name(month.month())),
            (\"year\", &month.year().to_string()),
        ],
    )
}""",
    },
    # ---- src/ui/chats.rs
    {
        "file": "src/ui/chats.rs",
        "old": """/// \"Fri 18 Sep, 21:00\" in the machine's time zone.
fn when_label(instant: i64) -> String {
    jiff::Timestamp::from_second(instant)
        .ok()
        .map(|moment| moment.to_zoned(jiff::tz::TimeZone::system()))
        .map(|moment| moment.strftime(\"%a %d %b, %H:%M\").to_string())
        .unwrap_or_default()
}""",
        "new": """/// \"Fri 18 Sep, 21:00\" in the machine's time zone.
fn when_label(instant: i64) -> String {
    crate::util::short_stamp(instant)
}""",
    },
]

data["sites"].extend(sites)
DATA.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
print(f"rows: {len(data['strings'])}, sites: {len(data['sites'])}")
