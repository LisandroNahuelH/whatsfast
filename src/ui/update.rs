//! Sticky update toast: one click downloads (if needed), installs, and restarts.

use egui::{Align, CornerRadius, Frame, Layout, Margin, RichText, Stroke};

use crate::app::App;
use crate::model::Action;
use crate::theme::{self, Icon};
use crate::updates::DownloadState;

pub fn toast(app: &mut App, ui: &mut egui::Ui) {
    let Some(release) = app.update.clone() else {
        return;
    };
    let palette = app.palette;
    let busy = app.update_support.is_none()
        || matches!(app.update_download, DownloadState::Installing)
        || (app.install_when_ready
            && matches!(
                app.update_download,
                DownloadState::Downloading { .. } | DownloadState::Idle
            ));
    Frame::new()
        .fill(palette.overlay)
        .stroke(Stroke::new(1.0, palette.outline))
        .corner_radius(CornerRadius::same(theme::RADIUS))
        .inner_margin(Margin::symmetric(14, 10))
        .shadow(egui::epaint::Shadow {
            offset: [0, 4],
            blur: 16,
            spread: 0,
            color: palette.shadow,
        })
        .show(ui, |ui| {
            ui.set_max_width(420.0);
            ui.horizontal(|ui| {
                if busy {
                    theme::spinner(ui, 16.0, palette.accent);
                } else {
                    theme::icon(ui, Icon::Info, 16.0, palette.accent);
                }
                ui.add(
                    egui::Label::new(
                        RichText::new("There's a new version available")
                            .font(theme::medium(13.5))
                            .color(palette.text),
                    )
                    .selectable(false),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    match &app.update_support {
                        None => {}
                        Some(Err(_)) => {
                            ui.add(egui::Hyperlink::from_label_and_url(
                                RichText::new("Download from GitHub")
                                    .font(theme::medium(13.0))
                                    .color(palette.secondary),
                                &release.url,
                            ));
                        }
                        Some(Ok(_)) if !busy => {
                            let label = if matches!(app.update_download, DownloadState::Failed(_)) {
                                "Retry"
                            } else {
                                "Update"
                            };
                            if theme::soft_button(ui, &palette, None, label, true).clicked() {
                                app.actions.push(Action::InstallUpdate);
                            }
                        }
                        Some(Ok(_)) => {}
                    }
                });
            });
            if let DownloadState::Failed(error) = &app.update_download {
                ui.add_space(6.0);
                message(ui, error, palette.danger);
            } else if let Some(Err(reason)) = &app.update_support {
                ui.add_space(6.0);
                message(ui, reason, palette.secondary);
            }
        });
}

fn message(ui: &mut egui::Ui, text: &str, color: egui::Color32) {
    let line = super::widgets::line(
        ui,
        text,
        theme::regular(14.0),
        color,
        ui.available_width(),
        usize::MAX,
    );
    let (rect, _) = ui.allocate_exact_size(line.size(), egui::Sense::hover());
    line.paint(ui, rect.min, color);
}
