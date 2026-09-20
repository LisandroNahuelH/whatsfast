//! Full-window photo and video viewer with a filmstrip and action header.

use std::path::Path;

use egui::{Align2, Color32, CornerRadius, Frame, Id, Rect, Sense, UiBuilder, pos2, vec2};

use crate::animation;
use crate::app::App;
use crate::archive::ChatMedia;
use crate::i18n::{self, Key};
use crate::model::{Action, Content, Dialog, Message};
use crate::theme::{self, Icon};
use crate::ui::conversation;
use crate::ui::widgets;
use crate::util;

const MIN_ZOOM: f32 = 1.0;
const MAX_ZOOM: f32 = 8.0;
const HEADER: f32 = 56.0;
const STRIP: f32 = 76.0;
const THUMB: f32 = 56.0;
const STRIP_GAP: f32 = 8.0;

// #region agent log
fn agent_dbg(hypothesis_id: &str, location: &str, message: &str, data: &str) {
    use std::io::Write;
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(r"C:\OfiSync\0. Lisandro\0. Programacion\WhatsFast\debug-138664.log")
    else {
        return;
    };
    let _ = writeln!(
        file,
        "{{\"sessionId\":\"138664\",\"runId\":\"post-fix\",\"hypothesisId\":\"{hypothesis_id}\",\"location\":\"{location}\",\"message\":\"{message}\",\"data\":{data},\"timestamp\":{ts}}}"
    );
}
// #endregion

#[derive(Clone, Debug)]
pub struct ImageViewer {
    pub chat: String,
    pub message: String,
    pub zoom: f32,
    pub offset: egui::Vec2,
    dragging: bool,
    wheel: f32,
    pinch: f32,
}

impl ImageViewer {
    pub fn open(chat: String, message: String) -> Self {
        Self {
            chat,
            message,
            zoom: 1.0,
            offset: egui::Vec2::ZERO,
            dragging: false,
            wheel: 0.0,
            pinch: 1.0,
        }
    }
}

/// Path of a downloaded image or non-GIF video.
pub fn media_path(message: &Message) -> Option<&Path> {
    match &message.content {
        Content::Image { media, .. } => media.path.as_deref(),
        Content::Video {
            gif: false, media, ..
        } => media.path.as_deref(),
        _ => None,
    }
}

/// Neighbor image or video in a loaded page. Does not wrap.
pub fn neighbor_image<'a>(messages: &'a [Message], current: &str, step: i8) -> Option<&'a str> {
    let ids: Vec<&str> = messages
        .iter()
        .filter(|message| is_gallery_item(&message.content))
        .map(|message| message.id.as_str())
        .collect();
    neighbor_ids(&ids, current, step)
}

fn is_gallery_item(content: &Content) -> bool {
    matches!(
        content,
        Content::Image { .. } | Content::Video { gif: false, .. }
    )
}

fn neighbor_ids<'a>(ids: &[&'a str], current: &str, step: i8) -> Option<&'a str> {
    let index = ids.iter().position(|id| *id == current)?;
    let next = index.checked_add_signed(step as isize)?;
    ids.get(next).copied()
}

/// Header, photo stage, and filmstrip inside the window.
fn panes(window: Rect) -> (Rect, Rect, Rect) {
    let header = Rect::from_min_max(window.min, pos2(window.max.x, window.min.y + HEADER));
    let strip = Rect::from_min_max(pos2(window.min.x, window.max.y - STRIP), window.max);
    let stage = Rect::from_min_max(
        pos2(window.min.x, header.max.y),
        pos2(window.max.x, strip.min.y),
    );
    (header, stage, strip)
}

fn strip_side_pad(viewport_w: f32) -> f32 {
    ((viewport_w - THUMB) * 0.5).max(0.0)
}

fn chevron_hit(stage: Rect, left: bool) -> Rect {
    let x = if left {
        stage.left() + 28.0
    } else {
        stage.right() - 28.0
    };
    Rect::from_center_size(pos2(x, stage.center().y), vec2(40.0, 40.0))
}

fn click_closes_viewer(pos: egui::Pos2, stage: Rect, media: Rect, left: Rect, right: Rect) -> bool {
    stage.contains(pos) && !media.contains(pos) && !left.contains(pos) && !right.contains(pos)
}

pub fn neighbor_media<'a>(items: &'a [ChatMedia], current: &str, step: i8) -> Option<&'a str> {
    let ids: Vec<&str> = items.iter().map(|item| item.id.as_str()).collect();
    neighbor_ids(&ids, current, step)
}

/// Consumes the wheel over the viewer so the chat behind it does not scroll.
pub fn intercept_scroll(app: &mut App, ctx: &egui::Context) {
    let Some(viewer) = app.image_viewer.as_mut() else {
        return;
    };
    ctx.input_mut(|input| {
        viewer.wheel = input.smooth_scroll_delta.y;
        viewer.pinch = input.zoom_delta();
        input.smooth_scroll_delta = egui::Vec2::ZERO;
    });
}

pub fn show(app: &mut App, ctx: &egui::Context) {
    let Some(mut viewer) = app.image_viewer.clone() else {
        return;
    };
    let mut actions = Vec::new();
    let mut zoom_by = 0.0_f32;
    let palette = app.palette;
    egui::Modal::new(Id::new("image-viewer"))
        .frame(Frame::new().fill(Color32::from_black_alpha(220)))
        .backdrop_color(Color32::from_black_alpha(220))
        .show(ctx, |ui| {
            let screen = ui.ctx().content_rect();
            ui.set_min_size(screen.size());
            ui.expand_to_include_rect(screen);
            let (header, stage, strip) = panes(screen);
            let response = ui.interact(stage, ui.id().with("stage"), Sense::click_and_drag());
            let media = paint_stage(app, ui, &mut viewer, stage, &response);
            paint_header(app, ui, &viewer, header, &mut actions, &mut zoom_by);
            paint_strip(app, ui, &viewer, strip, &mut actions);
            paint_chevrons(ui, &palette, stage, &mut actions);
            if response.clicked()
                && let Some(pos) = response.interact_pointer_pos()
            {
                let left = chevron_hit(stage, true);
                let right = chevron_hit(stage, false);
                let close = click_closes_viewer(pos, stage, media, left, right);
                // #region agent log
                agent_dbg(
                    "C",
                    "viewer.rs:show",
                    "stage click",
                    &format!(
                        "{{\"close\":{close},\"in_media\":{},\"in_chevron\":{}}}",
                        media.contains(pos),
                        left.contains(pos) || right.contains(pos)
                    ),
                );
                // #endregion
                if close {
                    actions.push(Action::CloseImageViewer);
                }
            }
        });
    if zoom_by != 0.0 {
        viewer.zoom = (viewer.zoom * zoom_by).clamp(MIN_ZOOM, MAX_ZOOM);
        if viewer.zoom <= MIN_ZOOM {
            viewer.offset = egui::Vec2::ZERO;
        }
    }
    app.image_viewer = Some(viewer);
    app.actions.extend(actions);
}

fn paint_header(
    app: &mut App,
    ui: &mut egui::Ui,
    viewer: &ImageViewer,
    rect: Rect,
    actions: &mut Vec<Action>,
    zoom_by: &mut f32,
) {
    let palette = app.palette;
    ui.painter()
        .rect_filled(rect, 0.0, Color32::from_black_alpha(140));
    let mut child = ui.new_child(
        UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    child.set_clip_rect(rect);
    if theme::macos_chrome(child.ctx()) {
        child.add_space(theme::traffic_light_inset(child.ctx()).max(8.0));
    } else {
        child.add_space(12.0);
    }
    let chat = app.chat(&viewer.chat).cloned();
    let title = chat
        .as_ref()
        .map(|chat| app.chat_title(chat))
        .unwrap_or_else(|| app.display_name_or(&viewer.chat, None));
    let picture = app.avatar(&viewer.chat);
    let (subtitle, color) = chat
        .as_ref()
        .map(|chat| conversation::subtitle(app, chat))
        .unwrap_or((String::new(), Color32::from_white_alpha(180)));
    widgets::avatar(
        &mut child,
        &palette,
        &title,
        &viewer.chat,
        36.0,
        picture.as_deref(),
    );
    child.add_space(8.0);
    child.vertical(|ui| {
        ui.add_space(8.0);
        widgets::rich_text(ui, &title, theme::semibold(15.0), Color32::WHITE);
        if !subtitle.is_empty() {
            widgets::rich_text(ui, &subtitle, theme::regular(12.0), color);
        }
    });
    child.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        ui.add_space(8.0);
        ui.spacing_mut().item_spacing.x = 2.0;
        let hover = Color32::WHITE;
        let idle = Color32::from_white_alpha(200);
        if theme::icon_button(ui, Icon::X, 18.0, idle, hover, i18n::t(Key::ViewerClose)).clicked() {
            actions.push(Action::CloseImageViewer);
        }
        if theme::icon_button(
            ui,
            Icon::Download,
            18.0,
            idle,
            hover,
            i18n::t(Key::CommonDownload),
        )
        .clicked()
        {
            actions.push(Action::DownloadSelected {
                chat: viewer.chat.clone(),
                messages: vec![viewer.message.clone()],
            });
        }
        if theme::icon_button(
            ui,
            Icon::Forward,
            18.0,
            idle,
            hover,
            i18n::t(Key::CommonForward),
        )
        .clicked()
        {
            actions.push(Action::ShowDialog(Dialog::Forward {
                chat: viewer.chat.clone(),
                messages: vec![viewer.message.clone()],
            }));
        }
        if theme::icon_button(
            ui,
            Icon::Smile,
            18.0,
            idle,
            hover,
            i18n::t(Key::ViewerReact),
        )
        .clicked()
        {
            actions.push(Action::OpenReactionPicker {
                chat: viewer.chat.clone(),
                message: viewer.message.clone(),
            });
        }
        let pinned = app
            .pins
            .get(&viewer.chat)
            .is_some_and(|ids| ids.contains(&viewer.message));
        if theme::icon_button(
            ui,
            Icon::Pin,
            18.0,
            if pinned { palette.accent } else { idle },
            hover,
            if pinned {
                i18n::t(Key::ChatUnpinMessage)
            } else {
                i18n::t(Key::ChatPinMessage)
            },
        )
        .clicked()
        {
            actions.push(Action::SetMessagePinned {
                chat: viewer.chat.clone(),
                message: viewer.message.clone(),
                pinned: !pinned,
            });
        }
        let starred = app
            .stars
            .get(&viewer.chat)
            .is_some_and(|ids| ids.contains(&viewer.message));
        if theme::icon_button(
            ui,
            Icon::Star,
            18.0,
            if starred { palette.accent } else { idle },
            hover,
            if starred {
                i18n::t(Key::ChatUnstar)
            } else {
                i18n::t(Key::ChatStar)
            },
        )
        .clicked()
        {
            actions.push(Action::StarSelected {
                chat: viewer.chat.clone(),
                messages: vec![viewer.message.clone()],
                starred: !starred,
            });
        }
        if theme::icon_button(ui, Icon::Reply, 18.0, idle, hover, i18n::t(Key::ChatReply)).clicked()
        {
            actions.push(Action::ReplyFromViewer {
                chat: viewer.chat.clone(),
                message: viewer.message.clone(),
            });
        }
        if theme::icon_button(
            ui,
            Icon::MessageCircle,
            18.0,
            idle,
            hover,
            i18n::t(Key::ViewerGoToMessage),
        )
        .clicked()
        {
            actions.push(Action::CloseImageViewer);
            actions.push(Action::OpenMessage {
                chat: viewer.chat.clone(),
                message: viewer.message.clone(),
            });
        }
        if theme::icon_button(
            ui,
            Icon::Minus,
            18.0,
            idle,
            hover,
            i18n::t(Key::ViewerZoomOut),
        )
        .clicked()
        {
            *zoom_by = 1.0 / 1.25;
        }
        if theme::icon_button(
            ui,
            Icon::Plus,
            18.0,
            idle,
            hover,
            i18n::t(Key::ViewerZoomIn),
        )
        .clicked()
        {
            *zoom_by = 1.25;
        }
    });
}

fn paint_stage(
    app: &mut App,
    ui: &mut egui::Ui,
    viewer: &mut ImageViewer,
    stage: Rect,
    response: &egui::Response,
) -> Rect {
    let item = app
        .viewer_media
        .iter()
        .find(|item| item.id == viewer.message)
        .cloned();
    let message = app
        .conversations
        .get(&viewer.chat)
        .and_then(|conversation| conversation.message(&viewer.message))
        .cloned();
    let video = item.as_ref().is_some_and(|item| item.video)
        || message
            .as_ref()
            .is_some_and(|message| matches!(message.content, Content::Video { gif: false, .. }));
    let path = item
        .as_ref()
        .and_then(|item| item.path.clone())
        .or_else(|| message.as_ref().and_then(media_path).map(Path::to_path_buf));
    let thumbnail = item
        .as_ref()
        .and_then(|item| item.thumbnail.as_deref())
        .or_else(|| {
            message
                .as_ref()
                .and_then(|message| message.thumbnail.as_deref())
        });

    let pointer = response.hover_pos();
    if !video {
        let wheel = viewer.wheel;
        let pinch = viewer.pinch;
        viewer.wheel = 0.0;
        viewer.pinch = 1.0;
        let factor = if (pinch - 1.0).abs() > f32::EPSILON {
            pinch
        } else if wheel.abs() > f32::EPSILON {
            1.1_f32.powf(wheel / 40.0)
        } else {
            1.0
        };
        if (factor - 1.0).abs() > f32::EPSILON
            && let Some(pointer) = pointer
            && stage.contains(pointer)
        {
            let next = (viewer.zoom * factor).clamp(MIN_ZOOM, MAX_ZOOM);
            let origin = stage.center() + viewer.offset;
            viewer.offset += (pointer - origin) * (1.0 - next / viewer.zoom);
            viewer.zoom = next;
            if viewer.zoom <= MIN_ZOOM {
                viewer.offset = egui::Vec2::ZERO;
            }
        }
        if response.dragged() && viewer.zoom > MIN_ZOOM {
            viewer.dragging = true;
            viewer.offset += response.drag_delta();
        } else if response.drag_stopped() {
            viewer.dragging = false;
        }
    }

    let inner = stage.shrink(12.0);
    if !ui.is_rect_visible(inner) {
        return inner;
    }
    if video {
        paint_video(
            ui,
            path.as_deref(),
            thumbnail,
            &viewer.chat,
            &viewer.message,
            inner,
        );
        return inner;
    }
    if let Some(path) = path.as_deref() {
        let image = egui::Image::new(util::image_uri(path));
        match image.load_for_size(ui.ctx(), inner.size()) {
            Ok(egui::load::TexturePoll::Ready { texture }) => {
                let fitted = fit(texture.size, inner.size());
                let size = fitted * viewer.zoom;
                let rect = Rect::from_center_size(inner.center() + viewer.offset, size);
                image.fit_to_exact_size(size).paint_at(ui, rect);
                return rect;
            }
            Ok(egui::load::TexturePoll::Pending { .. }) => {
                theme::paint_spinner(ui, inner, 28.0, Color32::WHITE);
            }
            Err(_) => {
                paint_placeholder(ui, thumbnail, &viewer.chat, &viewer.message, inner, false);
            }
        }
        return inner;
    }
    paint_placeholder(ui, thumbnail, &viewer.chat, &viewer.message, inner, false);
    inner
}

fn paint_video(
    ui: &mut egui::Ui,
    path: Option<&Path>,
    thumbnail: Option<&[u8]>,
    chat: &str,
    id: &str,
    rect: Rect,
) {
    if let Some(path) = path {
        match animation::frame(ui, path, rect) {
            animation::Frame::Ready(texture) => {
                ui.painter().image(
                    texture.id(),
                    rect,
                    Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                    Color32::WHITE,
                );
                return;
            }
            animation::Frame::Pending => {
                paint_placeholder(ui, thumbnail, chat, id, rect, true);
                let disc = Rect::from_center_size(rect.center(), vec2(48.0, 48.0));
                theme::paint_spinner(ui, disc, 24.0, Color32::WHITE);
                return;
            }
            animation::Frame::Unavailable => {}
        }
    }
    paint_placeholder(ui, thumbnail, chat, id, rect, true);
    let disc = Rect::from_center_size(rect.center(), vec2(56.0, 56.0));
    ui.painter()
        .circle_filled(disc.center(), 28.0, Color32::from_black_alpha(140));
    theme::paint_icon(ui, Icon::Play, disc, 28.0, Color32::WHITE);
}

fn paint_placeholder(
    ui: &mut egui::Ui,
    thumbnail: Option<&[u8]>,
    chat: &str,
    id: &str,
    rect: Rect,
    video: bool,
) {
    if let Some(bytes) = thumbnail {
        let uri = conversation::thumbnail_uri(ui.ctx(), chat, id, bytes);
        egui::Image::new(uri)
            .fit_to_exact_size(rect.size())
            .paint_at(ui, rect);
        return;
    }
    ui.painter()
        .rect_filled(rect, 8.0, Color32::from_white_alpha(12));
    theme::paint_icon(
        ui,
        if video { Icon::Video } else { Icon::Image },
        rect,
        42.0,
        Color32::from_white_alpha(180),
    );
    ui.painter().text(
        rect.center() + vec2(0.0, 40.0),
        Align2::CENTER_CENTER,
        i18n::t(Key::ViewerCouldNotDisplay),
        theme::regular(13.0),
        Color32::from_white_alpha(180),
    );
}

fn paint_strip(
    app: &mut App,
    ui: &mut egui::Ui,
    viewer: &ImageViewer,
    rect: Rect,
    actions: &mut Vec<Action>,
) {
    let palette = app.palette;
    ui.painter()
        .rect_filled(rect, 0.0, Color32::from_black_alpha(160));
    let items = app.viewer_media.clone();
    if items.is_empty() {
        return;
    }
    let inner = rect.shrink2(vec2(8.0, 8.0));
    let pad = strip_side_pad(inner.width());
    let mut child = ui.new_child(UiBuilder::new().max_rect(inner));
    egui::ScrollArea::horizontal()
        .id_salt("viewer-strip")
        .auto_shrink([false, false])
        .show(&mut child, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.set_min_height(THUMB);
                ui.add_space(pad);
                for (index, item) in items.iter().enumerate() {
                    if index > 0 {
                        ui.add_space(STRIP_GAP);
                    }
                    let (thumb, response) =
                        ui.allocate_exact_size(vec2(THUMB, THUMB), Sense::click());
                    if item.id == viewer.message {
                        ui.scroll_to_rect_animation(
                            thumb,
                            Some(egui::Align::Center),
                            egui::style::ScrollAnimation::none(),
                        );
                        // #region agent log
                        {
                            use std::sync::atomic::{AtomicU32, Ordering};
                            static FRAMES: AtomicU32 = AtomicU32::new(0);
                            let n = FRAMES.fetch_add(1, Ordering::Relaxed);
                            if n < 8 || n.is_multiple_of(30) {
                                let cx = inner.center().x;
                                agent_dbg(
                                    "A",
                                    "viewer.rs:paint_strip",
                                    "thumb centre",
                                    &format!(
                                        "{{\"n\":{n},\"dx\":{:.1},\"index\":{index},\"count\":{},\"last\":{}}}",
                                        thumb.center().x - cx,
                                        items.len(),
                                        index + 1 == items.len()
                                    ),
                                );
                            }
                        }
                        // #endregion
                    }
                    if ui.is_rect_visible(thumb) {
                        paint_thumb(ui, &palette, item, thumb, item.id == viewer.message);
                    }
                    if response
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        actions.push(Action::ViewImage {
                            chat: viewer.chat.clone(),
                            message: item.id.clone(),
                        });
                    }
                }
                ui.add_space(pad);
            });
        });
}

fn paint_thumb(
    ui: &egui::Ui,
    palette: &crate::theme::Palette,
    item: &ChatMedia,
    rect: Rect,
    current: bool,
) {
    ui.painter()
        .rect_filled(rect, 6.0, Color32::from_white_alpha(16));
    if let Some(path) = item.path.as_deref() {
        egui::Image::new(util::image_uri(path))
            .fit_to_exact_size(rect.size())
            .corner_radius(6.0)
            .paint_at(ui, rect);
    } else if let Some(bytes) = item.thumbnail.as_deref() {
        let uri = conversation::thumbnail_uri(ui.ctx(), "thumb", &item.id, bytes);
        egui::Image::new(uri)
            .fit_to_exact_size(rect.size())
            .corner_radius(6.0)
            .paint_at(ui, rect);
    } else {
        theme::paint_icon(
            ui,
            if item.video { Icon::Video } else { Icon::Image },
            rect,
            22.0,
            Color32::from_white_alpha(180),
        );
    }
    if item.video {
        let disc = Rect::from_center_size(rect.center(), vec2(18.0, 18.0));
        ui.painter()
            .circle_filled(disc.center(), 9.0, Color32::from_black_alpha(140));
        theme::paint_icon(ui, Icon::Play, disc, 12.0, Color32::WHITE);
    }
    if current {
        ui.painter().rect_stroke(
            rect,
            CornerRadius::same(6),
            egui::Stroke::new(2.0, palette.accent),
            egui::StrokeKind::Inside,
        );
    }
}

fn paint_chevrons(
    ui: &mut egui::Ui,
    palette: &crate::theme::Palette,
    stage: Rect,
    actions: &mut Vec<Action>,
) {
    let mut left = ui.new_child(UiBuilder::new().max_rect(chevron_hit(stage, true)).layout(
        egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
    ));
    if theme::circle_button(
        &mut left,
        Icon::ChevronLeft,
        36.0,
        Color32::from_black_alpha(120),
        palette.surface_hover,
        Color32::WHITE,
        i18n::t(Key::ViewerPrevious),
    )
    .clicked()
    {
        actions.push(Action::StepImage(-1));
    }
    let mut right = ui.new_child(UiBuilder::new().max_rect(chevron_hit(stage, false)).layout(
        egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
    ));
    if theme::circle_button(
        &mut right,
        Icon::ChevronRight,
        36.0,
        Color32::from_black_alpha(120),
        palette.surface_hover,
        Color32::WHITE,
        i18n::t(Key::ViewerNext),
    )
    .clicked()
    {
        actions.push(Action::StepImage(1));
    }
}

fn fit(size: egui::Vec2, max: egui::Vec2) -> egui::Vec2 {
    let scale = (max.x / size.x).min(max.y / size.y).min(1.0);
    size * scale
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Media, Message};

    #[test]
    fn panes_keep_a_positive_stage_inside_the_window() {
        let window = Rect::from_min_size(pos2(0.0, 0.0), vec2(1600.0, 880.0));
        let (header, stage, strip) = panes(window);
        assert_eq!(header.height(), HEADER);
        assert_eq!(strip.height(), STRIP);
        assert!(
            stage.height() > 600.0,
            "stage collapsed: {}",
            stage.height()
        );
        assert_eq!(stage.width(), 1600.0);
        assert!(stage.max.y <= strip.min.y + f32::EPSILON);
    }

    #[test]
    fn strip_padding_lets_the_last_thumb_sit_on_the_viewport_centre() {
        let viewport = 1600.0;
        let count = 50_usize;
        let pad = strip_side_pad(viewport);
        let first_center = pad + THUMB * 0.5;
        assert!((first_center - viewport * 0.5).abs() < 0.01);
        let last_center = pad + (count as f32 - 1.0) * (THUMB + STRIP_GAP) + THUMB * 0.5;
        let content = pad * 2.0 + count as f32 * THUMB + (count as f32 - 1.0) * STRIP_GAP;
        let needed = last_center - viewport * 0.5;
        assert!((needed - (content - viewport)).abs() < 0.01);
    }

    #[test]
    fn a_click_on_the_dimmed_stage_closes_and_a_click_on_the_photo_does_not() {
        let stage = Rect::from_min_size(pos2(0.0, 56.0), vec2(1600.0, 748.0));
        let media = Rect::from_center_size(stage.center(), vec2(400.0, 300.0));
        let left = chevron_hit(stage, true);
        let right = chevron_hit(stage, false);
        assert!(click_closes_viewer(
            pos2(800.0, stage.min.y + 20.0),
            stage,
            media,
            left,
            right
        ));
        assert!(!click_closes_viewer(
            media.center(),
            stage,
            media,
            left,
            right
        ));
        assert!(!click_closes_viewer(
            left.center(),
            stage,
            media,
            left,
            right
        ));
        assert!(!click_closes_viewer(
            pos2(800.0, 20.0),
            stage,
            media,
            left,
            right
        ));
    }

    fn photo(id: &str, path: Option<&str>) -> Message {
        Message {
            id: id.into(),
            chat: "c".into(),
            sender: "c".into(),
            sender_name: None,
            from_me: false,
            timestamp: 0,
            content: Content::Image {
                caption: None,
                media: Media {
                    mime: "image/jpeg".into(),
                    size: 1,
                    path: path.map(std::path::PathBuf::from),
                    ..Default::default()
                },
            },
            status: crate::model::Delivery::None,
            delivered_at: None,
            read_at: None,
            quoted: None,
            reactions: Vec::new(),
            edited: false,
            mentions: Vec::new(),
            forwarded: false,
            thumbnail: None,
            revoked_at: None,
        }
    }

    fn video(id: &str, gif: bool) -> Message {
        Message {
            content: Content::Video {
                caption: None,
                media: Media {
                    mime: "video/mp4".into(),
                    size: 1,
                    ..Default::default()
                },
                seconds: Some(1),
                gif,
            },
            ..photo(id, None)
        }
    }

    #[test]
    fn neighbor_walks_photos_and_videos_and_skips_gifs() {
        let messages = vec![
            photo("a", Some("a.jpg")),
            video("clip", false),
            video("loop", true),
            photo("b", None),
        ];
        assert_eq!(neighbor_image(&messages, "a", 1), Some("clip"));
        assert_eq!(neighbor_image(&messages, "clip", 1), Some("b"));
        assert_eq!(neighbor_image(&messages, "b", 1), None);
        assert_eq!(neighbor_image(&messages, "clip", -1), Some("a"));
    }

    #[test]
    fn neighbor_media_walks_archive_rows_without_wrapping() {
        let items = vec![
            ChatMedia {
                id: "a".into(),
                timestamp: 1,
                video: false,
                path: None,
                thumbnail: None,
            },
            ChatMedia {
                id: "clip".into(),
                timestamp: 2,
                video: true,
                path: None,
                thumbnail: None,
            },
        ];
        assert_eq!(neighbor_media(&items, "a", 1), Some("clip"));
        assert_eq!(neighbor_media(&items, "clip", 1), None);
        assert_eq!(neighbor_media(&items, "clip", -1), Some("a"));
        assert_eq!(neighbor_media(&items, "missing", 1), None);
    }
}
