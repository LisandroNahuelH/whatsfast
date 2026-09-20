//! Full-window photo viewer: zoom, pan, and previous/next in the open chat.

use std::path::PathBuf;

use egui::{Align2, Color32, CursorIcon, Id, Modal, Order, Rect, Sense, Vec2, pos2, vec2};

use crate::app::App;
use crate::i18n::{self, Key};
use crate::model::{Action, ChatId, Content, Message};
use crate::theme::{self, Icon};

const MIN_ZOOM: f32 = 0.25;
const MAX_ZOOM: f32 = 8.0;
const FIT_MARGIN: f32 = 48.0;
const CHROME: f32 = 40.0;

/// Open photo and the user's zoom/pan within it.
#[derive(Clone, Debug)]
pub struct ImageViewer {
    pub chat: ChatId,
    pub message: String,
    /// 1.0 fits the photo in the window.
    pub zoom: f32,
    pub offset: Vec2,
    /// Scroll captured before the chat list reads it.
    wheel: f32,
    /// Pinch factor captured this frame, 1.0 when idle.
    pinch: f32,
}

impl ImageViewer {
    pub fn open(chat: ChatId, message: String) -> Self {
        Self {
            chat,
            message,
            zoom: 1.0,
            offset: Vec2::ZERO,
            wheel: 0.0,
            pinch: 1.0,
        }
    }
}

/// Stops the conversation from scrolling while the viewer is open.
pub fn intercept_scroll(app: &mut App, ctx: &egui::Context) {
    let Some(viewer) = app.image_viewer.as_mut() else {
        return;
    };
    ctx.input_mut(|input| {
        viewer.wheel = input.smooth_scroll_delta.y;
        viewer.pinch = input.zoom_delta();
        input.smooth_scroll_delta = Vec2::ZERO;
    });
}

pub fn show(app: &mut App, ctx: &egui::Context) {
    let Some(mut viewer) = app.image_viewer.clone() else {
        return;
    };
    let path = app
        .conversations
        .get(&viewer.chat)
        .and_then(|conversation| conversation.message(&viewer.message))
        .and_then(image_path)
        .cloned();
    let Some(path) = path else {
        app.actions.push(Action::CloseImageViewer);
        return;
    };
    let prev = app
        .conversations
        .get(&viewer.chat)
        .and_then(|conversation| neighbor_image(&conversation.messages, &viewer.message, -1))
        .map(str::to_owned);
    let next = app
        .conversations
        .get(&viewer.chat)
        .and_then(|conversation| neighbor_image(&conversation.messages, &viewer.message, 1))
        .map(str::to_owned);
    let palette = app.palette;
    let wheel = viewer.wheel;
    let pinch = viewer.pinch;
    viewer.wheel = 0.0;
    viewer.pinch = 1.0;

    let id = Id::new("image-viewer");
    let modal = Modal::new(id)
        .frame(egui::Frame::NONE)
        .backdrop_color(Color32::from_black_alpha(230))
        .area(
            Modal::default_area(id)
                .order(Order::Foreground)
                .anchor(Align2::LEFT_TOP, Vec2::ZERO),
        )
        .show(ctx, |ui| {
            let mut close = false;
            let mut step = 0i8;
            let view = ui.ctx().content_rect();
            ui.set_min_size(view.size());
            ui.set_max_size(view.size());
            let (full, background) = ui.allocate_exact_size(view.size(), Sense::click());
            let image = egui::Image::new(crate::util::image_uri(&path));
            let photo = match image.load_for_size(ui.ctx(), vec2(8192.0, 8192.0)) {
                Ok(egui::load::TexturePoll::Ready { texture }) => {
                    let native = texture.size;
                    let fit = fit_scale(native, full.size());
                    let factor = zoom_factor(wheel, pinch);
                    if (factor - 1.0).abs() > f32::EPSILON {
                        let cursor = ui
                            .input(|input| input.pointer.hover_pos())
                            .unwrap_or(full.center())
                            .to_vec2();
                        let (zoom, offset) = zoom_at(
                            viewer.zoom,
                            viewer.offset,
                            fit,
                            cursor,
                            full.center().to_vec2(),
                            factor,
                        );
                        viewer.zoom = zoom;
                        viewer.offset = offset;
                    }
                    let drawn = native * (fit * viewer.zoom);
                    viewer.offset = clamp_offset(viewer.offset, drawn, full.size());
                    let rect = Rect::from_center_size(full.center() + viewer.offset, drawn);
                    image.fit_to_exact_size(drawn).paint_at(ui, rect);
                    Some(ui.interact(rect, ui.id().with("photo"), Sense::click_and_drag()))
                }
                Ok(egui::load::TexturePoll::Pending { .. }) => {
                    theme::paint_spinner(ui, full, 28.0, palette.accent);
                    None
                }
                Err(_) => {
                    ui.painter().text(
                        full.center(),
                        Align2::CENTER_CENTER,
                        i18n::t(Key::ViewerCouldNotDisplay),
                        theme::regular(14.0),
                        palette.text,
                    );
                    None
                }
            };
            let mut on_photo = false;
            if let Some(photo) = photo {
                if photo.dragged() {
                    viewer.offset += photo.drag_delta();
                    viewer.offset = clamp_offset(viewer.offset, photo.rect.size(), full.size());
                }
                if photo.double_clicked() {
                    viewer.zoom = 1.0;
                    viewer.offset = Vec2::ZERO;
                }
                on_photo = photo.clicked() || photo.dragged() || photo.drag_started();
                let cursor = if photo.dragged() {
                    CursorIcon::Grabbing
                } else {
                    CursorIcon::Grab
                };
                photo.on_hover_cursor(cursor);
            }
            let inset = theme::traffic_light_inset(ui.ctx());
            let close_pos = pos2(full.right() - 28.0, full.top() + 28.0);
            if chrome_button(
                ui,
                Rect::from_center_size(close_pos, Vec2::splat(CHROME)),
                Icon::X,
                i18n::t(Key::ViewerClose),
            ) {
                close = true;
            }
            if prev.is_some()
                && chrome_button(
                    ui,
                    Rect::from_center_size(
                        pos2(full.left() + 28.0 + inset, full.center().y),
                        Vec2::splat(CHROME),
                    ),
                    Icon::ChevronLeft,
                    i18n::t(Key::ViewerPrevious),
                )
            {
                step = -1;
            }
            if next.is_some()
                && chrome_button(
                    ui,
                    Rect::from_center_size(
                        pos2(full.right() - 28.0, full.center().y),
                        Vec2::splat(CHROME),
                    ),
                    Icon::ChevronRight,
                    i18n::t(Key::ViewerNext),
                )
            {
                step = 1;
            }
            if background.clicked() && !on_photo && step == 0 && !close {
                close = true;
            }
            (viewer, close, step)
        });
    let should_close = modal.should_close();
    let (viewer, close, step) = modal.inner;
    if close || should_close {
        app.actions.push(Action::CloseImageViewer);
    } else if step != 0 {
        app.actions.push(Action::StepImage(step));
    }
    if app.image_viewer.is_some() {
        app.image_viewer = Some(viewer);
    }
}

pub(crate) fn image_path(message: &Message) -> Option<&PathBuf> {
    match &message.content {
        Content::Image { media, .. } => media.path.as_ref(),
        _ => None,
    }
}

/// Next or previous downloaded photo in `messages`. Does not wrap.
pub(crate) fn neighbor_image<'a>(
    messages: &'a [Message],
    current: &str,
    step: i8,
) -> Option<&'a str> {
    if step == 0 {
        return None;
    }
    let ids: Vec<&str> = messages.iter().filter_map(downloaded_image_id).collect();
    let index = ids.iter().position(|id| *id == current)?;
    let next = index as i64 + i64::from(step);
    if next < 0 || next >= ids.len() as i64 {
        return None;
    }
    Some(ids[next as usize])
}

fn downloaded_image_id(message: &Message) -> Option<&str> {
    image_path(message).map(|_| message.id.as_str())
}

fn fit_scale(image: Vec2, view: Vec2) -> f32 {
    let view = (view - Vec2::splat(FIT_MARGIN * 2.0)).max(Vec2::splat(1.0));
    let width = image.x.max(1.0);
    let height = image.y.max(1.0);
    (view.x / width).min(view.y / height)
}

fn zoom_factor(wheel: f32, pinch: f32) -> f32 {
    if (pinch - 1.0).abs() > f32::EPSILON {
        pinch
    } else if wheel.abs() > f32::EPSILON {
        1.1_f32.powf(wheel / 40.0)
    } else {
        1.0
    }
}

fn zoom_at(
    zoom: f32,
    offset: Vec2,
    fit: f32,
    cursor: Vec2,
    center: Vec2,
    factor: f32,
) -> (f32, Vec2) {
    let old_scale = (fit * zoom).max(f32::EPSILON);
    let new_zoom = (zoom * factor).clamp(MIN_ZOOM, MAX_ZOOM);
    let new_scale = fit * new_zoom;
    let local = (cursor - center - offset) / old_scale;
    (new_zoom, cursor - center - local * new_scale)
}

fn clamp_offset(offset: Vec2, drawn: Vec2, view: Vec2) -> Vec2 {
    let extra = ((drawn - view) * 0.5).max(Vec2::ZERO);
    vec2(
        offset.x.clamp(-extra.x, extra.x),
        offset.y.clamp(-extra.y, extra.y),
    )
}

fn chrome_button(ui: &mut egui::Ui, rect: Rect, icon: Icon, tooltip: &str) -> bool {
    let response = ui
        .interact(rect, ui.id().with(tooltip), Sense::click())
        .on_hover_cursor(CursorIcon::PointingHand)
        .on_hover_text(tooltip);
    ui.painter()
        .circle_filled(rect.center(), 16.0, Color32::from_black_alpha(140));
    let tint = if response.hovered() {
        Color32::WHITE
    } else {
        Color32::from_white_alpha(200)
    };
    theme::paint_icon(ui, icon, rect, 18.0, tint);
    response.clicked()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Delivery, Media};

    fn photo(id: &str, path: Option<&str>) -> Message {
        Message {
            id: id.into(),
            chat: "a@s.whatsapp.net".into(),
            sender: "a@s.whatsapp.net".into(),
            sender_name: None,
            from_me: false,
            timestamp: 0,
            content: Content::Image {
                caption: None,
                media: Media {
                    mime: "image/jpeg".into(),
                    size: 1,
                    width: Some(800),
                    height: Some(600),
                    path: path.map(PathBuf::from),
                    ..Default::default()
                },
            },
            status: Delivery::None,
            delivered_at: None,
            read_at: None,
            quoted: None,
            reactions: Vec::new(),
            edited: false,
            mentions: Vec::new(),
            forwarded: false,
            thumbnail: None,
        }
    }

    fn text(id: &str) -> Message {
        let mut message = photo(id, None);
        message.content = Content::text("hi");
        message
    }

    #[test]
    fn neighbor_image_walks_downloaded_photos_and_stops_at_the_ends() {
        let messages = vec![
            photo("first", Some("a.jpg")),
            text("skip"),
            photo("waiting", None),
            photo("middle", Some("b.jpg")),
            photo("last", Some("c.jpg")),
        ];
        assert_eq!(neighbor_image(&messages, "first", 1), Some("middle"));
        assert_eq!(neighbor_image(&messages, "middle", 1), Some("last"));
        assert_eq!(neighbor_image(&messages, "last", 1), None);
        assert_eq!(neighbor_image(&messages, "last", -1), Some("middle"));
        assert_eq!(neighbor_image(&messages, "first", -1), None);
        assert_eq!(neighbor_image(&messages, "missing", 1), None);
        assert_eq!(neighbor_image(&messages, "middle", 0), None);
    }

    #[test]
    fn zoom_keeps_the_point_under_the_cursor() {
        let (zoom, offset) = zoom_at(1.0, Vec2::ZERO, 1.0, vec2(10.0, 0.0), Vec2::ZERO, 2.0);
        assert!((zoom - 2.0).abs() < 1e-5);
        assert!((offset.x + 10.0).abs() < 1e-3);
        assert!(offset.y.abs() < 1e-3);
    }

    #[test]
    fn fitted_photos_cannot_be_panned_off_the_window() {
        let offset = clamp_offset(vec2(80.0, -40.0), vec2(200.0, 100.0), vec2(400.0, 300.0));
        assert_eq!(offset, Vec2::ZERO);
        let zoomed = clamp_offset(vec2(500.0, 0.0), vec2(800.0, 400.0), vec2(400.0, 300.0));
        assert!((zoomed.x - 200.0).abs() < 1e-3);
    }
}
