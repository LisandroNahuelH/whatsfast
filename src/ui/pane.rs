//! Right inspector pane. Search is the first use; later panes reuse this shell.

use egui::{Align, Align2, CornerRadius, Frame, Layout, Margin, Sense, Vec2, pos2, vec2};
use jiff::civil::Date;

use crate::app::App;
use crate::model::{Action, Message, Page, RightPane};
use crate::schedule::{weekday_from_offset, weekday_name};
use crate::theme::{self, Icon, Palette};
use crate::util;

use super::widgets;

const CELL: f32 = 30.0;

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    if app.page != Page::Chats || app.open_chat.is_none() || app.right_pane.is_none() {
        return;
    }
    let palette = app.palette;
    let panel = egui::Panel::right("inspector")
        .resizable(true)
        .default_size(app.settings.inspector_width)
        .size_range(300.0..=520.0)
        .show_separator_line(false)
        .frame(Frame::new().fill(palette.panel).inner_margin(Margin::ZERO));
    let response = panel.show(ui, |ui| {
        super::chats::paint_split(ui, &palette);
        match app.right_pane {
            Some(RightPane::Search) => search(app, ui),
            None => {}
        }
    });
    let width = response.response.rect.width();
    if (width - app.settings.inspector_width).abs() > 1.0 {
        app.settings.inspector_width = width;
        app.actions.push(Action::SettingsChanged);
    }
}

fn search(app: &mut App, ui: &mut egui::Ui) {
    let palette = app.palette;
    let title = app
        .current_chat()
        .map(|chat| app.chat_title(chat))
        .unwrap_or_default();
    Frame::new()
        .inner_margin(Margin::symmetric(14, 10))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.set_min_height(32.0);
                if theme::icon_button(ui, Icon::X, 18.0, palette.secondary, palette.text, "Close")
                    .clicked()
                {
                    app.actions.push(Action::CloseRightPane);
                }
                ui.add_space(6.0);
                theme::text(ui, "Search messages", theme::semibold(16.0), palette.text);
            });
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.set_min_height(36.0);
                let calendar = theme::icon_button(
                    ui,
                    Icon::Calendar,
                    18.0,
                    if app.chat_search_day.is_some() || app.chat_search_calendar {
                        palette.accent
                    } else {
                        palette.secondary
                    },
                    palette.text,
                    "Filter by date",
                );
                if calendar.clicked() {
                    app.chat_search_calendar = !app.chat_search_calendar;
                }
                ui.add_space(6.0);
                let width = ui.available_width();
                let mut text = app.chat_search.clone();
                let response = widgets::search_field(
                    ui,
                    &palette,
                    egui::Id::new("chat-message-search"),
                    &mut text,
                    "Search",
                    width,
                );
                if text != app.chat_search {
                    app.actions.push(Action::SearchInChat(text));
                }
                if app.focus_chat_search {
                    app.focus_chat_search = false;
                    response.request_focus();
                }
            });
        });
    if app.chat_search_calendar {
        calendar(app, ui, &palette);
        return;
    }
    let empty = app.chat_search.trim().is_empty() && app.chat_search_day.is_none();
    if empty {
        ui.centered_and_justified(|ui| {
            theme::text(
                ui,
                format!("Search messages with {title}"),
                theme::regular(13.5),
                palette.dim,
            );
        });
        return;
    }
    let hits = app.chat_search_hits.clone();
    let query = app.chat_search.clone();
    if hits.is_empty() {
        widgets::empty_state(
            ui,
            &palette,
            Icon::Search,
            "No messages found",
            "Try another word or pick a different day.",
        );
        return;
    }
    egui::ScrollArea::vertical()
        .id_salt("chat-search-hits")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for hit in &hits {
                hit_row(app, ui, hit, &query);
            }
            ui.add_space(8.0);
        });
}

fn calendar(app: &mut App, ui: &mut egui::Ui, palette: &Palette) {
    Frame::new()
        .inner_margin(Margin::same(12))
        .fill(palette.surface)
        .corner_radius(CornerRadius::same(theme::RADIUS + 4))
        .show(ui, |ui| {
            month_header(app, ui, palette);
            ui.add_space(6.0);
            month_grid(app, ui, palette);
        });
}

fn month_header(app: &mut App, ui: &mut egui::Ui, palette: &Palette) {
    let month = app.chat_search_month;
    ui.horizontal(|ui| {
        theme::text(ui, month_label(month), theme::medium(14.5), palette.text);
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if theme::icon_button(
                ui,
                Icon::ChevronRight,
                16.0,
                palette.secondary,
                palette.text,
                "Next month",
            )
            .clicked()
            {
                app.chat_search_month = month_step(month, 1);
            }
            if theme::icon_button(
                ui,
                Icon::ChevronLeft,
                16.0,
                palette.secondary,
                palette.text,
                "Previous month",
            )
            .clicked()
            {
                app.chat_search_month = month_step(month, -1);
            }
        });
    });
}

fn month_grid(app: &mut App, ui: &mut egui::Ui, palette: &Palette) {
    let month = app.chat_search_month;
    let block = CELL * 7.0;
    let head = 18.0;
    let height = head + 4.0 + CELL * 6.0;
    let (rect, _) = ui.allocate_exact_size(
        vec2(ui.available_width().min(block + 8.0), height),
        Sense::hover(),
    );
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
            Align2::CENTER_CENTER,
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
            egui::Id::new(("chat-search-day", month.year(), month.month(), day)),
            Sense::click(),
        );
        let selected = app.chat_search_day == Some(date);
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
            let next = if selected { None } else { Some(date) };
            app.actions.push(Action::SetChatSearchDay(next));
        }
        response.on_hover_cursor(egui::CursorIcon::PointingHand);
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

fn hit_row(app: &mut App, ui: &mut egui::Ui, hit: &Message, query: &str) {
    let palette = app.palette;
    let id = egui::Id::new(("chat-search-hit", hit.chat.as_str(), hit.id.as_str()));
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 56.0), Sense::hover());
    let response = ui.interact(rect, id, Sense::click());
    if ui.is_rect_visible(rect) {
        if response.hovered() {
            ui.painter().rect_filled(rect, 0.0, palette.surface_hover);
        }
        let stamp = util::chat_stamp(hit.timestamp);
        ui.painter().text(
            pos2(rect.left() + 14.0, rect.top() + 10.0),
            Align2::LEFT_TOP,
            stamp,
            theme::regular(11.5),
            palette.dim,
        );
        let snippet = hit.summary();
        let mut x = rect.left() + 14.0;
        let line_y = rect.top() + 28.0;
        if hit.from_me {
            let checks =
                egui::Rect::from_center_size(pos2(x + 8.0, line_y + 8.0), Vec2::splat(14.0));
            Icon::CheckCheck
                .image(palette.dim, 12.0)
                .paint_at(ui, checks);
            x += 20.0;
        }
        paint_snippet(
            ui,
            pos2(x, line_y),
            (rect.right() - 14.0 - x).max(0.0),
            &snippet,
            query,
            &palette,
        );
    }
    let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
    if response.clicked() {
        app.actions.push(Action::OpenMessage {
            chat: hit.chat.clone(),
            message: hit.id.clone(),
        });
    }
}

fn paint_snippet(
    ui: &egui::Ui,
    pos: egui::Pos2,
    width: f32,
    text: &str,
    query: &str,
    palette: &Palette,
) {
    let query = query.trim();
    let range = (!query.is_empty()).then(|| {
        let lower = text.to_lowercase();
        let needle = query.to_lowercase();
        lower.find(&needle).and_then(|start| {
            let end = start + needle.len();
            (text.is_char_boundary(start) && text.is_char_boundary(end)).then_some((start, end))
        })
    });
    let mut job = egui::text::LayoutJob::default();
    let font = theme::regular(13.5);
    match range.flatten() {
        Some((start, end)) => {
            if start > 0 {
                job.append(
                    &text[..start],
                    0.0,
                    egui::TextFormat::simple(font.clone(), palette.secondary),
                );
            }
            if start < end {
                job.append(
                    &text[start..end],
                    0.0,
                    egui::TextFormat::simple(font.clone(), palette.accent),
                );
            }
            if end < text.len() {
                job.append(
                    &text[end..],
                    0.0,
                    egui::TextFormat::simple(font, palette.secondary),
                );
            }
        }
        None => {
            job.append(text, 0.0, egui::TextFormat::simple(font, palette.secondary));
        }
    }
    job.wrap.max_rows = 1;
    job.wrap.max_width = width.max(0.0);
    let galley = crate::bidi::layout_job(ui, job);
    ui.painter().galley(pos, galley, palette.secondary);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn months_step_across_the_year_boundary() {
        let december = Date::new(2026, 12, 1).expect("a date");
        assert_eq!(month_step(december, 1).month(), 1);
        assert_eq!(month_step(december, 1).year(), 2027);
        let january = Date::new(2026, 1, 1).expect("a date");
        assert_eq!(month_step(january, -1).month(), 12);
        assert_eq!(month_step(january, -1).year(), 2025);
    }
}
