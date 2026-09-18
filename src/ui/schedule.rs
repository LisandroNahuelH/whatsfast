//! The schedule dialog: pick a day in a small calendar, an hour, and whether
//! the message repeats.

use egui::{Align, Layout, Sense, Vec2};
use jiff::civil::Date;

use crate::app::App;
use crate::model::Action;
use crate::schedule::{Repeat, instant_of, next_after, weekday_from_offset, weekday_name};
use crate::theme::{self, Icon, Palette};

pub fn show(app: &mut App, ui: &mut egui::Ui, chat: &str) {
    let palette = app.palette;
    super::dialogs::title(ui, app, "Schedule message");
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        calendar(app, ui, &palette);
        ui.add_space(20.0);
        ui.vertical(|ui| {
            time_picker(app, ui, &palette);
            ui.add_space(12.0);
            repeat_picker(app, ui, &palette);
        });
    });
    ui.add_space(14.0);
    let ready = !app.composer.trim().is_empty();
    let mut schedule = None;
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        if theme::pill_button(ui, &palette, "Schedule", true).clicked() && ready {
            schedule = Some(());
        }
        if theme::soft_button(ui, &palette, None, "Cancel", false).clicked() {
            app.actions.push(Action::CloseDialog);
        }
    });
    if schedule.is_some() {
        let day = app.schedule_day;
        let hour = app.schedule_hour;
        let minute = app.schedule_minute;
        let recurrence = app.schedule_repeat.recurrence(day);
        let now = crate::util::now();
        let next_at = match app.schedule_repeat {
            Repeat::Once => instant_of(day, hour, minute),
            _ => next_after(recurrence, hour, minute, now),
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

/// The month grid: the weekdays on top, the days of the month below, and the
/// chevrons moving a month at a time.
fn calendar(app: &mut App, ui: &mut egui::Ui, palette: &Palette) {
    let month = app.schedule_month;
    ui.horizontal(|ui| {
        if theme::icon_button(
            ui,
            Icon::ChevronUp,
            16.0,
            palette.secondary,
            palette.text,
            "Previous month",
        )
        .clicked()
        {
            app.schedule_month = month_step(month, -1);
        }
        ui.add_space(6.0);
        theme::text(ui, month_label(month), theme::medium(13.5), palette.text);
        ui.add_space(6.0);
        if theme::icon_button(
            ui,
            Icon::ChevronDown,
            16.0,
            palette.secondary,
            palette.text,
            "Next month",
        )
        .clicked()
        {
            app.schedule_month = month_step(month, 1);
        }
    });
    ui.add_space(6.0);
    egui::Grid::new("schedule-calendar")
        .num_columns(7)
        .spacing([3.0, 3.0])
        .show(ui, |ui| {
            for offset in 0..7 {
                let name = weekday_name(weekday_from_offset(offset));
                theme::text(ui, &name[..2], theme::regular(11.0), palette.dim);
            }
            ui.end_row();
            let first = Date::new(month.year(), month.month(), 1).unwrap_or(month);
            let lead = usize::try_from(first.weekday().to_monday_zero_offset()).unwrap_or(0);
            for _ in 0..lead {
                ui.label("");
            }
            let mut column = lead;
            for day in 1..=first.days_in_month() {
                let date = Date::new(month.year(), month.month(), day).unwrap_or(first);
                let selected = date == app.schedule_day;
                let (rect, response) = ui.allocate_exact_size(Vec2::splat(26.0), Sense::click());
                if ui.is_rect_visible(rect) {
                    if selected {
                        ui.painter()
                            .circle_filled(rect.center(), 13.0, palette.accent);
                    } else if response.hovered() {
                        ui.painter()
                            .circle_filled(rect.center(), 13.0, palette.surface_hover);
                    }
                    let colour = if selected {
                        palette.on_accent
                    } else {
                        palette.text
                    };
                    let galley =
                        ui.painter()
                            .layout_no_wrap(day.to_string(), theme::regular(12.5), colour);
                    ui.painter()
                        .galley(rect.center() - galley.size() / 2.0, galley, colour);
                }
                if response.clicked() {
                    app.schedule_day = date;
                }
                response.on_hover_cursor(egui::CursorIcon::PointingHand);
                column += 1;
                if column % 7 == 0 {
                    ui.end_row();
                }
            }
        });
}

/// Two steppers, one for the hour and one for the minute.
fn time_picker(app: &mut App, ui: &mut egui::Ui, palette: &Palette) {
    theme::text(ui, "Time", theme::medium(13.5), palette.text);
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        for (label, value, step) in [
            ("Hour", &mut app.schedule_hour, 1i8),
            ("Minute", &mut app.schedule_minute, 5i8),
        ] {
            if theme::icon_button(
                ui,
                Icon::Minus,
                16.0,
                palette.secondary,
                palette.text,
                &format!("Less {label}"),
            )
            .clicked()
            {
                *value = wrap(*value - step, if step == 5 { 60 } else { 24 });
            }
            let text = format!("{:02}", *value);
            theme::text(ui, text, theme::medium(15.0), palette.text);
            if theme::icon_button(
                ui,
                Icon::Plus,
                16.0,
                palette.secondary,
                palette.text,
                &format!("More {label}"),
            )
            .clicked()
            {
                *value = wrap(*value + step, if step == 5 { 60 } else { 24 });
            }
            ui.add_space(8.0);
        }
    });
}

/// The recurrence dropdown, worded for the day that is picked.
fn repeat_picker(app: &mut App, ui: &mut egui::Ui, palette: &Palette) {
    theme::text(ui, "Repeat", theme::medium(13.5), palette.text);
    ui.add_space(4.0);
    let day = app.schedule_day;
    egui::ComboBox::from_id_salt("schedule-repeat")
        .width(240.0)
        .height(320.0)
        .selected_text(app.schedule_repeat.label(day))
        .show_ui(ui, |ui| {
            for repeat in Repeat::ALL {
                ui.selectable_value(&mut app.schedule_repeat, repeat, repeat.label(day));
            }
        });
    if app.schedule_repeat != Repeat::Once {
        ui.add_space(6.0);
        theme::text(
            ui,
            "The next one is sent after this one, and the series keeps going.",
            theme::regular(11.5),
            palette.secondary,
        );
    }
}

fn wrap(value: i8, span: i8) -> i8 {
    if value < 0 {
        span - 1
    } else if value >= span {
        0
    } else {
        value
    }
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
