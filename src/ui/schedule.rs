//! The schedule dialog: a month calendar with its own header on top, an hour,
//! and whether the message repeats.
//!
//! Everything is stacked in one column: the month header spans the dialog, the
//! grid sits under it, and the controls follow. A side-by-side layout leaves
//! the header looking detached from the calendar it moves.

use egui::{Sense, Vec2, pos2, vec2};
use jiff::civil::Date;

use crate::app::App;
use crate::model::Action;
use crate::schedule::{Repeat, instant_of, next_after, weekday_from_offset, weekday_name};
use crate::theme::{self, Icon, Palette};

/// Side of a day cell in the grid.
const CELL: f32 = 30.0;

/// The width this dialog asks for. The modal sizes its box with it, so the
/// centred parts below can rely on the same number.
pub const WIDTH: f32 = 460.0;

/// The modal's inner margin on both sides.
const INSET: f32 = 44.0;

pub fn show(app: &mut App, ui: &mut egui::Ui, chat: &str) {
    let palette = app.palette;
    super::dialogs::title(ui, app, "Schedule message");
    ui.add_space(10.0);
    month_header(app, ui, &palette);
    ui.add_space(6.0);
    grid(app, ui, &palette);
    ui.add_space(12.0);
    time_row(app, ui, &palette);
    ui.add_space(10.0);
    repeat_row(app, ui, &palette);
    ui.add_space(14.0);
    buttons(app, ui, &palette, chat);
}

/// The month's name with the chevrons that move it, centred over the grid.
fn month_header(app: &mut App, ui: &mut egui::Ui, palette: &Palette) {
    let month = app.schedule_month;
    let galley = ui
        .painter()
        .layout_no_wrap(month_label(month), theme::medium(14.5), palette.text);
    let row = 18.0 + 8.0 + galley.size().x + 8.0 + 18.0;
    ui.horizontal(|ui| {
        ui.add_space(((WIDTH - INSET - row) / 2.0).max(0.0));
        if theme::icon_button(
            ui,
            Icon::ChevronUp,
            18.0,
            palette.secondary,
            palette.text,
            "Previous month",
        )
        .clicked()
        {
            app.schedule_month = month_step(month, -1);
        }
        ui.add_space(8.0);
        theme::text(ui, month_label(month), theme::medium(14.5), palette.text);
        ui.add_space(8.0);
        if theme::icon_button(
            ui,
            Icon::ChevronDown,
            18.0,
            palette.secondary,
            palette.text,
            "Next month",
        )
        .clicked()
        {
            app.schedule_month = month_step(month, 1);
        }
    });
}

/// The month itself: a weekday header and the days, seven to a row. The cells
/// are placed by hand, because a plain row would put every day on one line.
fn grid(app: &mut App, ui: &mut egui::Ui, palette: &Palette) {
    let month = app.schedule_month;
    let block = CELL * 7.0;
    let head = 18.0;
    let height = head + 4.0 + CELL * 6.0;
    let (rect, _) = ui.allocate_exact_size(vec2(WIDTH - INSET, height), Sense::hover());
    if !ui.is_rect_visible(rect) {
        return;
    }
    let left = rect.center().x - block / 2.0;
    let top = rect.top() + head + 4.0;
    for offset in 0..7 {
        let name = weekday_name(weekday_from_offset(offset));
        ui.painter().text(
            pos2(
                left + CELL * offset as f32 + CELL / 2.0,
                rect.top() + head / 2.0,
            ),
            egui::Align2::CENTER_CENTER,
            &name[..2],
            theme::regular(11.0),
            palette.dim,
        );
    }
    let first = Date::new(month.year(), month.month(), 1).unwrap_or(month);
    let lead = usize::try_from(first.weekday().to_monday_zero_offset()).unwrap_or(0);
    for day in 1..=first.days_in_month() {
        let date = Date::new(month.year(), month.month(), day).unwrap_or(first);
        let index = lead + usize::from(day as u8 - 1);
        let cell = egui::Rect::from_center_size(
            pos2(
                left + CELL * (index % 7) as f32 + CELL / 2.0,
                top + CELL * (index / 7) as f32 + CELL / 2.0,
            ),
            Vec2::splat(CELL),
        );
        let response = ui.interact(
            cell,
            egui::Id::new(("schedule-day", month.year(), month.month(), day)),
            Sense::click(),
        );
        let selected = date == app.schedule_day;
        if selected {
            ui.painter()
                .circle_filled(cell.center(), CELL / 2.0 - 1.0, palette.accent);
        } else if response.hovered() {
            ui.painter()
                .circle_filled(cell.center(), CELL / 2.0 - 1.0, palette.surface_hover);
        }
        let colour = if selected {
            palette.on_accent
        } else {
            palette.text
        };
        let galley = ui
            .painter()
            .layout_no_wrap(day.to_string(), theme::regular(13.0), colour);
        ui.painter()
            .galley(cell.center() - galley.size() / 2.0, galley, colour);
        if response.clicked() {
            app.schedule_day = date;
        }
        response.on_hover_cursor(egui::CursorIcon::PointingHand);
    }
}

/// Hour and minute, each with a minus and a plus.
fn time_row(app: &mut App, ui: &mut egui::Ui, palette: &Palette) {
    ui.horizontal(|ui| {
        ui.add_space(6.0);
        theme::text(ui, "Time", theme::medium(13.5), palette.text);
        ui.add_space(12.0);
        let hour = app.schedule_hour;
        if theme::icon_button(
            ui,
            Icon::Minus,
            16.0,
            palette.secondary,
            palette.text,
            "Earlier",
        )
        .clicked()
        {
            app.schedule_hour = wrap(hour - 1, 24);
        }
        theme::text(
            ui,
            format!("{:02}", app.schedule_hour),
            theme::medium(16.0),
            palette.text,
        );
        if theme::icon_button(
            ui,
            Icon::Plus,
            16.0,
            palette.secondary,
            palette.text,
            "Later",
        )
        .clicked()
        {
            app.schedule_hour = wrap(hour + 1, 24);
        }
        ui.add_space(16.0);
        let minute = app.schedule_minute;
        if theme::icon_button(
            ui,
            Icon::Minus,
            16.0,
            palette.secondary,
            palette.text,
            "Earlier minutes",
        )
        .clicked()
        {
            app.schedule_minute = wrap(minute - 5, 60);
        }
        theme::text(
            ui,
            format!("{:02}", app.schedule_minute),
            theme::medium(16.0),
            palette.text,
        );
        if theme::icon_button(
            ui,
            Icon::Plus,
            16.0,
            palette.secondary,
            palette.text,
            "Later minutes",
        )
        .clicked()
        {
            app.schedule_minute = wrap(minute + 5, 60);
        }
    });
}

/// The recurrence dropdown, worded for the day that is picked.
fn repeat_row(app: &mut App, ui: &mut egui::Ui, palette: &Palette) {
    let day = app.schedule_day;
    ui.horizontal(|ui| {
        ui.add_space(6.0);
        theme::text(ui, "Repeat", theme::medium(13.5), palette.text);
        ui.add_space(8.0);
        let width = (WIDTH - INSET - 6.0 - 62.0 - 8.0).max(120.0);
        egui::ComboBox::from_id_salt("schedule-repeat")
            .width(width)
            .selected_text(app.schedule_repeat.label(day))
            .show_ui(ui, |ui| {
                for repeat in Repeat::ALL {
                    ui.selectable_value(&mut app.schedule_repeat, repeat, repeat.label(day));
                }
            });
    });
}

/// Cancel and Schedule, with what is about to happen said plainly.
fn buttons(app: &mut App, ui: &mut egui::Ui, palette: &Palette, chat: &str) {
    let day = app.schedule_day;
    let hour = app.schedule_hour;
    let minute = app.schedule_minute;
    let ready = !app.composer.trim().is_empty();
    let mut schedule = false;
    let line = summary(day, hour, minute, app.schedule_repeat);
    let galley = ui
        .painter()
        .layout_no_wrap(line.clone(), theme::regular(12.0), palette.secondary);
    let rest = (WIDTH - INSET - 6.0 - galley.size().x - 8.0 - 190.0).max(0.0);
    ui.horizontal(|ui| {
        ui.add_space(6.0);
        theme::text(ui, line, theme::regular(12.0), palette.secondary);
        ui.add_space(rest);
        if theme::pill_button(ui, palette, "Schedule", true).clicked() && ready {
            schedule = true;
        }
        if theme::soft_button(ui, palette, None, "Cancel", false).clicked() {
            app.actions.push(Action::CloseDialog);
        }
    });
    if !ready {
        ui.add_space(4.0);
        theme::text(
            ui,
            "Write the message first; it goes out at the time you pick.",
            theme::regular(11.5),
            palette.dim,
        );
        return;
    }
    if schedule {
        let recurrence = app.schedule_repeat.recurrence(day);
        let next_at = match app.schedule_repeat {
            Repeat::Once => instant_of(day, hour, minute),
            _ => next_after(recurrence, hour, minute, crate::util::now()),
        };
        if let Some(next_at) = next_at {
            app.actions.push(Action::ScheduleText {
                chat: chat.to_owned(),
                text: app.composer.trim().to_owned(),
                kind: recurrence.key().to_owned(),
                hour,
                minute,
                weekday: recurrence.weekday(),
                day_of_month: recurrence.day_of_month(),
                nth: recurrence.nth(),
                next_at,
            });
        }
    }
}

/// "Once, Fri 18 Sep at 21:00" or the repeat rule with its first time.
fn summary(day: Date, hour: i8, minute: i8, repeat: Repeat) -> String {
    let when = format!("{} at {:02}:{:02}", day_label(day), hour, minute);
    match repeat {
        Repeat::Once => when,
        repeat => format!("{}, from {when}", repeat.label(day)),
    }
}

/// "Fri 18 Sep" for the picked day.
fn day_label(day: Date) -> String {
    day.strftime("%a %d %b").to_string()
}

/// Wraps a value into `0..span`, so stepping past the end starts over.
fn wrap(value: i8, span: i8) -> i8 {
    value.rem_euclid(span)
}

fn month_step(month: Date, direction: i32) -> Date {
    let (year, number) = if direction < 0 {
        if month.month() == 1 {
            (month.year() - 1, 12)
        } else {
            (month.year(), month.month() - 1)
        }
    } else if month.month() == 12 {
        (month.year() + 1, 1)
    } else {
        (month.year(), month.month() + 1)
    };
    Date::new(year, number, 1).unwrap_or(month)
}

fn month_label(month: Date) -> String {
    const NAMES: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let name = NAMES
        .get(usize::try_from(month.month() - 1).unwrap_or(0))
        .copied()
        .unwrap_or("January");
    format!("{name} {}", month.year())
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::Weekday;

    #[test]
    fn months_step_across_the_year_boundary() {
        let december = Date::new(2026, 12, 1).expect("a date");
        assert_eq!(month_step(december, 1).month(), 1);
        assert_eq!(month_step(december, 1).year(), 2027);
        let january = Date::new(2026, 1, 1).expect("a date");
        assert_eq!(month_step(january, -1).month(), 12);
        assert_eq!(month_step(january, -1).year(), 2025);
    }

    #[test]
    fn the_month_name_is_written_out() {
        assert_eq!(
            month_label(Date::new(2026, 9, 1).expect("a date")),
            "September 2026"
        );
        assert_eq!(
            month_label(Date::new(2027, 1, 1).expect("a date")),
            "January 2027"
        );
    }

    #[test]
    fn the_hour_and_minute_wrap_around() {
        assert_eq!(wrap(24, 24), 0);
        assert_eq!(wrap(-1, 24), 23);
        assert_eq!(wrap(60, 60), 0);
        assert_eq!(wrap(-5, 60), 55);
    }

    #[test]
    fn the_summary_says_what_will_happen() {
        let day = Date::new(2026, 9, 18).expect("a date");
        assert_eq!(summary(day, 21, 0, Repeat::Once), "Fri 18 Sep at 21:00");
        assert_eq!(
            summary(day, 9, 30, Repeat::Weekly),
            "Every Friday, from Fri 18 Sep at 09:30"
        );
        assert_eq!(
            Repeat::Weekly.recurrence(day),
            crate::schedule::Recurrence::Weekly {
                weekday: Weekday::Friday
            }
        );
    }
}
