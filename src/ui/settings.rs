//! The settings page.

use std::time::Duration;

use egui::{CornerRadius, Frame, Margin, Order, Rect, Sense, pos2, vec2};

use crate::app::App;
use crate::backend::Command;
use crate::i18n::{self, Key, Language};
use crate::model::{Action, Dialog, Page, StorageStats};
use crate::privacy::{PrivacyChoice, PrivacyKind};
use crate::settings::{
    ChatWallpaper, HistoryPrefetch, ThemeChoice, WALLPAPER_SLOTS, WallpaperFamily,
};
use crate::theme::{self, Icon};

use super::wallpaper;
use super::widgets;

pub fn show(app: &mut App, ui: &mut egui::Ui) {
    request_storage_stats(app);
    super::standalone_header(app, ui);
    if theme::macos_chrome(ui.ctx()) {
        super::banner(app, ui);
    }
    let palette = app.palette;
    egui::ScrollArea::vertical()
        .id_salt("settings")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            Frame::new()
                .inner_margin(Margin::symmetric(32, 24))
                .show(ui, |ui| {
                    ui.set_max_width(640.0);
                    ui.horizontal(|ui| {
                        if theme::icon_button(
                            ui,
                            Icon::ArrowLeft,
                            20.0,
                            palette.secondary,
                            palette.text,
                            i18n::t(Key::CommonBack),
                        )
                        .clicked()
                        {
                            app.actions.push(Action::Open(Page::Chats));
                        }
                        theme::text(
                            ui,
                            i18n::t(Key::SettingsTitle),
                            theme::bold(24.0),
                            palette.text,
                        );
                    });
                    ui.add_space(18.0);

                    section(ui, app, i18n::t(Key::SettingsSectionAppearance));
                    let detail = app
                        .custom_themes
                        .detail(app.settings.custom_theme.as_deref());
                    let detail = if !detail.is_empty() {
                        detail
                    } else if app.custom_themes.follows_omarchy() {
                        i18n::t(Key::SettingsThemeOmarchyHint)
                    } else {
                        i18n::t(Key::SettingsThemeSystemHint)
                    };
                    widgets::setting_row(
                        ui,
                        &palette,
                        i18n::t(Key::SettingsThemeLabel),
                        detail,
                        |ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {
                                let selected = app
                                    .settings
                                    .custom_theme
                                    .as_deref()
                                    .map(theme::custom::label)
                                    .unwrap_or_else(|| app.settings.theme.label());
                                let response = egui::ComboBox::from_id_salt("appearance_theme")
                                    .selected_text(" ")
                                    .width(200.0_f32.min(ui.available_width()))
                                    .height(320.0)
                                    .show_ui(ui, |ui| {
                                        for choice in ThemeChoice::ALL {
                                            if theme_option(
                                                ui,
                                                &palette,
                                                choice.label(),
                                                app.settings.custom_theme.is_none()
                                                    && app.settings.theme == choice,
                                            ) {
                                                app.actions.push(Action::SetTheme(choice));
                                            }
                                        }
                                        if app.custom_themes.picker_themes().next().is_some() {
                                            ui.separator();
                                        }
                                        for custom in app.custom_themes.picker_themes() {
                                            if theme_option(
                                                ui,
                                                &palette,
                                                theme::custom::label(&custom.filename),
                                                app.settings.custom_theme.as_deref()
                                                    == Some(custom.filename.as_str()),
                                            ) {
                                                app.actions.push(Action::SetCustomTheme(
                                                    custom.filename.clone(),
                                                ));
                                            }
                                        }
                                    });
                                let rect = response.response.rect;
                                let text = widgets::line(
                                    ui,
                                    selected,
                                    theme::regular(14.0),
                                    palette.text,
                                    rect.width() - 36.0,
                                    1,
                                );
                                text.paint(
                                    ui,
                                    egui::pos2(
                                        rect.left() + 8.0,
                                        rect.center().y - text.size().y / 2.0,
                                    ),
                                    palette.text,
                                );
                                response.response.widget_info(|| {
                                    let mut info = egui::WidgetInfo::labeled(
                                        egui::WidgetType::ComboBox,
                                        ui.is_enabled(),
                                        i18n::t(Key::SettingsThemeLabel),
                                    );
                                    info.current_text_value = Some(selected.to_owned());
                                    info
                                });
                                if theme::soft_button(
                                    ui,
                                    &palette,
                                    Some(Icon::ExternalLink),
                                    i18n::t(Key::SettingsOpenThemesFolder),
                                    false,
                                )
                                .clicked()
                                {
                                    app.actions.push(Action::OpenThemesFolder);
                                }
                            });
                        },
                    );
                    widgets::setting_row(
                        ui,
                        &palette,
                        i18n::t(Key::SettingsWallpaperLabel),
                        i18n::t(Key::SettingsWallpaperHint),
                        |ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {
                                let selected = app
                                    .settings
                                    .chat_wallpaper
                                    .combo_label(app.settings.wallpaper_slot());
                                let mut preview = None;
                                let response = egui::ComboBox::from_id_salt("chat_wallpaper")
                                    .selected_text(" ")
                                    .width(200.0_f32.min(ui.available_width()))
                                    .height(500.0)
                                    .show_ui(ui, |ui| {
                                        let auto = wallpaper_option(
                                            ui,
                                            &palette,
                                            i18n::t(Key::SettingsWallpaperAuto),
                                            app.settings.chat_wallpaper == ChatWallpaper::Auto,
                                        );
                                        if auto.hovered() {
                                            preview = Some((
                                                wallpaper::family_for(&palette),
                                                app.settings.wallpaper_slot(),
                                            ));
                                        }
                                        if auto.clicked() {
                                            app.actions.push(Action::SetChatWallpaper {
                                                choice: ChatWallpaper::Auto,
                                                index: app.settings.wallpaper_slot(),
                                            });
                                        }
                                        for family in WallpaperFamily::ALL {
                                            for slot in 0..WALLPAPER_SLOTS {
                                                let label =
                                                    format!("{} {}", family.label(), slot + 1);
                                                let chosen = app.settings.chat_wallpaper
                                                    == family.as_choice()
                                                    && app.settings.wallpaper_slot() == slot;
                                                let row =
                                                    wallpaper_option(ui, &palette, &label, chosen);
                                                if row.hovered() {
                                                    preview = Some((family, slot));
                                                }
                                                if row.clicked() {
                                                    app.actions.push(Action::SetChatWallpaper {
                                                        choice: family.as_choice(),
                                                        index: slot,
                                                    });
                                                }
                                            }
                                        }
                                    });
                                let rect = response.response.rect;
                                let text = widgets::line(
                                    ui,
                                    &selected,
                                    theme::regular(14.0),
                                    palette.text,
                                    rect.width() - 36.0,
                                    1,
                                );
                                text.paint(
                                    ui,
                                    egui::pos2(
                                        rect.left() + 8.0,
                                        rect.center().y - text.size().y / 2.0,
                                    ),
                                    palette.text,
                                );
                                response.response.widget_info(|| {
                                    let mut info = egui::WidgetInfo::labeled(
                                        egui::WidgetType::ComboBox,
                                        ui.is_enabled(),
                                        i18n::t(Key::SettingsWallpaperLabel),
                                    );
                                    info.current_text_value = Some(selected.clone());
                                    info
                                });
                                show_wallpaper_preview(ui, app, preview);
                            });
                        },
                    );
                    widgets::setting_row(
                        ui,
                        &palette,
                        i18n::t(Key::SettingsZoomLabel),
                        i18n::t(Key::SettingsZoomHint),
                        |ui| {
                            if theme::icon_button(
                                ui,
                                Icon::Plus,
                                16.0,
                                palette.secondary,
                                palette.text,
                                i18n::t(Key::SettingsZoomLarger),
                            )
                            .clicked()
                            {
                                app.actions.push(Action::ZoomBy(0.1));
                            }
                            theme::text(
                                ui,
                                format!("{:.0}%", app.settings.zoom * 100.0),
                                theme::medium(13.5),
                                palette.text,
                            );
                            if theme::icon_button(
                                ui,
                                Icon::Minus,
                                16.0,
                                palette.secondary,
                                palette.text,
                                i18n::t(Key::SettingsZoomSmaller),
                            )
                            .clicked()
                            {
                                app.actions.push(Action::ZoomBy(-0.1));
                            }
                        },
                    );

                    section(ui, app, i18n::t(Key::SettingsSectionLanguage));
                    widgets::setting_row(
                        ui,
                        &palette,
                        i18n::t(Key::SettingsLanguageLabel),
                        i18n::t(Key::SettingsLanguageHint),
                        |ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {
                                let selected = app.settings.language.label();
                                let response = egui::ComboBox::from_id_salt("language")
                                    .selected_text(" ")
                                    .width(200.0_f32.min(ui.available_width()))
                                    .show_ui(ui, |ui| {
                                        for language in Language::ALL {
                                            if wallpaper_option(
                                                ui,
                                                &palette,
                                                language.label(),
                                                app.settings.language == language,
                                            )
                                            .clicked()
                                            {
                                                app.settings.language = language;
                                                i18n::set_language(language);
                                                app.actions.push(Action::SettingsChanged);
                                            }
                                        }
                                    });
                                let rect = response.response.rect;
                                let text = widgets::line(
                                    ui,
                                    selected,
                                    theme::regular(14.0),
                                    palette.text,
                                    rect.width() - 36.0,
                                    1,
                                );
                                text.paint(
                                    ui,
                                    egui::pos2(
                                        rect.left() + 8.0,
                                        rect.center().y - text.size().y / 2.0,
                                    ),
                                    palette.text,
                                );
                                response.response.widget_info(|| {
                                    let mut info = egui::WidgetInfo::labeled(
                                        egui::WidgetType::ComboBox,
                                        ui.is_enabled(),
                                        i18n::t(Key::SettingsLanguageLabel),
                                    );
                                    info.current_text_value = Some(selected.to_owned());
                                    info
                                });
                            });
                        },
                    );

                    section(ui, app, i18n::t(Key::SettingsSectionChats));
                    toggle(
                        ui,
                        app,
                        i18n::t(Key::SettingsEnterSends),
                        i18n::t(Key::SettingsEnterSendsHint),
                        |settings| &mut settings.enter_sends,
                    );
                    toggle(
                        ui,
                        app,
                        i18n::t(Key::SettingsShowSenderPictures),
                        i18n::t(Key::SettingsShowSenderPicturesHint),
                        |settings| &mut settings.show_sender_pictures,
                    );
                    toggle(
                        ui,
                        app,
                        i18n::t(Key::SettingsNamesFromContacts),
                        i18n::t(Key::SettingsNamesFromContactsHint),
                        |settings| &mut settings.names_from_contacts,
                    );
                    toggle(
                        ui,
                        app,
                        i18n::t(Key::SettingsSaveContacts),
                        i18n::t(Key::SettingsSaveContactsHint),
                        |settings| &mut settings.save_contacts_to_phone,
                    );
                    toggle(
                        ui,
                        app,
                        i18n::t(Key::SettingsForwardInOrder),
                        i18n::t(Key::SettingsForwardInOrderHint),
                        |settings| &mut settings.forward_in_order,
                    );
                    toggle(
                        ui,
                        app,
                        i18n::t(Key::SettingsShowPollButton),
                        i18n::t(Key::SettingsShowPollButtonHint),
                        |settings| &mut settings.show_poll_button,
                    );
                    toggle(
                        ui,
                        app,
                        i18n::t(Key::SettingsShowShortcutHints),
                        "",
                        |settings| &mut settings.show_shortcut_hints,
                    );

                    section(ui, app, i18n::t(Key::SettingsSectionDownloads));
                    toggle(
                        ui,
                        app,
                        i18n::t(Key::SettingsAutoDownload),
                        i18n::t(Key::SettingsAutoDownloadHint),
                        |settings| &mut settings.auto_download,
                    );
                    history_prefetch_row(ui, app, &palette);
                    storage_usage(ui, app);
                    widgets::setting_row(
                        ui,
                        &palette,
                        i18n::t(Key::SettingsDownloadedAttachments),
                        "",
                        |ui| {
                            if theme::soft_button(
                                ui,
                                &palette,
                                Some(Icon::ExternalLink),
                                i18n::t(Key::SettingsOpenFolder),
                                false,
                            )
                            .clicked()
                            {
                                open_media_folder(app);
                            }
                        },
                    );

                    section(ui, app, i18n::t(Key::SettingsSectionPrivacy));
                    let receipts_note = if app.account_receipts_off {
                        i18n::t(Key::SettingsReceiptsOffNote)
                    } else {
                        i18n::t(Key::SettingsReceiptsOnNote)
                    };
                    toggle(
                        ui,
                        app,
                        i18n::t(Key::SettingsSendReceipts),
                        receipts_note,
                        |settings| &mut settings.send_read_receipts,
                    );
                    toggle(ui, app, i18n::t(Key::SettingsSendTyping), "", |settings| {
                        &mut settings.send_typing
                    });
                    if app.account_privacy.fetch_failed {
                        widgets::rich_text(
                            ui,
                            i18n::t(Key::SettingsPrivacyLoadFailed),
                            theme::regular(12.5),
                            palette.secondary,
                        );
                        ui.add_space(8.0);
                    }
                    let privacy_on = app.is_connected() && app.account_privacy.loaded;
                    ui.add_enabled_ui(privacy_on, |ui| {
                        for kind in PrivacyKind::ALL {
                            privacy_row(ui, app, kind);
                        }
                    });

                    section(ui, app, i18n::t(Key::SettingsSectionWindow));
                    toggle(
                        ui,
                        app,
                        i18n::t(Key::SettingsKeepRunning),
                        i18n::t(Key::SettingsKeepRunningHint),
                        |settings| &mut settings.keep_running_in_background,
                    );
                    toggle(
                        ui,
                        app,
                        i18n::t(Key::SettingsNotify),
                        i18n::t(Key::SettingsNotifyHint),
                        |settings| &mut settings.notifications,
                    );
                    toggle(
                        ui,
                        app,
                        i18n::t(Key::SettingsHideSidebar),
                        i18n::t(Key::SettingsHideSidebarHint),
                        |settings| &mut settings.hide_sidebar_fully,
                    );

                    widgets::setting_row(
                        ui,
                        &palette,
                        i18n::t(Key::SettingsGiphyKey),
                        if crate::settings::BUILT_IN_GIPHY_KEY.is_some() {
                            i18n::t(Key::SettingsGiphyKeyHintBuiltIn)
                        } else {
                            i18n::t(Key::SettingsGiphyKeyHintRequired)
                        },
                        |ui| {
                            let response = ui.add(
                                egui::TextEdit::singleline(&mut app.settings.giphy_key)
                                    .font(theme::regular(13.0))
                                    .text_color(palette.text)
                                    .desired_width(220.0),
                            );
                            if response.changed() {
                                app.actions.push(Action::SettingsChanged);
                            }
                        },
                    );

                    section(ui, app, i18n::t(Key::SettingsSectionAccount));
                    let name = app.me_name.clone().unwrap_or_default();
                    let me = app.me.clone().unwrap_or_default();
                    let phone = crate::model::phone_of(&me)
                        .map(crate::util::phone)
                        .unwrap_or_else(|| me.clone());
                    let description = match &app.me_about {
                        Some(about) => format!("{phone} · {about}"),
                        None => phone,
                    };
                    ui.horizontal(|ui| {
                        let picture = app.avatar_full(&me).or_else(|| app.avatar(&me));
                        widgets::avatar(ui, &palette, &name, &me, 56.0, picture.as_deref());
                    });
                    ui.add_space(6.0);
                    widgets::setting_row(
                        ui,
                        &palette,
                        if name.is_empty() {
                            i18n::t(Key::SettingsLinkedDevice)
                        } else {
                            &name
                        },
                        &description,
                        |ui| {
                            if theme::soft_button(
                                ui,
                                &palette,
                                Some(Icon::LogOut),
                                i18n::t(Key::SettingsUnlink),
                                false,
                            )
                            .clicked()
                            {
                                app.actions.push(Action::ShowDialog(Dialog::ConfirmUnlink));
                            }
                        },
                    );

                    section(ui, app, i18n::t(Key::SettingsSectionFiles));
                    let archive = app.dirs.archive_db();
                    widgets::setting_row(
                        ui,
                        &palette,
                        i18n::t(Key::SettingsMessageArchive),
                        &archive.display().to_string(),
                        |ui| {
                            if theme::soft_button(
                                ui,
                                &palette,
                                Some(Icon::ExternalLink),
                                i18n::t(Key::SettingsOpenFolder),
                                false,
                            )
                            .clicked()
                            {
                                app.actions.push(Action::OpenFile(app.dirs.state.clone()));
                            }
                        },
                    );
                    toggle(
                        ui,
                        app,
                        i18n::t(Key::SettingsAskWhereToSave),
                        i18n::t(Key::SettingsAskWhereToSaveHint),
                        |settings| &mut settings.ask_where_to_save,
                    );
                    let log = app.dirs.log_file();
                    widgets::setting_row(
                        ui,
                        &palette,
                        i18n::t(Key::SettingsLogOfRun),
                        &log.display().to_string(),
                        |ui| {
                            if theme::soft_button(
                                ui,
                                &palette,
                                Some(Icon::FileText),
                                i18n::t(Key::CommonOpen),
                                false,
                            )
                            .clicked()
                            {
                                app.actions.push(Action::OpenFile(log.clone()));
                            }
                        },
                    );

                    section(ui, app, i18n::t(Key::SettingsSectionAbout));
                    widgets::setting_row(
                        ui,
                        &palette,
                        &format!("WhatsFast {}", env!("CARGO_PKG_VERSION")),
                        i18n::t(Key::SettingsAboutLine),
                        |ui| {
                            if theme::soft_button(
                                ui,
                                &palette,
                                Some(Icon::Info),
                                i18n::t(Key::SettingsSectionAbout),
                                false,
                            )
                            .clicked()
                            {
                                app.actions.push(Action::ShowDialog(Dialog::About));
                            }
                            if theme::soft_button(
                                ui,
                                &palette,
                                Some(Icon::Keyboard),
                                i18n::t(Key::CommonShortcuts),
                                false,
                            )
                            .clicked()
                            {
                                app.actions.push(Action::ShowDialog(Dialog::Shortcuts));
                            }
                        },
                    );
                    toggle(
                        ui,
                        app,
                        i18n::t(Key::SettingsCheckUpdates),
                        i18n::t(Key::SettingsCheckUpdatesHint),
                        |settings| &mut settings.check_for_updates,
                    );
                    toggle(
                        ui,
                        app,
                        i18n::t(Key::SettingsDownloadUpdates),
                        i18n::t(Key::SettingsDownloadUpdatesHint),
                        |settings| &mut settings.download_updates_automatically,
                    );
                });
        });
}

const STORAGE_STATS_TTL: Duration = Duration::from_secs(60);

fn request_storage_stats(app: &mut App) {
    let stale = app
        .storage_stats_at
        .is_none_or(|at| at.elapsed() >= STORAGE_STATS_TTL);
    if stale && !app.storage_stats_asked {
        app.backend.send(Command::StorageStats);
        app.storage_stats_asked = true;
    }
}

pub(crate) fn open_media_folder(app: &mut App) {
    let media = app.dirs.media_cache_dir();
    let _ = std::fs::create_dir_all(&media);
    app.actions.push(Action::OpenFile(media));
}

fn history_prefetch_row(ui: &mut egui::Ui, app: &mut App, palette: &theme::Palette) {
    widgets::setting_row(
        ui,
        palette,
        i18n::t(Key::SettingsHistoryLabel),
        i18n::t(Key::SettingsHistoryHint),
        |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {
                let selected = app.settings.history_prefetch.label();
                let response = egui::ComboBox::from_id_salt("history_prefetch")
                    .selected_text(" ")
                    .width(200.0_f32.min(ui.available_width()))
                    .show_ui(ui, |ui| {
                        for choice in HistoryPrefetch::ALL {
                            let row = wallpaper_option(
                                ui,
                                palette,
                                choice.label(),
                                app.settings.history_prefetch == choice,
                            );
                            if row.contains_pointer() {
                                show_prefetch_hint(app, &row, choice.hint());
                            }
                            if row.clicked() {
                                app.actions.push(Action::SetHistoryPrefetch(choice));
                            }
                        }
                    });
                let rect = response.response.rect;
                let text = widgets::line(
                    ui,
                    selected,
                    theme::regular(14.0),
                    palette.text,
                    rect.width() - 36.0,
                    1,
                );
                text.paint(
                    ui,
                    egui::pos2(rect.left() + 8.0, rect.center().y - text.size().y / 2.0),
                    palette.text,
                );
                response.response.widget_info(|| {
                    let mut info = egui::WidgetInfo::labeled(
                        egui::WidgetType::ComboBox,
                        ui.is_enabled(),
                        i18n::t(Key::SettingsHistoryLabel),
                    );
                    info.current_text_value = Some(selected.to_owned());
                    info
                });
            });
        },
    );
}

fn storage_usage(ui: &mut egui::Ui, app: &App) {
    let palette = app.palette;
    let stats = app.storage_stats;
    let total = stats.bytes_total();
    theme::text(
        ui,
        crate::util::bytes(total),
        theme::semibold(16.0),
        palette.text,
    );
    let hint = widgets::line(
        ui,
        i18n::t(Key::SettingsStorageHint),
        theme::regular(12.5),
        palette.secondary,
        ui.available_width(),
        usize::MAX,
    );
    let (rect, _) = ui.allocate_exact_size(hint.size(), Sense::hover());
    if ui.is_rect_visible(rect) {
        hint.paint(ui, rect.min, palette.secondary);
    }
    ui.add_space(4.0);
    widgets::rich_text(
        ui,
        &i18n::count(
            Key::SettingsStorageMessagesOne,
            Key::SettingsStorageMessagesMany,
            stats.messages as usize,
        ),
        theme::regular(12.5),
        palette.secondary,
    );
    ui.add_space(8.0);
    storage_bar_row(
        ui,
        &palette,
        i18n::t(Key::SettingsStorageImages),
        stats.images,
        stats.image_bytes,
        total,
    );
    storage_bar_row(
        ui,
        &palette,
        i18n::t(Key::SettingsStorageVideos),
        stats.videos,
        stats.video_bytes,
        total,
    );
    storage_bar_row(
        ui,
        &palette,
        i18n::t(Key::SettingsStorageStickersGifs),
        stats.stickers_gifs,
        stats.sticker_gif_bytes,
        total,
    );
    if stats.other_bytes > 0 {
        storage_bar_row(
            ui,
            &palette,
            i18n::t(Key::SettingsStorageOther),
            stats.other,
            stats.other_bytes,
            total,
        );
    }
}

fn storage_bar_row(
    ui: &mut egui::Ui,
    palette: &theme::Palette,
    label: &str,
    count: u64,
    bytes: u64,
    total: u64,
) {
    ui.horizontal(|ui| {
        theme::text(ui, label, theme::medium(14.0), palette.text);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            theme::text(
                ui,
                format!("{count} · {}", crate::util::bytes(bytes)),
                theme::regular(12.5),
                palette.secondary,
            );
        });
    });
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 6.0), Sense::hover());
    if ui.is_rect_visible(rect) {
        ui.painter()
            .rect_filled(rect, rect.height() / 2.0, palette.outline);
        let width = StorageStats::bar_width(bytes, total, rect.width());
        if width > 0.5 {
            ui.painter().rect_filled(
                Rect::from_min_size(rect.min, vec2(width, rect.height())),
                rect.height() / 2.0,
                palette.accent,
            );
        }
    }
    ui.add_space(8.0);
}

fn section(ui: &mut egui::Ui, app: &App, label: &str) {
    let palette = app.palette;
    ui.add_space(10.0);
    Frame::new()
        .fill(palette.panel)
        .corner_radius(CornerRadius::same(theme::RADIUS))
        .inner_margin(Margin::symmetric(14, 8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            theme::text(ui, label, theme::semibold(12.5), palette.accent);
        });
    ui.add_space(8.0);
}

fn toggle(
    ui: &mut egui::Ui,
    app: &mut App,
    label: &str,
    description: &str,
    field: impl Fn(&mut crate::settings::Settings) -> &mut bool,
) {
    let palette = app.palette;
    let mut value = *field(&mut app.settings);
    let mut changed = false;
    widgets::setting_row(ui, &palette, label, description, |ui| {
        changed = widgets::switch(ui, &palette, &mut value).changed();
    });
    if changed {
        *field(&mut app.settings) = value;
        app.actions.push(Action::SettingsChanged);
    }
}

fn privacy_row(ui: &mut egui::Ui, app: &mut App, kind: PrivacyKind) {
    let palette = app.palette;
    let current = app.account_privacy.get(kind);
    let selected = current.map(PrivacyChoice::label).unwrap_or("—");
    let pending = app.account_privacy.pending(kind);
    widgets::setting_row(ui, &palette, kind.label(), kind.hint(), |ui| {
        ui.add_enabled_ui(!pending, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {
                let salt = format!("privacy_{}", kind.wire_name());
                let response = egui::ComboBox::from_id_salt(salt)
                    .selected_text(" ")
                    .width(220.0_f32.min(ui.available_width()))
                    .show_ui(ui, |ui| {
                        for choice in kind.choices() {
                            if wallpaper_option(
                                ui,
                                &palette,
                                choice.label(),
                                current == Some(*choice),
                            )
                            .clicked()
                            {
                                app.actions.push(Action::SetAccountPrivacy {
                                    kind,
                                    choice: *choice,
                                });
                            }
                        }
                    });
                let rect = response.response.rect;
                let text = widgets::line(
                    ui,
                    selected,
                    theme::regular(14.0),
                    palette.text,
                    rect.width() - 36.0,
                    1,
                );
                text.paint(
                    ui,
                    egui::pos2(rect.left() + 8.0, rect.center().y - text.size().y / 2.0),
                    palette.text,
                );
                response.response.widget_info(|| {
                    let mut info = egui::WidgetInfo::labeled(
                        egui::WidgetType::ComboBox,
                        ui.is_enabled(),
                        kind.label(),
                    );
                    info.current_text_value = Some(selected.to_owned());
                    info
                });
            });
        });
    });
}

/// Theme filenames can contain emoji, so paint them through the shared line renderer.
fn theme_option(ui: &mut egui::Ui, palette: &theme::Palette, text: &str, selected: bool) -> bool {
    let response = ui.add(
        egui::Button::selectable(selected, " ").min_size(egui::vec2(ui.available_width(), 28.0)),
    );
    let rect = response.rect;
    let line = widgets::line(
        ui,
        text,
        theme::regular(14.0),
        palette.text,
        rect.width() - 16.0,
        1,
    );
    if ui.is_rect_visible(rect) {
        line.paint(
            ui,
            egui::pos2(rect.left() + 8.0, rect.center().y - line.size().y / 2.0),
            palette.text,
        );
    }
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            text,
        )
    });
    response.clicked()
}

fn wallpaper_option(
    ui: &mut egui::Ui,
    palette: &theme::Palette,
    text: &str,
    selected: bool,
) -> egui::Response {
    let response = ui.add(
        egui::Button::selectable(selected, " ").min_size(egui::vec2(ui.available_width(), 28.0)),
    );
    let rect = response.rect;
    let line = widgets::line(
        ui,
        text,
        theme::regular(14.0),
        palette.text,
        rect.width() - 16.0,
        1,
    );
    if ui.is_rect_visible(rect) {
        line.paint(
            ui,
            egui::pos2(rect.left() + 8.0, rect.center().y - line.size().y / 2.0),
            palette.text,
        );
    }
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            text,
        )
    });
    response
}

fn settings_hover_slot(ui: &egui::Ui, app: &App, height: f32) -> Option<(f32, egui::Pos2)> {
    let window = ui.ctx().content_rect();
    let sidebar = if app.sidebar_visible {
        app.settings.sidebar_width
    } else if app.compact_sidebar() {
        56.0
    } else {
        0.0
    };
    let column = (640.0_f32 + 64.0).min((window.width() - sidebar).max(0.0));
    let col_right = window.left() + sidebar + column;
    let gap = (window.right() - col_right - 16.0).max(0.0);
    let width = if gap < 160.0 {
        gap
    } else {
        gap.min(window.width() * 0.32).clamp(160.0, 420.0)
    };
    if width < 80.0 {
        return None;
    }
    let top = (window.top() + 72.0).min(window.bottom() - height - 16.0);
    Some((width, pos2(col_right + 8.0, top.max(window.top() + 8.0))))
}

fn show_wallpaper_preview(ui: &mut egui::Ui, app: &App, preview: Option<(WallpaperFamily, u8)>) {
    let Some((family, slot)) = preview else {
        return;
    };
    let height_guess = 160.0 * 9.0 / 16.0;
    let Some((width, origin)) = settings_hover_slot(ui, app, height_guess) else {
        return;
    };
    let height = width * 9.0 / 16.0;
    egui::Area::new(egui::Id::new("wallpaper-preview"))
        .order(Order::Foreground)
        .fixed_pos(origin)
        .interactable(false)
        .show(ui.ctx(), |ui| {
            Frame::new()
                .fill(app.palette.panel)
                .stroke(egui::Stroke::new(1.0, app.palette.dim))
                .corner_radius(CornerRadius::same(8))
                .inner_margin(Margin::same(6))
                .show(ui, |ui| {
                    let size = vec2(width, height);
                    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
                    wallpaper::paint_in(ui, family, slot, rect);
                });
        });
}

fn show_prefetch_hint(app: &App, row: &egui::Response, hint: &str) {
    let mut rect = row.interact_rect;
    if let Some(to_global) = row.ctx.layer_transform_to_global(row.layer_id) {
        rect = to_global * rect;
    }
    let window = row.ctx.content_rect();
    let gap_right = (window.right() - rect.right() - 8.0).max(0.0);
    let gap_left = (rect.left() - window.left() - 8.0).max(0.0);
    let (align, width) = if gap_right >= 80.0 {
        (egui::RectAlign::RIGHT_START, gap_right.min(320.0))
    } else if gap_left >= 80.0 {
        (egui::RectAlign::LEFT_START, gap_left.min(320.0))
    } else {
        return;
    };
    let frame = Frame::new()
        .fill(app.palette.panel)
        .stroke(egui::Stroke::new(1.0, app.palette.dim))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(Margin::symmetric(10, 8));
    egui::Popup::from_response(row)
        .id(egui::Id::new("prefetch-hint"))
        .kind(egui::PopupKind::Tooltip)
        .align(align)
        .align_alternatives(&[])
        .gap(8.0)
        .close_behavior(egui::PopupCloseBehavior::IgnoreClicks)
        .interactable(false)
        .width(width)
        .frame(frame)
        .show(|ui| {
            ui.set_max_width(width);
            let width = ui.available_width();
            let line = widgets::line(ui, hint, theme::regular(13.0), app.palette.text, width, 4);
            let (rect, _) = ui.allocate_exact_size(line.size(), Sense::hover());
            line.paint(ui, rect.min, app.palette.text);
        });
}
