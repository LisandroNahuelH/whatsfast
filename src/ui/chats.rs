//! The left panel: the chat list.

use egui::{Align, Frame, Layout, Margin, Rect, Sense, Stroke, Vec2, pos2, vec2};

use crate::app::App;
use crate::model::{Action, Chat, Contact, Dialog, Message, Page};
use crate::theme::{self, Icon, Palette};

use super::widgets;

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    let palette = app.palette;
    let panel = egui::Panel::left("chats")
        .resizable(true)
        .default_size(app.settings.sidebar_width)
        .size_range(if theme::macos_chrome(ui.ctx()) {
            (theme::traffic_light_inset(ui.ctx()) + 210.0).max(280.0)..=520.0
        } else {
            260.0..=520.0
        })
        .show_separator_line(false)
        .frame(Frame::new().fill(palette.panel).inner_margin(Margin::ZERO));
    let response = panel.show(ui, |ui| {
        header(app, ui);
        list(app, ui);
    });
    let width = response.response.rect.width();
    if (width - app.settings.sidebar_width).abs() > 1.0 {
        app.settings.sidebar_width = width;
        app.actions.push(Action::SettingsChanged);
    }
    // Separate the panel from the conversation.
    let rect = response.response.rect;
    ui.painter().vline(
        rect.right(),
        rect.y_range(),
        egui::Stroke::new(1.0, palette.outline),
    );
}

/// Width of the sidebar when it narrows to its bones.
const COMPACT_WIDTH: f32 = 72.0;

/// Height of one chat picture in the narrow sidebar.
const COMPACT_ROW: f32 = 56.0;

/// The sidebar narrowed to its bones: your picture, search, archived chats,
/// and one picture per chat. Hiding the bar then never traps a user who does
/// not know the shortcut.
pub fn show_compact(app: &mut App, ui: &mut egui::Ui) {
    let palette = app.palette;
    let panel = egui::Panel::left("chats-compact")
        .resizable(false)
        .default_size(COMPACT_WIDTH)
        .size_range(COMPACT_WIDTH..=COMPACT_WIDTH)
        .show_separator_line(false)
        .frame(Frame::new().fill(palette.panel).inner_margin(Margin::ZERO));
    let response = panel.show(ui, |ui| {
        compact_header(app, ui);
        compact_list(app, ui);
    });
    // Separate the panel from the conversation.
    let rect = response.response.rect;
    ui.painter().vline(
        rect.right(),
        rect.y_range(),
        egui::Stroke::new(1.0, palette.outline),
    );
}

/// Your picture, search, and archived chats, stacked in the narrow sidebar.
fn compact_header(app: &mut App, ui: &mut egui::Ui) {
    let palette = app.palette;
    let top = if theme::macos_chrome(ui.ctx()) {
        theme::traffic_light_inset(ui.ctx()) + 8.0
    } else {
        10.0
    };
    Frame::new()
        .inner_margin(Margin {
            left: 0,
            right: 0,
            top: top as i8,
            bottom: 6,
        })
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                let me = app.me.clone().unwrap_or_default();
                let name = app.me_name.clone().unwrap_or_else(|| "You".to_owned());
                let picture = app.avatar(&me);
                let tooltip = match &app.me_about {
                    Some(about) => format!("{name}\n{about}"),
                    None => name.clone(),
                };
                let response = widgets::avatar(ui, &palette, &name, &me, 34.0, picture.as_deref())
                    .interact(Sense::click())
                    .on_hover_text(tooltip)
                    .on_hover_cursor(egui::CursorIcon::PointingHand);
                if response.clicked() {
                    app.actions.push(Action::ToggleSettings);
                }
                ui.add_space(2.0);
                if theme::icon_button(
                    ui,
                    Icon::PanelLeft,
                    18.0,
                    palette.secondary,
                    palette.text,
                    "Show the chat list (Ctrl+B)",
                )
                .clicked()
                {
                    app.actions.push(Action::ToggleSidebar);
                }
                if theme::icon_button(
                    ui,
                    Icon::Search,
                    18.0,
                    palette.secondary,
                    palette.text,
                    "Search (Ctrl+F)",
                )
                .clicked()
                {
                    app.actions.push(Action::FocusSearch);
                }
                if theme::icon_button(
                    ui,
                    Icon::Archive,
                    18.0,
                    palette.secondary,
                    palette.text,
                    "Archived chats",
                )
                .clicked()
                {
                    app.sidebar_visible = true;
                    app.show_archived = true;
                }
            });
        });
}

/// One picture per chat, with no names: enough to get back into a chat.
fn compact_list(app: &mut App, ui: &mut egui::Ui) {
    let palette = app.palette;
    let chats: Vec<Chat> = app.visible_chats().into_iter().cloned().collect();
    egui::ScrollArea::vertical()
        .id_salt("chats-compact")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for chat in &chats {
                let title = app.chat_title(chat);
                let picture = app.avatar(&chat.id);
                let (rect, response) =
                    ui.allocate_exact_size(vec2(ui.available_width(), COMPACT_ROW), Sense::click());
                if ui.is_rect_visible(rect) {
                    if response.hovered() {
                        ui.painter()
                            .rect_filled(rect.shrink(4.0), 10.0, palette.surface_hover);
                    }
                    let avatar_rect = Rect::from_center_size(rect.center(), Vec2::splat(44.0));
                    widgets::paint_avatar(
                        ui,
                        &palette,
                        avatar_rect,
                        &title,
                        &chat.id,
                        picture.as_deref(),
                    );
                }
                if response
                    .on_hover_text(&title)
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                {
                    app.actions.push(Action::OpenChat(chat.id.clone()));
                }
            }
        });
}

fn header(app: &mut App, ui: &mut egui::Ui) {
    if theme::macos_chrome(ui.ctx()) {
        macos_header(app, ui);
        return;
    }
    let palette = app.palette;
    Frame::new()
        .inner_margin(Margin {
            left: 14,
            right: 10,
            top: 12,
            bottom: 8,
        })
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                if app.show_scheduled {
                    if theme::icon_button(
                        ui,
                        Icon::ArrowLeft,
                        18.0,
                        palette.secondary,
                        palette.text,
                        "Back to chats",
                    )
                    .clicked()
                    {
                        app.actions.push(Action::ToggleScheduled);
                    }
                    theme::text(ui, "Scheduled", theme::bold(20.0), palette.text);
                } else if app.show_archived {
                    if theme::icon_button(
                        ui,
                        Icon::ArrowLeft,
                        18.0,
                        palette.secondary,
                        palette.text,
                        "Back to chats",
                    )
                    .clicked()
                    {
                        app.show_archived = false;
                    }
                    theme::text(ui, "Archived", theme::bold(20.0), palette.text);
                } else if app.show_starred {
                    if theme::icon_button(
                        ui,
                        Icon::ArrowLeft,
                        18.0,
                        palette.secondary,
                        palette.text,
                        "Back to chats",
                    )
                    .clicked()
                    {
                        app.actions.push(Action::ToggleStarred);
                    }
                    theme::text(ui, "Starred", theme::bold(20.0), palette.text);
                } else {
                    let me = app.me.clone().unwrap_or_default();
                    let name = app.me_name.clone().unwrap_or_else(|| "You".to_owned());
                    let picture = app.avatar(&me);
                    let tooltip = match &app.me_about {
                        Some(about) => format!("{name}\n{about}"),
                        None => name.clone(),
                    };
                    let response =
                        widgets::avatar(ui, &palette, &name, &me, 34.0, picture.as_deref())
                            .interact(Sense::click())
                            .on_hover_text(tooltip)
                            .on_hover_cursor(egui::CursorIcon::PointingHand);
                    if response.clicked() {
                        app.actions.push(Action::ToggleSettings);
                    }
                    ui.add_space(2.0);
                    theme::text(ui, "Chats", theme::bold(20.0), palette.text);
                }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if theme::icon_button(
                        ui,
                        Icon::Clock,
                        18.0,
                        if app.show_scheduled {
                            palette.accent
                        } else {
                            palette.secondary
                        },
                        palette.text,
                        "Scheduled messages",
                    )
                    .clicked()
                    {
                        app.actions.push(Action::ToggleScheduled);
                    }
                    if theme::icon_button(
                        ui,
                        Icon::Star,
                        18.0,
                        if app.show_starred {
                            palette.accent
                        } else {
                            palette.secondary
                        },
                        palette.text,
                        "Starred messages",
                    )
                    .clicked()
                    {
                        app.actions.push(Action::ToggleStarred);
                    }
                    if theme::icon_button(
                        ui,
                        Icon::Settings,
                        18.0,
                        if app.page == Page::Settings {
                            palette.accent
                        } else {
                            palette.secondary
                        },
                        palette.text,
                        "Settings (Ctrl+,)",
                    )
                    .clicked()
                    {
                        app.actions.push(Action::ToggleSettings);
                    }
                    if theme::icon_button(
                        ui,
                        Icon::SquarePen,
                        18.0,
                        palette.secondary,
                        palette.text,
                        "New contact",
                    )
                    .clicked()
                    {
                        app.actions
                            .push(Action::ShowDialog(crate::model::Dialog::NewContact));
                    }
                    if theme::icon_button(
                        ui,
                        Icon::PanelLeft,
                        18.0,
                        palette.secondary,
                        palette.text,
                        "Hide the chat list (Ctrl+B)",
                    )
                    .clicked()
                    {
                        app.actions.push(Action::ToggleSidebar);
                    }
                });
            });
            ui.add_space(6.0);
            let id = egui::Id::new("chat-search");
            let width = ui.available_width();
            let mut text = app.search.clone();
            let response = widgets::search_field(ui, &palette, id, &mut text, "Search", width);
            if text != app.search {
                app.actions.push(Action::Search(text));
            }
            if app.focus_search {
                app.focus_search = false;
                response.request_focus();
            }
        });
}

fn macos_header(app: &mut App, ui: &mut egui::Ui) {
    let palette = app.palette;
    let inset = theme::traffic_light_inset(ui.ctx());
    let mut drag = ui.max_rect();
    drag.min.x += inset;
    drag.max.y = drag.min.y + 60.0;
    super::titlebar_drag(ui, drag);
    Frame::new()
        .inner_margin(Margin::symmetric(14, 8))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.set_min_height(44.0);
                ui.add_space((inset - 14.0).max(0.0));
                if app.show_scheduled {
                    if theme::icon_button(
                        ui,
                        Icon::ArrowLeft,
                        18.0,
                        palette.secondary,
                        palette.text,
                        "Back to chats",
                    )
                    .clicked()
                    {
                        app.actions.push(Action::ToggleScheduled);
                    }
                    theme::text(ui, "Scheduled", theme::bold(20.0), palette.text);
                } else if app.show_archived {
                    if theme::icon_button(
                        ui,
                        Icon::ArrowLeft,
                        18.0,
                        palette.secondary,
                        palette.text,
                        "Back to chats",
                    )
                    .clicked()
                    {
                        app.show_archived = false;
                    }
                    theme::text(ui, "Archived", theme::bold(16.0), palette.text);
                } else if app.show_starred {
                    if theme::icon_button(
                        ui,
                        Icon::ArrowLeft,
                        18.0,
                        palette.secondary,
                        palette.text,
                        "Back to chats",
                    )
                    .clicked()
                    {
                        app.actions.push(Action::ToggleStarred);
                    }
                    theme::text(ui, "Starred", theme::bold(20.0), palette.text);
                } else {
                    theme::text(ui, "Chats", theme::bold(20.0), palette.text);
                }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if theme::icon_button(
                        ui,
                        Icon::Star,
                        18.0,
                        if app.show_starred {
                            palette.accent
                        } else {
                            palette.secondary
                        },
                        palette.text,
                        "Starred messages",
                    )
                    .clicked()
                    {
                        app.actions.push(Action::ToggleStarred);
                    }
                    if theme::icon_button(
                        ui,
                        Icon::SquarePen,
                        18.0,
                        palette.secondary,
                        palette.text,
                        "New contact (⌘N)",
                    )
                    .clicked()
                    {
                        app.actions.push(Action::ShowDialog(Dialog::NewContact));
                    }
                    if theme::icon_button(
                        ui,
                        Icon::PanelLeft,
                        18.0,
                        palette.secondary,
                        palette.text,
                        "Hide the chat list (⌘B)",
                    )
                    .clicked()
                    {
                        app.actions.push(Action::ToggleSidebar);
                    }
                });
            });
            ui.add_space(6.0);
            let mut text = app.search.clone();
            let response = widgets::search_field(
                ui,
                &palette,
                egui::Id::new("chat-search"),
                &mut text,
                "Search",
                ui.available_width(),
            );
            if text != app.search {
                app.actions.push(Action::Search(text));
            }
            if app.focus_search {
                app.focus_search = false;
                response.request_focus();
            }
        });
}

/// The scheduled messages, in the place the chat list usually takes.
fn scheduled_list(app: &mut App, ui: &mut egui::Ui) {
    let palette = app.palette;
    let entries: Vec<crate::archive::Scheduled> = app.scheduled.clone();
    if entries.is_empty() {
        widgets::empty_state(
            ui,
            &palette,
            Icon::Clock,
            "No scheduled messages",
            "Write a message and pick a time with the clock beside the send button.",
        );
        return;
    }
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for entry in &entries {
                scheduled_row(app, ui, &palette, entry);
            }
        });
}

/// One scheduled message: where it goes, when, how it repeats, and a way to
/// drop it.
fn scheduled_row(
    app: &mut App,
    ui: &mut egui::Ui,
    palette: &Palette,
    entry: &crate::archive::Scheduled,
) {
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 72.0), Sense::click());
    if !ui.is_rect_visible(rect) {
        return;
    }
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, palette.surface_hover);
    }
    let failed = entry.state == "failed";
    theme::paint_icon(
        ui,
        Icon::Clock,
        egui::Rect::from_center_size(pos2(rect.left() + 30.0, rect.center().y), Vec2::splat(22.0)),
        22.0,
        if failed {
            palette.danger
        } else {
            palette.accent
        },
    );
    let name = app.display_name_or(&entry.chat, None);
    let rule =
        crate::schedule::recurrence(&entry.kind, entry.weekday, entry.day_of_month, entry.nth)
            .map(|rule| rule.label())
            .unwrap_or_else(|| "Once".to_owned());
    let when = when_label(entry.next_at);
    let top = rect.top();
    // First line: where it goes and how it repeats, with the time on the right.
    ui.painter().text(
        pos2(rect.left() + 56.0, top + 20.0),
        egui::Align2::LEFT_CENTER,
        format!("{name} · {rule}"),
        theme::medium(14.0),
        palette.text,
    );
    ui.painter().text(
        pos2(rect.right() - 40.0, top + 20.0),
        egui::Align2::RIGHT_CENTER,
        when,
        theme::regular(11.5),
        palette.secondary,
    );
    // Second line: the message itself, with the reason it failed when it did.
    ui.painter().text(
        pos2(rect.left() + 56.0, top + 46.0),
        egui::Align2::LEFT_CENTER,
        preview(&entry.text),
        theme::regular(12.5),
        palette.secondary,
    );
    if let Some(error) = &entry.last_error {
        ui.painter().text(
            pos2(rect.right() - 40.0, top + 46.0),
            egui::Align2::RIGHT_CENTER,
            error,
            theme::regular(11.5),
            palette.danger,
        );
    }
    let close = egui::Rect::from_center_size(
        pos2(rect.right() - 24.0, rect.center().y),
        Vec2::splat(26.0),
    );
    let cancel = ui.interact(
        close,
        egui::Id::new(("cancel-scheduled", entry.id.as_str())),
        Sense::click(),
    );
    if cancel.hovered() {
        ui.painter()
            .circle_filled(close.center(), 13.0, palette.surface_active);
    }
    theme::paint_icon(
        ui,
        Icon::X,
        close,
        15.0,
        if cancel.hovered() {
            palette.text
        } else {
            palette.secondary
        },
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, palette.outline),
    );
    if cancel
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
    {
        app.actions.push(Action::CancelScheduled {
            id: entry.id.clone(),
        });
    }
}

/// The message's own words, flattened to one line for the list.
fn preview(text: &str) -> String {
    let flat: String = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut out: String = flat.chars().take(48).collect();
    if flat.chars().count() > 48 {
        out.push('…');
    }
    out
}

/// "Fri 18 Sep, 21:00" in the machine's time zone.
fn when_label(instant: i64) -> String {
    jiff::Timestamp::from_second(instant)
        .ok()
        .map(|moment| moment.to_zoned(jiff::tz::TimeZone::system()))
        .map(|moment| moment.strftime("%a %d %b, %H:%M").to_string())
        .unwrap_or_default()
}

/// The starred messages, in the place the chat list usually takes. Newest
/// star first, each row naming the chat, the moment, and the message.
fn starred_list(app: &mut App, ui: &mut egui::Ui) {
    let palette = app.palette;
    let entries: Vec<crate::archive::Starred> = app.starred.clone();
    if entries.is_empty() {
        widgets::empty_state(
            ui,
            &palette,
            Icon::Star,
            "No starred messages",
            "Pick messages in a chat and press Star to keep them here.",
        );
        return;
    }
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for entry in &entries {
                starred_row(app, ui, &palette, entry);
            }
        });
}

/// One starred message: where it is, when it was starred, and the message
/// itself drawn as the bubble it is in the chat. Clicking opens its chat at
/// the message.
fn starred_row(
    app: &mut App,
    ui: &mut egui::Ui,
    palette: &Palette,
    entry: &crate::archive::Starred,
) {
    let width = ui.available_width();
    let clock = ui.painter().layout_no_wrap(
        crate::util::clock(entry.sent_at),
        theme::regular(11.0),
        palette.secondary,
    );
    // The bubble wraps, so the row is as tall as the message needs. The text
    // leaves room for the time in the corner, like the chat does.
    let galley = ui.painter().layout(
        entry.text.clone(),
        theme::regular(13.0),
        palette.text,
        (width - 32.0 - 24.0 - clock.size().x - 10.0).max(60.0),
    );
    let bubble_height = galley.rect.height() + 12.0;
    let (rect, response) =
        ui.allocate_exact_size(vec2(width, 26.0 + bubble_height + 10.0), Sense::click());
    if !ui.is_rect_visible(rect) {
        return;
    }
    if response.hovered() {
        ui.painter()
            .rect_filled(rect, 6.0, palette.surface_hover.gamma_multiply(0.5));
    }
    let name = app.display_name_or(&entry.chat, None);
    let when = when_label(entry.starred_at);
    // First line: where the message is, with the moment it was starred.
    ui.painter().text(
        pos2(rect.left() + 16.0, rect.top() + 14.0),
        egui::Align2::LEFT_CENTER,
        name,
        theme::medium(14.0),
        palette.text,
    );
    ui.painter().text(
        pos2(rect.right() - 16.0, rect.top() + 14.0),
        egui::Align2::RIGHT_CENTER,
        when,
        theme::regular(11.5),
        palette.secondary,
    );
    // The message itself, as the bubble the chat shows: same fill, same side,
    // with the time in its bottom corner.
    let bubble_width = (galley.rect.width() + 20.0 + clock.size().x + 6.0).min(width - 32.0);
    let left = if entry.from_me {
        rect.right() - 16.0 - bubble_width
    } else {
        rect.left() + 16.0
    };
    let bubble = Rect::from_min_size(
        pos2(left, rect.top() + 26.0),
        vec2(bubble_width, bubble_height),
    );
    ui.painter().rect_filled(
        bubble,
        10.0,
        if entry.from_me {
            palette.bubble_out
        } else {
            palette.bubble_in
        },
    );
    ui.painter().galley(
        pos2(bubble.left() + 10.0, bubble.top() + 6.0),
        galley,
        palette.text,
    );
    ui.painter().galley(
        pos2(
            bubble.right() - 10.0 - clock.size().x,
            bubble.bottom() - 6.0 - clock.size().y,
        ),
        clock,
        palette.secondary,
    );
    if response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
    {
        app.actions.push(Action::OpenMessage {
            chat: entry.chat.clone(),
            message: entry.id.clone(),
        });
    }
}

fn list(app: &mut App, ui: &mut egui::Ui) {
    let palette = app.palette;
    if app.show_scheduled {
        scheduled_list(app, ui);
        return;
    }
    if app.show_starred {
        starred_list(app, ui);
        return;
    }
    if !app.search.trim().is_empty() {
        results(app, ui);
        return;
    }
    let mut chats: Vec<Chat> = app.visible_chats().into_iter().cloned().collect();
    let archived = app.archived_count();
    let show_archive_row = !app.show_archived && archived > 0;
    if chats.is_empty() && !show_archive_row {
        let (title, body) = if app.show_archived {
            ("Nothing archived", "Archived chats appear here.")
        } else if app.syncing {
            ("Loading your chats", "Receiving history from your phone.")
        } else {
            (
                "No chats yet",
                "New chats appear here. You can start one from your phone.",
            )
        };
        widgets::empty_state(ui, &palette, Icon::MessageCircle, title, body);
        return;
    }
    let row_height = theme::ROW_HEIGHT;
    let stride = row_height + ui.spacing().item_spacing.y;
    let list_top = ui.cursor().top();
    let list_id = ui.make_persistent_id(egui::IdSalt::new("chat-list"));
    let offset = egui::scroll_area::State::load(ui.ctx(), list_id)
        .unwrap_or_default()
        .offset
        .y;
    let pinned = chats.iter().filter(|chat| chat.pinned).count();
    let finished = pin_gesture(
        app,
        ui,
        list_top,
        offset,
        pinned,
        stride,
        usize::from(show_archive_row),
    );
    // The list draws the order a release would write: the held chat sits in
    // the slot under the pointer and the rest close the gap behind it, so the
    // user sees where it lands before letting go.
    if let Some(drag) = app.pin_drag.as_ref().filter(|drag| drag.active)
        && drag.from < chats.len()
    {
        let chat = chats.remove(drag.from);
        chats.insert(drag.to.min(chats.len()), chat);
    }
    let total = chats.len() + usize::from(show_archive_row);
    let mut scroll_area = egui::ScrollArea::vertical()
        .id_salt("chat-list")
        .auto_shrink([false, false]);
    let target_row = app.scroll_chat_into_view.as_ref().and_then(|target| {
        chats
            .iter()
            .position(|chat| chat.id == *target)
            .map(|index| index + usize::from(show_archive_row))
    });
    if let Some(target_row) = target_row {
        let id = ui.make_persistent_id(egui::IdSalt::new("chat-list"));
        let current = egui::scroll_area::State::load(ui.ctx(), id)
            .unwrap_or_default()
            .offset
            .y;
        let offset = row_scroll_offset(
            current,
            ui.available_height(),
            target_row,
            row_height,
            ui.spacing().item_spacing.y,
        );
        scroll_area = scroll_area.vertical_scroll_offset(offset);
        app.scroll_chat_into_view = None;
    }
    scroll_area.show_rows(ui, row_height, total, |ui, range| {
        for index in range {
            if show_archive_row && index == 0 {
                archive_row(app, ui, archived);
                continue;
            }
            let chat = &chats[index - usize::from(show_archive_row)];
            let row_index = index - usize::from(show_archive_row);
            // Key by chat so an open menu survives list reordering.
            ui.push_id(("chat", &chat.id), |ui| row(app, ui, chat, row_index));
        }
    });
    // The held chat itself rides with the pointer, so the list reads as
    // picked up instead of pasted into place.
    let held = app
        .pin_drag
        .as_ref()
        .filter(|drag| drag.active)
        .map(|drag| (drag.chat.clone(), drag.grab_y));
    if let Some((id, grab_y)) = held {
        if let Some(chat) = app.chat(&id).cloned() {
            let title = app.chat_title(&chat);
            let picture = app.avatar(&chat.id);
            if let Some(pointer) = ui.input(|input| input.pointer.interact_pos()) {
                let rect = Rect::from_min_size(
                    pos2(ui.max_rect().left(), pointer.y - grab_y),
                    vec2(ui.max_rect().width(), row_height),
                );
                paint_dragged(ui, &palette, rect, &title, &chat.id, picture.as_deref());
            }
        }
    }
    // The rows have drawn with the gesture still set, so the release cannot
    // read as a click that opens the chat.
    if finished {
        app.pin_drag = None;
    }
}

/// How long a pinned row must be held before it can be moved.
const PIN_HOLD: f64 = 0.35;

/// Runs the pinned-chat gesture: the hold turns it on, the pointer picks the
/// slot, and the release writes the new order. It runs before the rows are
/// drawn, because `show_rows` stops drawing the rows that scroll out of view
/// and a release read there would be lost.
fn pin_gesture(
    app: &mut App,
    ui: &mut egui::Ui,
    top: f32,
    offset: f32,
    pinned: usize,
    stride: f32,
    first: usize,
) -> bool {
    let Some(mut drag) = app.pin_drag.clone() else {
        return false;
    };
    let (time, down, released, pointer) = ui.input(|input| {
        (
            input.time,
            input.pointer.primary_down(),
            input.pointer.any_released(),
            input.pointer.interact_pos(),
        )
    });
    if !drag.active && time - drag.since >= PIN_HOLD {
        drag.active = true;
    }
    if drag.active
        && let Some(pointer) = pointer
    {
        drag.to = pinned_slot(pointer.y, drag.grab_y, top, offset, stride, first, pinned);
    }
    if !down || released {
        if drag.active && drag.to != drag.from {
            app.actions
                .push(Action::ReorderPinned(pinned_order(app, drag.from, drag.to)));
        }
        // The rows still read the gesture while they draw this frame: egui
        // counts a hold of up to 0.8 s as a click, so clearing it here would
        // let the release open the chat. `list` clears it once they are drawn.
        app.pin_drag = Some(drag);
        return true;
    }
    // A still pointer sends no events, so the hold would never reach its
    // threshold without asking for the next frame.
    ui.ctx().request_repaint();
    app.pin_drag = Some(drag);
    false
}

/// Paints the held chat riding under the pointer, above the list.
fn paint_dragged(
    ui: &egui::Ui,
    palette: &Palette,
    rect: Rect,
    title: &str,
    id: &str,
    picture: Option<&std::path::Path>,
) {
    let painter = ui.painter();
    // The lift: a shadow under the row, the row itself on top.
    painter.rect_filled(rect.translate(vec2(0.0, 3.0)), 10.0, palette.shadow);
    painter.rect_filled(rect, 10.0, palette.surface_active);
    painter.rect_stroke(
        rect,
        10.0,
        egui::Stroke::new(1.0, palette.outline),
        egui::StrokeKind::Inside,
    );
    let avatar_rect =
        Rect::from_center_size(pos2(rect.left() + 38.0, rect.center().y), Vec2::splat(48.0));
    widgets::paint_avatar(ui, palette, avatar_rect, title, id, picture);
    let name = widgets::line(
        ui,
        title,
        theme::medium(14.5),
        palette.text,
        rect.right() - 14.0 - (rect.left() + 76.0),
        1,
    );
    name.paint(
        ui,
        pos2(rect.left() + 76.0, rect.top() + 14.0),
        palette.text,
    );
}

/// The pinned slot under the pointer. It measures from the middle of the
/// carried row, not from the pointer itself, so the row lands where it looks
/// like it lands. `first` is the archived row, which is not pinned and must
/// not shift every slot down by one.
fn pinned_slot(
    pointer: f32,
    grab_y: f32,
    top: f32,
    offset: f32,
    stride: f32,
    first: usize,
    pinned: usize,
) -> usize {
    let carried = pointer - grab_y + theme::ROW_HEIGHT / 2.0;
    let row = ((carried - top + offset) / stride).floor() - first as f32;
    (row.max(0.0) as usize).min(pinned.saturating_sub(1))
}

/// The pinned chats, with the one at `from` moved to `to`, top first.
fn pinned_order(app: &App, from: usize, to: usize) -> Vec<crate::model::ChatId> {
    let mut ids: Vec<crate::model::ChatId> = app
        .visible_chats()
        .into_iter()
        .filter(|chat| chat.pinned)
        .map(|chat| chat.id.clone())
        .collect();
    if from < ids.len() {
        let moved = ids.remove(from);
        ids.insert(to.min(ids.len()), moved);
    }
    ids
}

/// Returns the smallest offset that fully reveals a fixed-height row.
fn row_scroll_offset(
    current: f32,
    viewport_height: f32,
    row: usize,
    row_height: f32,
    spacing: f32,
) -> f32 {
    let top = row as f32 * (row_height + spacing);
    let bottom = top + row_height;
    if top < current {
        top
    } else if bottom > current + viewport_height {
        (bottom - viewport_height).max(0.0)
    } else {
        current
    }
}

/// Search results grouped into chats, messages, and contacts.
fn results(app: &mut App, ui: &mut egui::Ui) {
    let palette = app.palette;
    let chats: Vec<Chat> = app.visible_chats().into_iter().cloned().collect();
    let hits: Vec<Message> = app.search_hits.clone();
    let contacts: Vec<Contact> = app.matching_contacts().into_iter().cloned().collect();
    if chats.is_empty() && hits.is_empty() && contacts.is_empty() {
        widgets::empty_state(
            ui,
            &palette,
            Icon::Search,
            "No results",
            "Try another name, number, or message text.",
        );
        return;
    }
    egui::ScrollArea::vertical()
        .id_salt("search-results")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if !chats.is_empty() {
                section(ui, &palette, "Chats");
                for chat in &chats {
                    let reveal = app.scroll_chat_into_view.as_deref() == Some(chat.id.as_str());
                    let response = ui
                        .push_id(("chat", &chat.id), |ui| row(app, ui, chat, 0))
                        .inner;
                    if reveal {
                        response.scroll_to_me(None);
                        app.scroll_chat_into_view = None;
                    }
                }
            }
            if !hits.is_empty() {
                section(ui, &palette, "Messages");
                for hit in &hits {
                    ui.push_id(("hit", &hit.chat, &hit.id), |ui| hit_row(app, ui, hit));
                }
            }
            if !contacts.is_empty() {
                section(ui, &palette, "Contacts");
                for contact in &contacts {
                    ui.push_id(("contact", &contact.id), |ui| contact_row(app, ui, contact));
                }
            }
            ui.add_space(8.0);
        });
}

fn section(ui: &mut egui::Ui, palette: &Palette, label: &str) {
    ui.add_space(10.0);
    Frame::new()
        .inner_margin(Margin {
            left: 14,
            right: 14,
            top: 0,
            bottom: 4,
        })
        .show(ui, |ui| {
            theme::text(ui, label, theme::semibold(12.5), palette.accent);
        });
}

/// A message search result. Clicking it opens the chat at that message.
fn hit_row(app: &mut App, ui: &mut egui::Ui, hit: &Message) {
    let palette = app.palette;
    let title = match app.chat(&hit.chat) {
        Some(chat) => app.chat_title(&chat.clone()),
        None => app.display_name_or(&hit.chat, None),
    };
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), theme::ROW_HEIGHT),
        Sense::click(),
    );
    if ui.is_rect_visible(rect) {
        if response.hovered() {
            ui.painter().rect_filled(rect, 0.0, palette.surface_hover);
        }
        let avatar_rect =
            Rect::from_center_size(pos2(rect.left() + 38.0, rect.center().y), Vec2::splat(48.0));
        let picture = app.avatar(&hit.chat);
        widgets::paint_avatar(
            ui,
            &palette,
            avatar_rect,
            &title,
            &hit.chat,
            picture.as_deref(),
        );
        let left = rect.left() + 76.0;
        let right = rect.right() - 14.0;
        let stamp_galley = ui.painter().layout_no_wrap(
            crate::util::chat_stamp(hit.timestamp),
            theme::regular(11.5),
            palette.dim,
        );
        let name_top = rect.top() + 14.0;
        ui.painter().galley(
            pos2(right - stamp_galley.size().x, name_top + 1.0),
            stamp_galley.clone(),
            palette.dim,
        );
        let name_width = (right - stamp_galley.size().x - 8.0 - left).max(0.0);
        let name = widgets::line(ui, &title, theme::medium(14.5), palette.text, name_width, 1);
        name.paint(ui, pos2(left, name_top), palette.text);
        // Show the sender for group messages.
        let line_y = rect.top() + 38.0;
        let mut x = left;
        if hit.from_me {
            let who = widgets::line(
                ui,
                "You: ",
                theme::regular(13.0),
                palette.dim,
                (right - x) * 0.5,
                1,
            );
            who.paint(ui, pos2(x, line_y), palette.dim);
            x += who.size().x;
        } else if crate::model::ChatKind::from_id(&hit.chat) == crate::model::ChatKind::Group {
            let sender = app.display_name_or(&hit.sender, hit.sender_name.as_deref());
            let first = sender.split_whitespace().next().unwrap_or(&sender);
            let who = widgets::line(
                ui,
                &format!("{first}: "),
                theme::regular(13.0),
                palette.dim,
                (right - x) * 0.5,
                1,
            );
            who.paint(ui, pos2(x, line_y), palette.dim);
            x += who.size().x;
        }
        let words = widgets::line(
            ui,
            &crate::markup::plain(&app.resolve_mention_tokens(&hit.summary()), &[]),
            theme::regular(13.0),
            palette.dim,
            (right - x).max(0.0),
            1,
        );
        words.paint(ui, pos2(x, line_y), palette.dim);
        ui.painter().hline(
            left..=rect.right(),
            rect.bottom() - 0.5,
            egui::Stroke::new(1.0, palette.outline),
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

/// A contact without a chat. Clicking starts one.
fn contact_row(app: &mut App, ui: &mut egui::Ui, contact: &Contact) {
    let palette = app.palette;
    let name = contact
        .display_name()
        .map(str::to_owned)
        .unwrap_or_else(|| app.display_name_or(&contact.id, None));
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), theme::ROW_HEIGHT),
        Sense::click(),
    );
    if ui.is_rect_visible(rect) {
        if response.hovered() {
            ui.painter().rect_filled(rect, 0.0, palette.surface_hover);
        }
        let avatar_rect =
            Rect::from_center_size(pos2(rect.left() + 38.0, rect.center().y), Vec2::splat(48.0));
        let picture = app.avatar(&contact.id);
        widgets::paint_avatar(
            ui,
            &palette,
            avatar_rect,
            &name,
            &contact.id,
            picture.as_deref(),
        );
        let left = rect.left() + 76.0;
        let name_line = widgets::line(
            ui,
            &name,
            theme::medium(14.5),
            palette.text,
            rect.right() - 14.0 - left,
            1,
        );
        name_line.paint(ui, pos2(left, rect.top() + 14.0), palette.text);
        if let Some(phone) = crate::model::phone_of(&contact.id) {
            let phone_line = widgets::line(
                ui,
                &format!("+{phone}"),
                theme::regular(13.0),
                palette.dim,
                rect.right() - 14.0 - left,
                1,
            );
            phone_line.paint(ui, pos2(left, rect.top() + 38.0), palette.dim);
        }
        ui.painter().hline(
            left..=rect.right(),
            rect.bottom() - 0.5,
            egui::Stroke::new(1.0, palette.outline),
        );
    }
    let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
    if response.clicked() {
        app.actions.push(Action::StartChat {
            id: contact.id.clone(),
            name,
        });
    }
}

fn archive_row(app: &mut App, ui: &mut egui::Ui, count: usize) {
    let palette = app.palette;
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), theme::ROW_HEIGHT),
        Sense::click(),
    );
    if ui.is_rect_visible(rect) {
        if response.hovered() {
            ui.painter().rect_filled(rect, 0.0, palette.surface_hover);
        }
        let icon_rect =
            Rect::from_center_size(pos2(rect.left() + 38.0, rect.center().y), Vec2::splat(22.0));
        Icon::Archive
            .image(palette.accent, 22.0)
            .paint_at(ui, icon_rect);
        ui.painter().text(
            pos2(rect.left() + 76.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            "Archived",
            theme::medium(14.5),
            palette.text,
        );
        ui.painter().text(
            pos2(rect.right() - 16.0, rect.center().y),
            egui::Align2::RIGHT_CENTER,
            count.to_string(),
            theme::regular(12.5),
            palette.accent,
        );
        ui.painter().hline(
            (rect.left() + 76.0)..=rect.right(),
            rect.bottom() - 0.5,
            egui::Stroke::new(1.0, palette.outline),
        );
    }
    if response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
    {
        app.show_archived = true;
    }
}

fn row(app: &mut App, ui: &mut egui::Ui, chat: &Chat, index: usize) -> egui::Response {
    let palette = app.palette;
    let title = app.chat_title(chat);
    let selected = app.open_chat.as_deref() == Some(chat.id.as_str());
    let now = crate::util::now();
    let muted = chat.muted(now);
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), theme::ROW_HEIGHT),
        Sense::click(),
    );
    // A press on a pinned row starts the hold that can move it. The rest of
    // the gesture runs before the rows are drawn, in `pin_gesture`.
    if chat.pinned
        && app.can_reorder_pinned()
        && app.pin_drag.is_none()
        && response.is_pointer_button_down_on()
        && ui.input(|input| input.pointer.primary_pressed())
    {
        app.pin_drag = Some(crate::model::PinDrag {
            chat: chat.id.clone(),
            from: index,
            to: index,
            since: ui.input(|input| input.time),
            active: false,
            grab_y: ui
                .input(|input| input.pointer.interact_pos())
                .map_or(0.0, |pointer| pointer.y - rect.top()),
        });
    }
    // While a pinned chat is held, every pinned row shows its handle and the
    // pointer says it can be grabbed.
    let moving = app.pin_drag.as_ref().is_some_and(|drag| drag.active) && chat.pinned;
    let held = app
        .pin_drag
        .as_ref()
        .is_some_and(|drag| drag.active && drag.chat == chat.id);
    let response = response.on_hover_cursor(if moving {
        egui::CursorIcon::Grabbing
    } else {
        egui::CursorIcon::PointingHand
    });
    if ui.is_rect_visible(rect) {
        if selected {
            ui.painter().rect_filled(rect, 0.0, palette.surface_active);
        } else if moving {
            ui.painter()
                .rect_filled(rect, 0.0, palette.accent.gamma_multiply(0.16));
        } else if response.hovered() {
            ui.painter().rect_filled(rect, 0.0, palette.surface_hover);
        }
        if moving {
            theme::paint_icon(
                ui,
                Icon::Menu,
                Rect::from_center_size(
                    pos2(rect.left() + 12.0, rect.center().y),
                    Vec2::splat(16.0),
                ),
                16.0,
                palette.dim,
            );
        }
        let avatar_rect = Rect::from_center_size(
            pos2(
                rect.left() + if moving { 58.0 } else { 38.0 },
                rect.center().y,
            ),
            Vec2::splat(48.0),
        );
        let picture = app.avatar(&chat.id);
        widgets::paint_avatar(
            ui,
            &palette,
            avatar_rect,
            &title,
            &chat.id,
            picture.as_deref(),
        );
        if chat.ephemeral_expiration.is_some() {
            widgets::paint_disappearing_badge(ui, &palette, avatar_rect);
        }

        let left = rect.left() + if moving { 96.0 } else { 76.0 };
        let right = rect.right() - 14.0;
        let stamp = if chat.last_activity > 0 {
            crate::util::chat_stamp(chat.last_activity)
        } else {
            String::new()
        };
        let unread = chat.unread > 0;
        let stamp_color = if unread && !muted {
            palette.accent
        } else {
            palette.dim
        };
        let stamp_galley = ui
            .painter()
            .layout_no_wrap(stamp, theme::regular(11.5), stamp_color);
        let name_top = rect.top() + 14.0;
        ui.painter().galley(
            pos2(right - stamp_galley.size().x, name_top + 1.0),
            stamp_galley.clone(),
            stamp_color,
        );
        let name_width = (right - stamp_galley.size().x - 8.0 - left).max(0.0);
        let name_font = if unread {
            theme::semibold(14.5)
        } else {
            theme::medium(14.5)
        };
        let name = widgets::line(ui, &title, name_font, palette.text, name_width, 1);
        name.paint(ui, pos2(left, name_top), palette.text);

        // Leave room for badges beside the latest-message preview.
        let mut badge_right = right;
        let line_y = rect.top() + 38.0;
        if unread {
            let width = widgets::badge(
                ui,
                &palette,
                pos2(badge_right - 10.0, line_y + 8.0),
                chat.unread,
                muted,
            );
            badge_right -= width + 6.0;
        }
        if muted {
            let icon_rect =
                Rect::from_center_size(pos2(badge_right - 8.0, line_y + 8.0), Vec2::splat(15.0));
            Icon::VolumeX
                .image(palette.dim, 15.0)
                .paint_at(ui, icon_rect);
            badge_right -= 20.0;
        }
        if chat.pinned {
            let icon_rect =
                Rect::from_center_size(pos2(badge_right - 8.0, line_y + 8.0), Vec2::splat(14.0));
            Icon::Pin.image(palette.dim, 14.0).paint_at(ui, icon_rect);
            badge_right -= 20.0;
        }
        let mut x = left;
        let typing = app.typing_in(&chat.id);
        let preview_color = if unread && !muted {
            palette.secondary
        } else {
            palette.dim
        };
        let preview = if !typing.is_empty() {
            let who = if chat.is_group() {
                format!("{} is typing…", typing[0].1.trim_start_matches('~'))
            } else {
                "typing…".to_owned()
            };
            widgets::line(
                ui,
                &who,
                theme::medium(13.0),
                palette.accent,
                badge_right - x,
                1,
            )
        } else if let Some(last) = &chat.last {
            if last.from_me {
                let tick_rect =
                    Rect::from_center_size(pos2(x + 8.0, line_y + 8.0), Vec2::splat(16.0));
                widgets::ticks(ui, &palette, tick_rect, last.status);
                x += 20.0;
            } else if chat.is_group() {
                let sender = app.display_name_or(&last.sender, last.sender_name.as_deref());
                let first = sender.split_whitespace().next().unwrap_or(&sender);
                let sender = widgets::line(
                    ui,
                    &format!("{first}: "),
                    theme::regular(13.0),
                    preview_color,
                    (badge_right - x) * 0.5,
                    1,
                );
                let width = sender.size().x;
                sender.paint(ui, pos2(x, line_y), preview_color);
                x += width;
            }
            widgets::line(
                ui,
                &crate::markup::plain(&app.resolve_mention_tokens(&last.summary), &[]),
                theme::regular(13.0),
                preview_color,
                (badge_right - x).max(0.0),
                1,
            )
        } else {
            widgets::line(ui, "", theme::regular(13.0), preview_color, 1.0, 1)
        };
        preview.paint(ui, pos2(x, line_y), preview_color);
        ui.painter().hline(
            left..=rect.right(),
            rect.bottom() - 0.5,
            egui::Stroke::new(1.0, palette.outline),
        );
        // The held chat rides under the pointer: its slot shows the gap it
        // leaves, so the list reads as moving instead of copied.
        if held {
            ui.painter().rect_filled(rect, 0.0, palette.panel);
            ui.painter()
                .rect_filled(rect.shrink(8.0), 10.0, palette.surface_hover);
        }
    }
    // While a pinned chat is held, a release moves it instead of opening it:
    // egui still counts a hold of up to 0.8 s as a click.
    let holding = app.pin_drag.as_ref().is_some_and(|drag| drag.active);
    if !holding && response.clicked() {
        app.actions.push(Action::OpenChat(chat.id.clone()));
    }
    let menu_palette = palette;
    egui::Popup::context_menu(&response)
        .frame(widgets::menu_frame(&menu_palette))
        .show(|ui| {
            ui.set_min_width(190.0);
            context_menu(app, ui, chat, &menu_palette);
        });
    response
}

fn context_menu(app: &mut App, ui: &mut egui::Ui, chat: &Chat, palette: &Palette) {
    if chat.unread > 0 && widgets::menu_item(ui, palette, Some(Icon::CheckCheck), "Mark as read") {
        app.actions.push(Action::MarkRead(chat.id.clone()));
    }
    if widgets::menu_item(
        ui,
        palette,
        Some(if chat.pinned { Icon::PinOff } else { Icon::Pin }),
        if chat.pinned { "Unpin" } else { "Pin to top" },
    ) {
        app.actions
            .push(Action::SetPinned(chat.id.clone(), !chat.pinned));
    }
    if widgets::menu_item(
        ui,
        palette,
        Some(Icon::Archive),
        if chat.archived {
            "Unarchive"
        } else {
            "Archive"
        },
    ) {
        app.actions
            .push(Action::SetArchived(chat.id.clone(), !chat.archived));
    }
    let now = crate::util::now();
    if chat.muted(now) {
        if widgets::menu_item(ui, palette, Some(Icon::Bell), "Unmute") {
            app.actions.push(Action::SetMuted(chat.id.clone(), None));
        }
    } else {
        for (label, until) in [
            ("Mute for 8 hours", Some(now + 8 * 3600)),
            ("Mute for a week", Some(now + 7 * 86_400)),
            ("Mute indefinitely", Some(0)),
        ] {
            if widgets::menu_item(ui, palette, Some(Icon::BellOff), label) {
                app.actions.push(Action::SetMuted(chat.id.clone(), until));
            }
        }
    }
    widgets::menu_separator(ui, palette);
    if let Some(phone) = chat.phone()
        && widgets::menu_item(ui, palette, Some(Icon::Copy), "Copy number")
    {
        app.actions.push(Action::CopyText(format!("+{phone}")));
    }
    if widgets::menu_item(ui, palette, Some(Icon::Info), "Info") {
        app.actions
            .push(Action::ShowDialog(Dialog::ChatInfo(chat.id.clone())));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::AppDirs;
    use crate::settings::Settings;

    #[test]
    fn the_pointer_moves_one_row_to_change_slot() {
        let stride = theme::ROW_HEIGHT + 6.0;
        let grab = 34.0;
        let pinned = 3;
        // Pressed in the middle of the second pinned row, with the archived
        // row above it: the slot must read 1, not 2.
        let pressed = 2.0 * stride + grab;
        assert_eq!(pinned_slot(pressed, grab, 0.0, 0.0, stride, 1, pinned), 1);
        // A third of a row up stays where it is.
        assert_eq!(
            pinned_slot(pressed - 24.0, grab, 0.0, 0.0, stride, 1, pinned),
            1
        );
        // A full row up is the place before, a full row down the one after.
        assert_eq!(
            pinned_slot(pressed - stride, grab, 0.0, 0.0, stride, 1, pinned),
            0
        );
        assert_eq!(
            pinned_slot(pressed + stride, grab, 0.0, 0.0, stride, 1, pinned),
            2
        );
        // Without the archived row the same pointer reads one higher.
        assert_eq!(pinned_slot(pressed, grab, 0.0, 0.0, stride, 0, pinned), 2);
    }

    #[test]
    fn the_hidden_sidebar_keeps_a_way_back_unless_the_setting_says_otherwise() {
        let root = std::env::temp_dir().join(format!(
            "whatsfast-compact-sidebar-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let (mut app, _events) = App::headless(AppDirs::under(&root), Settings::default());
        assert!(app.open_chat.is_none());
        assert!(
            app.compact_sidebar(),
            "the rail shows while nothing is open"
        );
        app.open_chat = Some("491700000000@s.whatsapp.net".to_owned());
        assert!(
            app.compact_sidebar(),
            "an open chat keeps the rail: the way back is always there"
        );
        app.page = crate::model::Page::Settings;
        assert!(app.compact_sidebar(), "settings keeps the rail");
        app.settings.hide_sidebar_fully = true;
        assert!(!app.compact_sidebar(), "the setting hides the bar outright");
    }

    #[test]
    fn moving_a_pinned_chat_rewrites_the_order() {
        let root = std::env::temp_dir().join(format!(
            "whatsfast-pin-order-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let (mut app, _events) = App::headless(AppDirs::under(&root), Settings::default());
        for index in 0..3 {
            let mut chat = Chat::new(
                format!("49170000{index:04}@s.whatsapp.net"),
                format!("Chat {index}"),
            );
            chat.pinned = true;
            chat.pinned_at = 100 - i64::from(index);
            app.chats.push(chat);
        }
        let ids: Vec<String> = app
            .visible_chats()
            .iter()
            .filter(|chat| chat.pinned)
            .map(|chat| chat.id.clone())
            .collect();
        assert_eq!(ids.len(), 3, "three pinned chats, newest pin first");
        assert_eq!(
            pinned_order(&app, 0, 2),
            vec![ids[1].clone(), ids[2].clone(), ids[0].clone()],
            "the top row moved to the end"
        );
        assert_eq!(
            pinned_order(&app, 2, 0),
            vec![ids[2].clone(), ids[0].clone(), ids[1].clone()],
            "the bottom row moved to the top"
        );
        assert_eq!(
            pinned_order(&app, 1, 1),
            ids,
            "the same slot leaves it alone"
        );
    }

    #[test]
    fn alt_navigation_scrolls_the_destination_chat_into_view() {
        let root = std::env::temp_dir().join(format!(
            "whatsfast-chat-list-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let (mut app, _events) = App::headless(AppDirs::under(&root), Settings::default());
        let mut first = String::new();
        let mut last = String::new();
        for index in 0..24 {
            let id = format!("49170000{index:04}@s.whatsapp.net");
            let mut chat = Chat::new(id.clone(), format!("Chat {index:02}"));
            chat.last_activity = 100 - i64::from(index);
            if index == 0 {
                first.clone_from(&id);
            }
            last.clone_from(&id);
            app.chats.push(chat);
        }
        app.open_chat = Some(first);

        let ctx = egui::Context::default();
        app.attach(&ctx);
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(360.0, 240.0))),
            events: vec![egui::Event::Key {
                key: egui::Key::ArrowUp,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::ALT,
            }],
            ..Default::default()
        };
        let mut offset = 0.0;
        let mut output = ctx.run_ui(input, |ui| {
            super::super::keys::handle(&mut app, ui.ctx());
            let scroll_id = ui.make_persistent_id(egui::IdSalt::new("chat-list"));
            list(&mut app, ui);
            offset = egui::scroll_area::State::load(ui.ctx(), scroll_id)
                .expect("chat-list scroll state")
                .offset
                .y;
        });
        output.textures_delta.clear();

        assert!(
            app.actions.contains(&Action::OpenChat(last)),
            "Alt+Up wraps to the last visible chat"
        );
        assert!(app.scroll_chat_into_view.is_none(), "reveal was consumed");
        assert!(offset > 0.0, "the list moved down to reveal the last row");
    }
}
