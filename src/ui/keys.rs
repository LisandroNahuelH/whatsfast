//! Keyboard shortcuts.

use egui::{Key, Modifiers};

use crate::app::App;
use crate::i18n::{self, Key as I18nKey};
use crate::model::{Action, Dialog, Page, RightPane};

pub fn handle(app: &mut App, ctx: &egui::Context) {
    let mut actions = Vec::new();
    ctx.input_mut(|input| {
        let mut key = |modifiers: Modifiers, key: Key, action: Action| {
            if input.consume_key(modifiers, key) {
                actions.push(action);
            }
        };
        key(Modifiers::COMMAND, Key::F, Action::FocusSearch);
        key(Modifiers::COMMAND, Key::K, Action::FocusSearch);
        if app.page == Page::Chats
            && app.open_chat.is_some()
            && app.dialog.is_none()
            && app.image_viewer.is_none()
            && app.recording.is_none()
        {
            key(Modifiers::COMMAND, Key::L, Action::FocusComposer);
            key(
                Modifiers::COMMAND,
                Key::G,
                Action::OpenRightPane(RightPane::Search),
            );
        }
        key(Modifiers::COMMAND, Key::B, Action::ToggleSidebar);
        key(Modifiers::COMMAND, Key::Comma, Action::Open(Page::Settings));
        key(Modifiers::COMMAND, Key::Q, Action::Quit);
        key(Modifiers::COMMAND, Key::W, Action::CloseWindow);
        key(Modifiers::NONE, Key::F11, Action::ToggleFullscreen);
        key(
            Modifiers::COMMAND,
            Key::Slash,
            Action::ShowDialog(Dialog::Shortcuts),
        );
        key(Modifiers::COMMAND, Key::Plus, Action::ZoomBy(0.1));
        key(Modifiers::COMMAND, Key::Equals, Action::ZoomBy(0.1));
        key(Modifiers::COMMAND, Key::Minus, Action::ZoomBy(-0.1));
        key(Modifiers::COMMAND, Key::Num0, Action::ResetZoom);
        key(Modifiers::COMMAND, Key::End, Action::ScrollToBottom);
        if app.image_viewer.is_some() && app.dialog.is_none() && app.picker.is_none() {
            key(Modifiers::NONE, Key::ArrowLeft, Action::StepImage(-1));
            key(Modifiers::NONE, Key::ArrowRight, Action::StepImage(1));
        }
    });
    // Escape cancels the topmost state. Menus handle Escape themselves.
    let menu_open = egui::Popup::is_any_open(ctx);
    let search_focused = ctx.memory(|memory| memory.has_focus(egui::Id::new("chat-search")));
    let escape =
        !menu_open && ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Escape));
    if escape {
        if app.dialog.is_some() {
            actions.push(Action::CloseDialog);
        } else if app.picker.is_some() || app.reaction_target.is_some() {
            actions.push(Action::ClosePicker);
        } else if app.image_viewer.is_some() {
            actions.push(Action::CloseImageViewer);
        } else if app.selecting.is_some() {
            actions.push(Action::ClearSelection);
        } else if app.show_scheduled {
            actions.push(Action::ToggleScheduled);
        } else if app.show_starred {
            actions.push(Action::ToggleStarred);
        } else if app.show_pinned {
            actions.push(Action::TogglePinned);
        } else if app.recording.is_some() {
            actions.push(Action::CancelRecording);
        } else if app.emoji_start.is_some() {
            actions.push(Action::CloseEmojiSuggestions);
        } else if app.mention_start.is_some() {
            actions.push(Action::CloseMentions);
        } else if !app.pending.is_empty() {
            actions.push(Action::ClearPending);
        } else if app.editing.is_some() {
            actions.push(Action::CancelEdit);
        } else if app.reply_to.is_some() {
            actions.push(Action::CancelReply);
        } else if app.chat_search_calendar {
            app.chat_search_calendar = false;
        } else if app.right_pane.is_some() {
            actions.push(Action::CloseRightPane);
        } else if app.page == Page::Settings {
            actions.push(Action::Open(Page::Chats));
        } else if search_focused || !app.search.is_empty() {
            if !app.search.is_empty() {
                actions.push(Action::Search(String::new()));
            }
            if app.open_chat.is_some() {
                actions.push(Action::FocusComposer);
            }
        } else if app.open_chat.is_some() {
            actions.push(Action::CloseChat);
        }
    }
    // Enter sends a recording because the text field is hidden.
    if app.recording.is_some()
        && ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Enter))
    {
        actions.push(Action::SendRecording);
    }
    // Alt+Up/Down switches chats without leaving the composer.
    let step = ctx.input_mut(|input| {
        if input.consume_key(Modifiers::ALT, Key::ArrowDown) {
            1
        } else if input.consume_key(Modifiers::ALT, Key::ArrowUp) {
            -1
        } else {
            0
        }
    });
    if step != 0 {
        let visible = app.visible_chats();
        if !visible.is_empty() {
            let current = app
                .open_chat
                .as_ref()
                .and_then(|open| visible.iter().position(|chat| chat.id == *open));
            let next = match current {
                Some(index) => (index as i64 + step).rem_euclid(visible.len() as i64) as usize,
                None => 0,
            };
            let next = visible[next].id.clone();
            app.scroll_chat_into_view = Some(next.clone());
            actions.push(Action::OpenChat(next));
        }
    }
    app.actions.extend(actions);
}

/// Shortcuts shown in the help dialog.
pub fn shortcuts() -> [(&'static str, &'static str); 17] {
    [
        ("Ctrl+F / Ctrl+K", i18n::t(I18nKey::ShortcutSearchChats)),
        ("Ctrl+G", i18n::t(I18nKey::ShortcutSearchMessages)),
        ("Ctrl+L", i18n::t(I18nKey::ShortcutFocusComposer)),
        ("Alt+↑ / Alt+↓", i18n::t(I18nKey::ShortcutPreviousNextChat)),
        ("Enter", i18n::t(I18nKey::ShortcutSend)),
        ("Escape", i18n::t(I18nKey::ShortcutDismiss)),
        (
            i18n::t(I18nKey::ShortcutPhotoViewer),
            i18n::t(I18nKey::ShortcutPhotoViewerKeys),
        ),
        ("Ctrl+V", i18n::t(I18nKey::ShortcutPaste)),
        ("Ctrl+B", i18n::t(I18nKey::ShortcutChatList)),
        ("Ctrl+End", i18n::t(I18nKey::ShortcutNewest)),
        ("Ctrl+,", i18n::t(I18nKey::SettingsTitle)),
        ("Ctrl++ / Ctrl+-", i18n::t(I18nKey::ShortcutZoom)),
        ("Ctrl+0", i18n::t(I18nKey::ShortcutResetZoom)),
        ("Ctrl+/", i18n::t(I18nKey::ShortcutThisList)),
        ("F11", i18n::t(I18nKey::ShortcutFullscreen)),
        ("Ctrl+W", i18n::t(I18nKey::ShortcutCloseWindow)),
        ("Ctrl+Q", i18n::t(I18nKey::CommonQuit)),
    ]
}

/// Uses Command and Option labels on macOS.
pub fn label(keys: &str) -> String {
    if cfg!(target_os = "macos") {
        keys.replace("Ctrl", "⌘").replace("Alt", "⌥")
    } else {
        keys.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn escape(app: &mut App, ctx: &egui::Context) {
        let mut output = ctx.run_ui(
            egui::RawInput {
                events: vec![egui::Event::Key {
                    key: Key::Escape,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: Modifiers::NONE,
                }],
                ..Default::default()
            },
            |ui| handle(app, ui.ctx()),
        );
        output.textures_delta.clear();
    }

    #[test]
    fn focus_input_shortcut_only_targets_an_available_composer() {
        let root = tempfile::tempdir().unwrap();
        let mut app = App::headless(
            crate::paths::AppDirs::under(root.path()),
            crate::settings::Settings::default(),
        )
        .0;
        let ctx = egui::Context::default();
        for (page, chat, dialog, expected) in [
            (Page::Chats, Some("fixture"), None, true),
            (Page::Chats, None, None, false),
            (Page::Settings, Some("fixture"), None, false),
            (Page::Chats, Some("fixture"), Some(Dialog::Shortcuts), false),
        ] {
            app.page = page;
            app.open_chat = chat.map(str::to_owned);
            app.dialog = dialog;
            app.actions.clear();
            let mut output = ctx.run_ui(
                egui::RawInput {
                    events: vec![egui::Event::Key {
                        key: Key::L,
                        physical_key: None,
                        pressed: true,
                        repeat: false,
                        modifiers: Modifiers::COMMAND,
                    }],
                    ..Default::default()
                },
                |ui| handle(&mut app, ui.ctx()),
            );
            output.textures_delta.clear();
            assert_eq!(
                matches!(app.actions.as_slice(), [Action::FocusComposer]),
                expected
            );
        }
    }

    #[test]
    fn escape_returns_from_search_and_reply_before_closing_the_chat() {
        let root = tempfile::tempdir().unwrap();
        let mut app = App::headless(
            crate::paths::AppDirs::under(root.path()),
            crate::settings::Settings::default(),
        )
        .0;
        app.page = Page::Chats;
        app.open_chat = Some("fixture".into());
        let ctx = egui::Context::default();
        app.reply_to = Some("reply".into());
        escape(&mut app, &ctx);
        assert!(matches!(app.actions.as_slice(), [Action::CancelReply]));
        app.actions.clear();
        app.reply_to = None;
        app.search = "Ada".into();
        escape(&mut app, &ctx);
        assert!(
            matches!(app.actions.as_slice(), [Action::Search(text), Action::FocusComposer] if text.is_empty())
        );
        app.actions.clear();
        app.search.clear();
        escape(&mut app, &ctx);
        assert!(matches!(app.actions.as_slice(), [Action::CloseChat]));
    }

    #[test]
    fn escape_closes_the_image_viewer_before_closing_the_chat() {
        let root = tempfile::tempdir().unwrap();
        let mut app = App::headless(
            crate::paths::AppDirs::under(root.path()),
            crate::settings::Settings::default(),
        )
        .0;
        app.page = Page::Chats;
        app.open_chat = Some("fixture".into());
        app.image_viewer = Some(crate::ui::viewer::ImageViewer::open(
            "fixture".into(),
            "photo".into(),
        ));
        let ctx = egui::Context::default();
        escape(&mut app, &ctx);
        assert!(matches!(app.actions.as_slice(), [Action::CloseImageViewer]));
    }

    #[test]
    fn ctrl_g_opens_chat_search_when_a_chat_is_open() {
        let root = tempfile::tempdir().unwrap();
        let mut app = App::headless(
            crate::paths::AppDirs::under(root.path()),
            crate::settings::Settings::default(),
        )
        .0;
        let ctx = egui::Context::default();
        app.page = Page::Chats;
        app.open_chat = Some("fixture".into());
        let mut output = ctx.run_ui(
            egui::RawInput {
                events: vec![egui::Event::Key {
                    key: Key::G,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: Modifiers::COMMAND,
                }],
                ..Default::default()
            },
            |ui| handle(&mut app, ui.ctx()),
        );
        output.textures_delta.clear();
        assert!(matches!(
            app.actions.as_slice(),
            [Action::OpenRightPane(RightPane::Search)]
        ));
        app.actions.clear();
        app.open_chat = None;
        let mut output = ctx.run_ui(
            egui::RawInput {
                events: vec![egui::Event::Key {
                    key: Key::G,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: Modifiers::COMMAND,
                }],
                ..Default::default()
            },
            |ui| handle(&mut app, ui.ctx()),
        );
        output.textures_delta.clear();
        assert!(app.actions.is_empty());
    }

    #[test]
    fn escape_closes_the_search_calendar_then_the_pane() {
        let root = tempfile::tempdir().unwrap();
        let mut app = App::headless(
            crate::paths::AppDirs::under(root.path()),
            crate::settings::Settings::default(),
        )
        .0;
        app.page = Page::Chats;
        app.open_chat = Some("fixture".into());
        app.right_pane = Some(RightPane::Search);
        app.chat_search_calendar = true;
        let ctx = egui::Context::default();
        escape(&mut app, &ctx);
        assert!(!app.chat_search_calendar);
        assert!(app.actions.is_empty());
        escape(&mut app, &ctx);
        assert!(matches!(app.actions.as_slice(), [Action::CloseRightPane]));
    }

    #[test]
    fn f11_toggles_fullscreen() {
        let root = tempfile::tempdir().unwrap();
        let mut app = App::headless(
            crate::paths::AppDirs::under(root.path()),
            crate::settings::Settings::default(),
        )
        .0;
        let ctx = egui::Context::default();
        let mut output = ctx.run_ui(
            egui::RawInput {
                events: vec![egui::Event::Key {
                    key: Key::F11,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: Modifiers::NONE,
                }],
                ..Default::default()
            },
            |ui| handle(&mut app, ui.ctx()),
        );
        output.textures_delta.clear();
        assert!(matches!(app.actions.as_slice(), [Action::ToggleFullscreen]));
    }
}
