//! A repeatable tour driven through the real pointer and keyboard handlers.

mod media;
mod session;

use crate::{
    app::App,
    model::{Content, Page},
    settings::ThemeChoice,
};
use egui::{Event, Key, Modifiers, PointerButton, Pos2, Rect, pos2, vec2};
use serde::Serialize;
use std::{
    collections::HashMap,
    path::PathBuf,
    time::{Duration, Instant},
};

const PHOTO_CAPTION: &str = "A little poster for launch day ⚡";
/// Length of the input-driven tour, excluding its optional start delay.
pub const DURATION: Duration = Duration::from_secs(41);
/// Sets up the opening shot. Call only on an app populated with demo data.
pub fn prepare(app: &mut App) {
    assert!(app.backend.is_offline(), "a tour requires an offline app");
    app.settings.theme = ThemeChoice::Dark;
    app.settings.keep_running_in_background = false;
    app.page = Page::Chats;
    app.dialog = None;
    app.picker = None;
    app.search.clear();
    app.search_hits.clear();
    app.composer.clear();
    app.drafts.clear();
    app.reply_to = None;
    app.typing.clear();
    app.actions.clear();
    app.open_chat = Some(super::SAMPLES[0].id.to_owned());
    app.scroll_to_bottom = true;
    app.scroll_anchor = None;
    app.focus_composer = false;
    app.sidebar_visible = true;
    app.show_archived = false;
    app.chat_list = crate::model::ChatListId::All;
    app.backend.record_demo_commands();
    media::populate(app).expect("bundled demo media");
    if let Some(row) = app
        .conversations
        .get_mut(super::SAMPLES[0].id)
        .and_then(|chat| chat.message_mut("ada-sticker"))
        && let Content::Sticker { media, animated } = &mut row.content
    {
        media.path = app.stickers_saved.first().cloned();
        *animated = false;
    }
    super::apply_flags(app, Some("voice"));
    // Show fully loaded media instead of the deliberately blurry download
    // previews used by the general screenshot fixtures.
    let (photo, _) = super::sample_files(app);
    for (chat, id, caption) in [
        (super::SAMPLES[0].id, "ada-photo", PHOTO_CAPTION),
        (
            super::SAMPLES[1].id,
            "group-photo",
            "Tonight's meetup, doors at 18:30",
        ),
    ] {
        if let Some(row) = app
            .conversations
            .get_mut(chat)
            .and_then(|chat| chat.message_mut(id))
            && let Content::Image {
                media,
                caption: text,
            } = &mut row.content
        {
            media.path = Some(photo.clone());
            media.width = Some(900);
            media.height = Some(1200);
            *text = Some(caption.to_owned());
        }
    }
    if let Some(quote) = app
        .conversations
        .get_mut(super::SAMPLES[0].id)
        .and_then(|chat| chat.message_mut("ada-reply"))
        .and_then(|row| row.quoted.as_mut())
    {
        quote.summary = "Voice message (0:06)".into();
    }
    // Keep the launch footage focused on this app.
    if let Some(row) = app
        .conversations
        .get_mut(super::SAMPLES[0].id)
        .and_then(|chat| chat.message_mut("ada-link"))
    {
        row.content = Content::text("The desktop app is ready! https://whatsfast.rocks");
        row.thumbnail = None;
        let summary = row.summary();
        if let Some(last) = app.chats.first_mut().and_then(|chat| chat.last.as_mut()) {
            last.summary = summary;
        }
    }
}

#[derive(Clone, Copy)]
enum Target {
    Label(&'static str),
    Widget(&'static str),
    Bubble(&'static str),
    Picker,
    Gif,
    Sticker,
}

enum Gesture {
    Key(Key, Modifiers, &'static str),
    Text(char),
    Move(Target),
    Click(PointerButton),
}

struct Cue {
    at: f32,
    gesture: Gesture,
}

fn command() -> Modifiers {
    Modifiers {
        command: true,
        ctrl: !cfg!(target_os = "macos"),
        mac_cmd: cfg!(target_os = "macos"),
        ..Modifiers::NONE
    }
}

fn script() -> Vec<Cue> {
    use Gesture::*;
    use Target::*;
    let mut cues = Vec::new();
    let mut add = |at, gesture| cues.push(Cue { at, gesture });
    let left = PointerButton::Primary;
    add(0.0, Key(egui::Key::K, command(), "Ctrl + K · Search chats"));
    add(0.8, Move(Label("Rust Berlin")));
    add(1.15, Click(left));
    add(
        1.7,
        Key(
            egui::Key::Escape,
            Modifiers::NONE,
            "Esc · Return to the chat",
        ),
    );
    add(
        2.1,
        Key(
            egui::Key::ArrowUp,
            Modifiers::ALT,
            "Alt + ↑ · Previous chat",
        ),
    );
    add(
        2.6,
        Key(egui::Key::ArrowDown, Modifiers::ALT, "Alt + ↓ · Next chat"),
    );
    add(
        3.1,
        Key(
            egui::Key::ArrowUp,
            Modifiers::ALT,
            "Alt + ↑ · Previous chat",
        ),
    );
    add(
        4.8,
        Key(egui::Key::End, command(), "Ctrl + End · Latest messages"),
    );
    add(5.4, Move(Bubble("ada-voice")));
    add(5.8, Click(PointerButton::Secondary));
    add(6.25, Move(Label("Reply")));
    add(6.7, Click(left));
    add(
        8.1,
        Key(egui::Key::Enter, Modifiers::NONE, "Enter · Complete emoji"),
    );
    add(
        8.7,
        Key(egui::Key::Enter, Modifiers::NONE, "Enter · Send reply"),
    );
    add(9.4, Move(Picker));
    add(9.8, Click(left));
    add(10.5, Move(Label("GIF")));
    add(10.9, Click(left));
    add(11.5, Move(Widget("gif-search")));
    add(11.9, Click(left));
    add(
        12.6,
        Key(egui::Key::Enter, Modifiers::NONE, "Enter · Search GIFs"),
    );
    add(13.3, Move(Gif));
    add(
        13.8,
        Key(egui::Key::Escape, Modifiers::NONE, "Esc · Close GIF search"),
    );
    add(15.9, Move(Picker));
    add(16.3, Click(left));
    add(17.0, Move(Label("Stickers")));
    add(17.4, Click(left));
    add(18.0, Move(Sticker));
    add(18.5, Click(left));
    add(
        20.2,
        Key(egui::Key::ArrowDown, Modifiers::ALT, "Alt + ↓ · Next chat"),
    );
    add(
        21.6,
        Key(
            egui::Key::Enter,
            Modifiers::NONE,
            "Enter · Complete mention",
        ),
    );
    add(
        23.0,
        Key(egui::Key::Enter, Modifiers::NONE, "Enter · Send message"),
    );
    add(
        24.0,
        Key(egui::Key::B, command(), "Ctrl + B · Hide chat list"),
    );
    add(
        25.0,
        Key(egui::Key::B, command(), "Ctrl + B · Show chat list"),
    );
    add(26.0, Move(Label("Rust Berlin")));
    add(26.5, Click(left));
    add(
        28.0,
        Key(egui::Key::Escape, Modifiers::NONE, "Esc · Close group info"),
    );
    add(
        28.6,
        Key(egui::Key::Slash, command(), "Ctrl + / · Keyboard shortcuts"),
    );
    add(
        31.8,
        Key(egui::Key::Escape, Modifiers::NONE, "Esc · Close shortcuts"),
    );
    add(
        32.5,
        Key(egui::Key::Comma, command(), "Ctrl + , · Settings"),
    );
    add(33.0, Move(Label("Dark")));
    add(33.35, Click(left));
    add(33.5, Move(Label("Light")));
    add(33.85, Click(left));
    add(
        34.5,
        Key(egui::Key::Escape, Modifiers::NONE, "Esc · Back to chats"),
    );
    add(
        36.0,
        Key(
            egui::Key::ArrowUp,
            Modifiers::ALT,
            "Alt + ↑ · Previous chat",
        ),
    );
    add(
        37.5,
        Key(egui::Key::Comma, command(), "Ctrl + , · Settings"),
    );
    add(38.0, Move(Label("Light")));
    add(38.35, Click(left));
    add(38.5, Move(Label("Dark")));
    add(38.85, Click(left));
    add(
        39.2,
        Key(egui::Key::Escape, Modifiers::NONE, "Esc · Back to chats"),
    );
    for (start, text) in [
        (0.2, "Rust"),
        (7.1, "See you tonight! :smile"),
        (12.1, "party"),
        (20.8, "@mi"),
        (21.9, " see you in the front row!"),
    ] {
        for (index, character) in text.chars().enumerate() {
            cues.push(Cue {
                at: start + index as f32 * 0.025,
                gesture: Text(character),
            });
        }
    }
    cues.sort_by(|a, b| a.at.total_cmp(&b.at));
    cues
}

#[derive(Serialize)]
struct Trace {
    at: f32,
    #[serde(flatten)]
    event: TraceEvent,
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum TraceEvent {
    Pointer {
        x: f32,
        y: f32,
    },
    Click {
        x: f32,
        y: f32,
        button: &'static str,
    },
    Keys {
        label: String,
    },
}

/// Supplies ordinary egui input. The optional trace is rendered onto video later.
pub struct Tour {
    delay: Option<Duration>,
    start: Option<Instant>,
    cues: Vec<Cue>,
    next: usize,
    previous: f32,
    pointer: Pos2,
    motion: Option<(f32, Pos2, Pos2)>,
    labels: HashMap<String, Pos2>,
    trace: Vec<Trace>,
    trace_path: Option<PathBuf>,
    saved: bool,
    failed: bool,
}

impl Tour {
    pub fn new(delay: Option<Duration>, trace_path: Option<PathBuf>) -> Self {
        Self {
            delay,
            start: None,
            cues: script(),
            next: 0,
            previous: 0.0,
            pointer: pos2(680.0, 440.0),
            motion: None,
            labels: HashMap::new(),
            trace: Vec::new(),
            trace_path,
            saved: false,
            failed: false,
        }
    }

    pub fn input(&mut self, app: &mut App, ctx: &egui::Context, input: &mut egui::RawInput) {
        let replay = input.events.iter().any(|event| {
            matches!(event,
            Event::Key { key: Key::Space, pressed: true, repeat: false, modifiers, .. }
                if modifiers.is_none())
        });
        if replay {
            input.events.retain(|event| {
                !matches!(
                    event,
                    Event::Key {
                        key: Key::Space,
                        ..
                    } | Event::Text(_)
                )
            });
            super::populate(app);
            prepare(app);
            self.start = Some(Instant::now());
            self.delay = None;
            self.next = 0;
            self.previous = 0.0;
            self.motion = None;
            self.trace.clear();
            self.saved = false;
            self.failed = false;
        }
        if let Some(start) = self.start
            && Instant::now() >= start
        {
            self.input_at(app, ctx, input, start.elapsed().as_secs_f32());
        }
    }

    fn target(&self, target: Target, app: &App, ctx: &egui::Context) -> Option<Pos2> {
        match target {
            Target::Label(label) => self.labels.get(label).copied(),
            Target::Widget(id) => ctx
                .read_response(egui::Id::new(id))
                .map(|r| r.rect.center()),
            Target::Bubble(message) => {
                let id = crate::ui::conversation::bubble_id(app.open_chat.as_deref()?, message)
                    .with("rect");
                let rect = ctx.data(|d| d.get_temp::<Rect>(id))?;
                let view = (*app.selection_view.lock().unwrap_or_else(|p| p.into_inner()))?;
                let rect = rect.intersect(view);
                rect.is_positive().then_some(rect.center())
            }
            Target::Picker => app.picker_anchor.map(|rect| rect.center()),
            Target::Gif => ctx
                .read_response(egui::Id::new("gif-search"))
                .map(|r| r.rect.left_bottom() + vec2(60.0, 55.0)),
            Target::Sticker => self.labels.get("Saved").map(|pos| *pos + vec2(25.0, 52.0)),
        }
    }

    fn input_at(&mut self, app: &App, ctx: &egui::Context, input: &mut egui::RawInput, at: f32) {
        if self.failed {
            return;
        }
        while self.next < self.cues.len() && at >= self.cues[self.next].at {
            match self.cues[self.next].gesture {
                Gesture::Move(target) => {
                    let Some(end) = self.target(target, app, ctx) else {
                        self.failed = true;
                        log::error!("tour stopped: missing UI target at step {}", self.next);
                        return;
                    };
                    self.motion = Some((at, self.pointer, end));
                }
                Gesture::Click(button) => {
                    for pressed in [true, false] {
                        input.events.push(Event::PointerButton {
                            pos: self.pointer,
                            button,
                            pressed,
                            modifiers: Modifiers::NONE,
                        });
                    }
                    self.trace.push(Trace {
                        at,
                        event: TraceEvent::Click {
                            x: self.pointer.x,
                            y: self.pointer.y,
                            button: if button == PointerButton::Secondary {
                                "right"
                            } else {
                                "left"
                            },
                        },
                    });
                }
                Gesture::Key(key, modifiers, label) => {
                    for pressed in [true, false] {
                        input.events.push(Event::Key {
                            key,
                            physical_key: None,
                            pressed,
                            repeat: false,
                            modifiers,
                        });
                    }
                    self.trace.push(Trace {
                        at,
                        event: TraceEvent::Keys {
                            label: crate::ui::keys::label(label),
                        },
                    });
                }
                Gesture::Text(character) => input.events.push(Event::Text(character.to_string())),
            }
            self.next += 1;
        }
        if let Some((began, from, to)) = self.motion {
            let t = ((at - began) / 0.28).clamp(0.0, 1.0);
            self.pointer = from.lerp(to, t * t * (3.0 - 2.0 * t));
            if t == 1.0 {
                self.motion = None;
            }
        }
        let scroll = (at.min(4.6) - self.previous.max(3.6)).max(0.0) * 340.0;
        if scroll > 0.0
            && let Some(view) = *app.selection_view.lock().unwrap_or_else(|p| p.into_inner())
        {
            self.pointer = view.center();
            input.events.push(Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: vec2(0.0, scroll),
                modifiers: Modifiers::NONE,
                phase: egui::TouchPhase::Move,
            });
        }
        input.events.insert(0, Event::PointerMoved(self.pointer));
        if self.trace.last().is_none_or(|event| {
            !matches!(event.event,
            TraceEvent::Pointer { x, y } if x == self.pointer.x && y == self.pointer.y)
        }) {
            self.trace.push(Trace {
                at,
                event: TraceEvent::Pointer {
                    x: self.pointer.x,
                    y: self.pointer.y,
                },
            });
        }
        self.previous = at;
    }

    pub fn drive(&mut self, _app: &mut App, ctx: &egui::Context) {
        let now = Instant::now();
        if let Some(delay) = self.delay.take() {
            self.start = Some(now + delay);
        }
        let Some(start) = self.start else {
            return;
        };
        if now < start {
            ctx.request_repaint_after(start - now);
        } else if start.elapsed() < DURATION && !self.failed {
            ctx.request_repaint_after(Duration::from_millis(16));
        } else if !self.saved {
            self.saved = true;
            if let Some(path) = &self.trace_path {
                let data = serde_json::json!({ "width": ctx.content_rect().width(),
                    "height": ctx.content_rect().height(), "duration": DURATION.as_secs(),
                    "complete": !self.failed, "events": self.trace });
                if let Err(error) = std::fs::write(path, data.to_string()) {
                    log::error!("could not write tour input trace: {error}");
                }
            }
        }
    }

    /// Finds click targets in the actual painted UI; no view-specific hooks or
    /// hard-coded menu coordinates are needed. Called after frame_ui.
    pub fn observe(&mut self, app: &mut App, ctx: &egui::Context) {
        session::respond(app);
        self.labels.clear();
        let layers: Vec<_> = ctx.memory(|memory| memory.layer_ids().collect());
        for layer in layers {
            let transform = ctx.layer_transform_to_global(layer).unwrap_or_default();
            ctx.graphics(|graphics| {
                if let Some(list) = graphics.get(layer) {
                    for clipped in list.all_entries() {
                        if let egui::Shape::Text(text) = &clipped.shape {
                            let rect = Rect::from_min_size(text.pos, text.galley.size());
                            if rect.intersects(clipped.clip_rect) {
                                self.labels.insert(
                                    text.galley.text().to_owned(),
                                    transform * rect.center(),
                                );
                            }
                        }
                    }
                }
            });
        }
    }
    /// Every label the last `observe` pass found, with its position.
    pub fn labels(&self) -> &HashMap<String, Pos2> {
        &self.labels
    }
}

/// Renders one demo page offscreen and returns every painted label, sorted.
/// Used by the Spanish label test and by screenshots of a translated page.
pub fn harvest_labels(page: Option<&str>) -> Vec<String> {
    let root = std::env::temp_dir().join(format!(
        "whatsfast-labels-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let (mut app, _events) = App::headless(
        crate::paths::AppDirs::under(&root),
        crate::settings::Settings::default(),
    );
    crate::demo::populate(&mut app);
    crate::demo::apply_flags(&mut app, page);
    let ctx = egui::Context::default();
    app.attach(&ctx);
    let mut tour = Tour::new(None, None);
    for _ in 0..3 {
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1180.0, 780.0))),
            ..Default::default()
        };
        let mut output = ctx.run_ui(input, |ui| {
            app.background_frame(&ctx);
            app.frame_ui(ui);
            tour.observe(&mut app, &ctx);
        });
        output.textures_delta.clear();
    }
    let mut labels: Vec<String> = tour.labels().keys().cloned().collect();
    labels.sort();
    labels
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Action, Dialog, PickerTab};
    use crate::settings::HistoryPrefetch;

    fn frame(app: &mut App, tour: &mut Tour, ctx: &egui::Context, events: Vec<Event>) {
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1180.0, 2000.0))),
            events,
            ..Default::default()
        };
        let mut output = ctx.run_ui(input, |ui| {
            app.background_frame(ctx);
            app.frame_ui(ui);
            tour.observe(app, ctx);
        });
        output.textures_delta.clear();
    }
    fn click(app: &mut App, tour: &mut Tour, ctx: &egui::Context, label: &str) {
        let pos = *tour
            .labels
            .get(label)
            .unwrap_or_else(|| panic!("missing {label}"));
        for pressed in [true, false] {
            frame(
                app,
                tour,
                ctx,
                vec![
                    Event::PointerMoved(pos),
                    Event::PointerButton {
                        pos,
                        button: PointerButton::Primary,
                        pressed,
                        modifiers: Modifiers::NONE,
                    },
                ],
            );
        }
        frame(app, tour, ctx, Vec::new());
    }

    #[test]
    fn polls_are_created_and_voted_through_real_controls() {
        let mut app = super::super::tests::app();
        app.backend.record_demo_commands();
        let ctx = egui::Context::default();
        app.attach(&ctx);
        let mut tour = Tour::new(None, None);
        let chat = app.open_chat.clone().unwrap();
        for multiple in [false, true] {
            app.actions
                .push(crate::model::Action::ShowDialog(Dialog::CreatePoll(
                    chat.clone(),
                )));
            frame(&mut app, &mut tour, &ctx, Vec::new());
            app.poll_draft = crate::model::PollDraft {
                question: "Lunch?".into(),
                options: vec!["Pizza".into(), "Pasta".into()],
                multiple,
            };
            for _ in 0..3 {
                frame(&mut app, &mut tour, &ctx, Vec::new());
            }
            click(&mut app, &mut tour, &ctx, "Send poll");
            assert!(app.dialog.is_none());
            assert!(!app.poll_creating);
            for _ in 0..3 {
                frame(&mut app, &mut tour, &ctx, Vec::new());
            }
            click(&mut app, &mut tour, &ctx, "Pizza");
            click(&mut app, &mut tour, &ctx, "Pasta");
            let Content::Poll { state, .. } =
                &app.conversations[&chat].messages.last().unwrap().content
            else {
                panic!("poll")
            };
            assert_eq!(state.selected, if multiple { vec![0, 1] } else { vec![1] });
            assert_eq!(state.voters, 1);
            click(&mut app, &mut tour, &ctx, "Pasta");
            if multiple {
                click(&mut app, &mut tour, &ctx, "Pizza");
            }
            let Content::Poll { state, .. } =
                &app.conversations[&chat].messages.last().unwrap().content
            else {
                panic!("poll")
            };
            assert!(state.selected.is_empty());
            assert_eq!(state.counts, vec![0, 0]);
            assert_eq!(state.voters, 0);
            assert!(app.poll_voting.is_empty());
        }
    }

    #[test]
    fn the_theme_dropdown_selects_spotifast_palettes_and_returns_to_follow_system() {
        let mut app = super::super::tests::app();
        app.page = Page::Settings;
        app.custom_themes = crate::theme::custom::Catalog::preview(
            crate::theme::presets::themes().collect(),
            false,
        );
        let ctx = egui::Context::default();
        app.attach(&ctx);
        let mut tour = Tour::new(None, None);
        for _ in 0..3 {
            frame(&mut app, &mut tour, &ctx, Vec::new());
        }
        click(&mut app, &mut tour, &ctx, "Dark");
        for name in [
            "Follow system",
            "Light",
            "Dark",
            "Catppuccin Latte.json",
            "Catppuccin.json",
            "Nord.json",
            "Ristretto.json",
            "Tokyo Night.json",
        ] {
            assert!(
                tour.labels.contains_key(name),
                "missing theme choice {name}"
            );
        }
        click(&mut app, &mut tour, &ctx, "Nord.json");
        assert_eq!(app.settings.custom_theme.as_deref(), Some("Nord.json"));
        assert_eq!(
            app.palette.window,
            egui::Color32::from_rgb(0x2e, 0x34, 0x40)
        );
        click(&mut app, &mut tour, &ctx, "Nord.json");
        click(&mut app, &mut tour, &ctx, "Follow system");
        assert!(app.settings.custom_theme.is_none());
        assert_eq!(app.settings.theme, ThemeChoice::System);
    }

    #[test]
    fn the_history_prefetch_dropdown_lists_the_three_modes() {
        let mut app = super::super::tests::app();
        app.page = Page::Settings;
        let ctx = egui::Context::default();
        app.attach(&ctx);
        let mut tour = Tour::new(None, None);
        for _ in 0..3 {
            let input = egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1180.0, 2000.0))),
                events: Vec::new(),
                ..Default::default()
            };
            let mut output = ctx.run_ui(input, |ui| {
                app.background_frame(&ctx);
                app.frame_ui(ui);
                tour.observe(&mut app, &ctx);
            });
            output.textures_delta.clear();
        }
        click(&mut app, &mut tour, &ctx, "Recent and pinned");
        for name in ["Off", "Current Chat", "Recent and pinned"] {
            assert!(
                tour.labels.contains_key(name),
                "missing history prefetch choice {name}"
            );
        }
        click(&mut app, &mut tour, &ctx, "Current Chat");
        assert_eq!(app.settings.history_prefetch, HistoryPrefetch::Focused);
        assert!(
            tour.labels.keys().any(|text| text.contains("30 days")),
            "settings copy should mention the 30-day retry"
        );
    }

    #[test]
    fn hovering_a_prefetch_choice_pins_its_hint_to_that_row() {
        let mut app = super::super::tests::app();
        app.page = Page::Settings;
        let ctx = egui::Context::default();
        app.attach(&ctx);
        let mut tour = Tour::new(None, None);
        for _ in 0..3 {
            frame(&mut app, &mut tour, &ctx, Vec::new());
        }
        click(&mut app, &mut tour, &ctx, "Recent and pinned");
        let option = *tour.labels.get("Current Chat").expect("prefetch choice");
        for _ in 0..3 {
            frame(&mut app, &mut tour, &ctx, vec![Event::PointerMoved(option)]);
        }
        let hint = crate::i18n::t(crate::i18n::Key::SettingsHistoryCurrentHint);
        let pos = tour
            .labels
            .iter()
            .find(|(text, _)| text.as_str() == hint || text.contains("open chat only"))
            .map(|(_, pos)| *pos)
            .unwrap_or_else(|| {
                let nearby: Vec<_> = tour
                    .labels
                    .keys()
                    .filter(|text| {
                        text.contains("Fetch")
                            || text.contains("chat")
                            || text.contains("Chat")
                            || text.contains("30 days")
                    })
                    .collect();
                panic!("choice hint missing; nearby labels: {nearby:?}")
            });
        assert!(
            (pos.y - option.y).abs() < 48.0,
            "hint at {pos:?} should sit on the hovered row at {option:?}"
        );
        assert!(
            pos.x > option.x,
            "hint at {pos:?} should sit to the right of {option:?}"
        );
    }

    #[test]
    fn receipts_and_typing_sit_in_settings_privacy() {
        let mut app = super::super::tests::app();
        app.page = Page::Settings;
        let ctx = egui::Context::default();
        app.attach(&ctx);
        let mut tour = Tour::new(None, None);
        for _ in 0..3 {
            frame(&mut app, &mut tour, &ctx, Vec::new());
        }
        let privacy = *tour.labels.get("Privacy").expect("Privacy section");
        let last_seen = *tour.labels.get("Last seen").expect("account last seen");
        let enter = *tour.labels.get("Enter sends").expect("Chats row");
        let receipts = *tour
            .labels
            .get("Send read receipts")
            .expect("read receipts");
        let typing = *tour.labels.get("Show when you are typing").expect("typing");
        assert!(
            enter.y < privacy.y,
            "Chats should stay above Privacy: enter {enter:?} privacy {privacy:?}"
        );
        assert!(
            privacy.y < receipts.y && receipts.y < last_seen.y,
            "receipts at {receipts:?} should sit between Privacy {privacy:?} and Last seen {last_seen:?}"
        );
        assert!(
            privacy.y < typing.y && typing.y < last_seen.y,
            "typing at {typing:?} should sit between Privacy {privacy:?} and Last seen {last_seen:?}"
        );
    }

    #[test]
    fn the_chat_wallpaper_dropdown_lists_auto_and_numbered_families() {
        use crate::settings::ChatWallpaper;
        let mut app = super::super::tests::app();
        app.page = Page::Settings;
        let ctx = egui::Context::default();
        app.attach(&ctx);
        let mut tour = Tour::new(None, None);
        for _ in 0..3 {
            frame(&mut app, &mut tour, &ctx, Vec::new());
        }
        click(&mut app, &mut tour, &ctx, "Auto");
        assert!(
            tour.labels.contains_key("Auto"),
            "missing chat wallpaper choice Auto"
        );
        assert!(
            tour.labels.contains_key("Black 1"),
            "missing chat wallpaper choice Black 1"
        );
        click(&mut app, &mut tour, &ctx, "Green 1");
        assert_eq!(app.settings.chat_wallpaper, ChatWallpaper::Green);
        assert_eq!(app.settings.chat_wallpaper_index, 0);
    }

    #[test]
    fn settings_downloads_shows_usage_and_opens_the_media_folder() {
        let mut app = super::super::tests::app();
        app.page = Page::Settings;
        let ctx = egui::Context::default();
        app.attach(&ctx);
        let mut tour = Tour::new(None, None);
        for _ in 0..3 {
            let input = egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1180.0, 2000.0))),
                events: Vec::new(),
                ..Default::default()
            };
            let mut output = ctx.run_ui(input, |ui| {
                app.background_frame(&ctx);
                app.frame_ui(ui);
                tour.observe(&mut app, &ctx);
            });
            output.textures_delta.clear();
        }
        for name in [
            "Downloads",
            "Images",
            "Videos",
            "Stickers and GIFs",
            "Other",
            "Open folder",
        ] {
            assert!(
                tour.labels.contains_key(name),
                "missing downloads label {name}"
            );
        }
        crate::ui::settings::open_media_folder(&mut app);
        let media = app.dirs.media_cache_dir();
        assert!(
            matches!(app.actions.last(), Some(Action::OpenFile(path)) if *path == media),
            "Open folder should target the media cache"
        );
        assert!(media.is_dir(), "Open folder should create the media cache");
    }

    #[test]
    fn real_input_opens_menus_completes_text_and_sends_offline_media() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        let ctx = egui::Context::default();
        app.attach(&ctx);
        let mut tour = Tour::new(None, None);
        let mut seen = [false; 7];
        for frame in 0..=42 * 60 {
            let at = frame as f32 / 60.0;
            let mut input = egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1280.0, 800.0))),
                time: Some(at as f64),
                ..Default::default()
            };
            // Give the first screen a frame to establish its hit targets.
            if frame > 0 {
                tour.input_at(&app, &ctx, &mut input, at);
            }
            let mut output = ctx.run_ui(input, |ui| {
                app.background_frame(&ctx);
                app.frame_ui(ui);
                tour.observe(&mut app, &ctx);
                seen[0] |= egui::Popup::is_any_open(&ctx);
                seen[1] |= app.reply_to.is_some();
                seen[2] |= app.picker == Some(PickerTab::Gifs);
                seen[3] |= app.picker == Some(PickerTab::Stickers);
                seen[4] |= matches!(app.dialog, Some(Dialog::ChatInfo(_)));
                seen[5] |= matches!(app.dialog, Some(Dialog::Shortcuts));
                seen[6] |= app.settings.theme == ThemeChoice::Light;
            });
            output.textures_delta.clear();
            assert!(
                !tour.failed,
                "missing target at {at:.2}s, cue {}",
                tour.next
            );
        }
        assert_eq!(
            seen, [true; 7],
            "every advertised interaction must be visible"
        );
        let ada = &app.conversations[super::super::SAMPLES[0].id];
        let sent: Vec<_> = ada
            .messages
            .iter()
            .filter(|row| row.id.starts_with("tour-"))
            .collect();
        assert_eq!(
            sent.len(),
            2,
            "reply and still sticker through real send commands"
        );
        assert!(sent[0].quoted.is_some());
        assert!(matches!(&sent[0].content, Content::Text { text, .. }
            if text.starts_with("See you tonight! ") && !text.contains(':')));
        assert!(
            matches!(&sent[1].content, Content::Sticker { animated: false, media }
            if media.path.as_ref().unwrap().is_file())
        );
        for row in &ada.messages {
            assert!(!matches!(
                row.content,
                Content::Sticker { animated: true, .. }
            ));
        }
        let group = &app.conversations[super::super::SAMPLES[1].id];
        assert_eq!(group.messages.last().unwrap().mentions.len(), 1);
        assert_eq!(app.settings.theme, ThemeChoice::Dark);
        assert!(app.backend.is_offline());
        assert!(app.composer.is_empty());
        assert!(app.dialog.is_none());

        let mut input = egui::RawInput::default();
        input.events.push(Event::Key {
            key: Key::Space,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        });
        input.events.push(Event::Text(" ".into()));
        tour.input(&mut app, &ctx, &mut input);
        assert_eq!(tour.next, 1, "replay immediately runs the first shortcut");
        assert!(
            app.conversations[super::super::SAMPLES[0].id]
                .messages
                .iter()
                .all(|row| !row.id.starts_with("tour-"))
        );
    }

    /// Runs one frame with the given pointer events.
    fn step(app: &mut App, ctx: &egui::Context, events: Vec<Event>, at: f32) {
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1280.0, 800.0))),
            time: Some(at as f64),
            events,
            ..Default::default()
        };
        let mut output = ctx.run_ui(input, |ui| {
            app.background_frame(ctx);
            app.frame_ui(ui);
        });
        output.textures_delta.clear();
    }

    /// A press and its release, the shape the tour uses for a click.
    fn click_events(pos: Pos2, button: PointerButton) -> Vec<Event> {
        [true, false]
            .into_iter()
            .map(|pressed| Event::PointerButton {
                pos,
                button,
                pressed,
                modifiers: Modifiers::NONE,
            })
            .collect()
    }

    /// Runs one frame and hands back its output, for shape checks.
    fn step_output(
        app: &mut App,
        ctx: &egui::Context,
        events: Vec<Event>,
        at: f32,
        focused: bool,
    ) -> egui::FullOutput {
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(1280.0, 800.0))),
            time: Some(at as f64),
            focused,
            events,
            ..Default::default()
        };
        let mut output = ctx.run_ui(input, |ui| {
            app.background_frame(ctx);
            app.frame_ui(ui);
        });
        output.textures_delta.clear();
        output
    }

    /// Whether the composer's caret is painted in the composer's band.
    fn caret_in(output: &egui::FullOutput, app: &App) -> bool {
        output.shapes.iter().any(|clipped| {
            matches!(&clipped.shape, egui::Shape::LineSegment { points, stroke }
                if stroke.width >= 1.5
                    && stroke.color == app.palette.accent
                    && points[0].y > 700.0
                    && points[1].y > 700.0)
        })
    }

    #[test]
    fn the_composer_shows_its_caret_while_the_window_has_focus() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        let ctx = egui::Context::default();
        app.attach(&ctx);
        step_output(&mut app, &ctx, Vec::new(), 0.0, true);
        // egui hides a text field's caret when the raw input says the window
        // lost focus, and eframe's native backends never set that flag, which
        // is why the composer had no caret. Shell::raw_input_hook now mirrors
        // the viewport's real focus into it.
        ctx.memory_mut(|memory| memory.request_focus(egui::Id::new("composer-text")));
        let focused = step_output(&mut app, &ctx, Vec::new(), 0.1, true);
        assert!(
            ctx.memory(|memory| memory.has_focus(egui::Id::new("composer-text"))),
            "the composer takes focus"
        );
        assert!(
            caret_in(&focused, &app),
            "the caret is painted in the composer while the window has focus"
        );
        let bubble = ctx
            .read_response(egui::Id::new("composer-bubble"))
            .expect("composer bubble")
            .rect;
        let caret_h = focused
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::LineSegment { points, stroke }
                    if stroke.width >= 1.5
                        && stroke.color == app.palette.accent
                        && points[0].y > 700.0 =>
                {
                    Some((points[1].y - points[0].y).abs())
                }
                _ => None,
            })
            .fold(0.0_f32, f32::max);
        assert!(
            caret_h + 0.5 >= bubble.height(),
            "caret height {caret_h} should fill the empty composer {bubble_h}",
            bubble_h = bubble.height()
        );
        let unfocused = step_output(&mut app, &ctx, Vec::new(), 0.2, false);
        assert!(
            !caret_in(&unfocused, &app),
            "a window without focus keeps the caret hidden"
        );
    }

    #[test]
    fn the_empty_composer_is_taller_than_one_line_and_clear_of_the_window_edge() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        let ctx = egui::Context::default();
        app.attach(&ctx);
        for at in 0..3 {
            step_output(&mut app, &ctx, Vec::new(), at as f32 * 0.05, true);
        }
        let rect = ctx
            .read_response(egui::Id::new("composer-bubble"))
            .expect("composer bubble")
            .rect;
        let line = ctx.fonts_mut(|fonts| {
            fonts
                .layout_no_wrap(
                    "x".to_owned(),
                    crate::theme::regular(14.5),
                    app.palette.text,
                )
                .size()
                .y
        });
        assert!(
            rect.height() + 0.5 >= line * 1.35,
            "empty composer height {} should be at least 35% taller than one line {}",
            rect.height(),
            line
        );
        assert!(
            800.0 - rect.bottom() >= 10.0,
            "composer sits above the window edge, gap {}",
            800.0 - rect.bottom()
        );
    }

    #[test]
    fn the_schedule_dialog_stacks_its_parts() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        let ctx = egui::Context::default();
        app.attach(&ctx);
        step_output(&mut app, &ctx, Vec::new(), 0.0, true);
        let chat = app.open_chat.clone().expect("a chat is open");
        app.composer = "See you at nine".to_owned();
        app.dialog = Some(crate::model::Dialog::ScheduleMessage(chat));
        step_output(&mut app, &ctx, Vec::new(), 0.1, true);
        let output = step_output(&mut app, &ctx, Vec::new(), 0.2, true);

        let months = [
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
        let mut month: Option<Pos2> = None;
        let mut numbers: Vec<(String, Pos2)> = Vec::new();
        let mut time: Option<Pos2> = None;
        let mut repeat: Option<Pos2> = None;
        let mut schedule: Option<Pos2> = None;
        for clipped in &output.shapes {
            let egui::Shape::Text(text) = &clipped.shape else {
                continue;
            };
            let content = text.galley.text();
            assert!(
                text.pos.x.is_finite() && text.pos.y.is_finite(),
                "every label has a real position"
            );
            if months.iter().any(|name| content.starts_with(name)) {
                month = Some(text.pos);
            } else if content.parse::<u8>().is_ok() {
                numbers.push((content.to_owned(), text.pos));
            } else if content == "Time" {
                time = Some(text.pos);
            } else if content == "Repeat" {
                repeat = Some(text.pos);
            } else if content == "Schedule" {
                schedule = Some(text.pos);
            }
        }
        let month = month.expect("the month header is drawn");
        let time = time.expect("the time row is drawn");
        let repeat = repeat.expect("the repeat row is drawn");
        schedule.expect("the schedule button is drawn");
        // The calendar's days: the numbers between the month header and the
        // time row, so numbers elsewhere on the screen do not count.
        let days: Vec<Pos2> = numbers
            .iter()
            .filter(|(_, pos)| pos.y > month.y + 10.0 && pos.y < time.y)
            .map(|(_, pos)| *pos)
            .collect();
        assert!(days.len() >= 28, "a month shows its days");
        let first_day = days.iter().map(|pos| pos.y).fold(f32::MAX, f32::min);
        let last_day = days.iter().map(|pos| pos.y).fold(f32::MIN, f32::max);
        assert!(
            month.y < first_day,
            "the month header sits above the calendar, not beside it"
        );
        assert!(
            time.y > last_day && repeat.y > last_day,
            "the time and repeat rows sit under the calendar"
        );
        assert!(repeat.y > time.y, "repeat follows time");
        // Seven to a row at most, and the month wraps onto several rows.
        let mut rows: Vec<f32> = days.iter().map(|pos| pos.y).collect();
        rows.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        rows.dedup_by(|a, b| (*a - *b).abs() < 4.0);
        assert!(rows.len() >= 4, "the days wrap onto several rows");
        for row in &rows {
            let count = days.iter().filter(|pos| (pos.y - row).abs() < 4.0).count();
            assert!(count <= 7, "no row holds more than seven days");
        }
    }

    #[test]
    fn the_leave_group_dialog_offers_leave_and_archive() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        let ctx = egui::Context::default();
        app.attach(&ctx);
        let group = super::super::sample_ids()[1].to_owned();
        app.open_chat = Some(group.clone());
        app.dialog = Some(crate::model::Dialog::ChatInfo(group.clone()));
        let mut info = None;
        for at in 0..3 {
            info = Some(step_output(
                &mut app,
                &ctx,
                Vec::new(),
                at as f32 * 0.05,
                true,
            ));
        }
        let info = info.expect("chat info");
        assert!(
            info.shapes.iter().any(|clipped| matches!(
                &clipped.shape,
                egui::Shape::Text(text) if text.galley.text() == "Leave group"
            )),
            "group info offers Leave group"
        );
        app.dialog = Some(crate::model::Dialog::ConfirmLeaveGroup(group));
        let confirm = step_output(&mut app, &ctx, Vec::new(), 0.1, true);
        let mut leave = false;
        let mut archive = false;
        let mut title = false;
        for clipped in &confirm.shapes {
            let egui::Shape::Text(text) = &clipped.shape else {
                continue;
            };
            let content = text.galley.text();
            leave |= content == "Leave group";
            archive |= content == "Leave group and archive";
            title |= content == "Leave this group?";
        }
        assert!(title, "the confirm dialog names the action");
        assert!(leave, "Leave group is offered");
        assert!(archive, "Leave group and archive is offered");
    }

    #[test]
    fn the_leave_channel_dialog_offers_leave_and_archive() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        let ctx = egui::Context::default();
        app.attach(&ctx);
        let channel = super::super::sample_ids()
            .into_iter()
            .find(|id| id.ends_with("@newsletter"))
            .expect("channel sample")
            .to_owned();
        app.open_chat = Some(channel.clone());
        app.dialog = Some(crate::model::Dialog::ChatInfo(channel.clone()));
        let mut info = None;
        for at in 0..3 {
            info = Some(step_output(
                &mut app,
                &ctx,
                Vec::new(),
                at as f32 * 0.05,
                true,
            ));
        }
        let info = info.expect("chat info");
        assert!(
            info.shapes.iter().any(|clipped| matches!(
                &clipped.shape,
                egui::Shape::Text(text) if text.galley.text() == "Leave channel"
            )),
            "channel info offers Leave channel"
        );
        app.dialog = Some(crate::model::Dialog::ConfirmLeaveGroup(channel));
        let confirm = step_output(&mut app, &ctx, Vec::new(), 0.1, true);
        let mut leave = false;
        let mut archive = false;
        let mut title = false;
        for clipped in &confirm.shapes {
            let egui::Shape::Text(text) = &clipped.shape else {
                continue;
            };
            let content = text.galley.text();
            leave |= content == "Leave channel";
            archive |= content == "Leave channel and archive";
            title |= content == "Leave this channel?";
        }
        assert!(title, "the confirm dialog names the action");
        assert!(leave, "Leave channel is offered");
        assert!(archive, "Leave channel and archive is offered");
    }

    #[test]
    fn the_chat_header_search_opens_the_right_pane() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        let ctx = egui::Context::default();
        app.attach(&ctx);
        step_output(&mut app, &ctx, Vec::new(), 0.0, true);
        assert!(app.right_pane.is_none());
        // More is the rightmost 30px control; Search sits one slot left.
        let search = pos2(1280.0 - 14.0 - 30.0 - 8.0 - 15.0, 8.0 + 22.0);
        step_output(
            &mut app,
            &ctx,
            click_events(search, PointerButton::Primary),
            0.1,
            true,
        );
        assert_eq!(
            app.right_pane,
            Some(crate::model::RightPane::Search),
            "header Search opens the inspector"
        );
        assert!(!app.focus_search, "Ctrl+F still owns the left list");
        let output = step_output(&mut app, &ctx, Vec::new(), 0.15, true);
        assert!(
            output.shapes.iter().any(|clipped| matches!(
                &clipped.shape,
                egui::Shape::Text(text)
                    if text.galley.text() == "Search messages with Ada Lovelace"
            )),
            "empty pane names the open chat"
        );
        assert!(app.sidebar_visible);
        assert_eq!(app.page, Page::Chats);
        let search = pos2(
            1280.0 - app.settings.inspector_width - 14.0 - 30.0 - 8.0 - 15.0,
            8.0 + 22.0,
        );
        step_output(
            &mut app,
            &ctx,
            click_events(search, PointerButton::Primary),
            0.2,
            true,
        );
        assert!(app.right_pane.is_none(), "a second click closes the pane");
    }

    #[test]
    fn clicking_outside_closes_the_search_calendar() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        app.right_pane = Some(crate::model::RightPane::Search);
        let ctx = egui::Context::default();
        app.attach(&ctx);
        step_output(&mut app, &ctx, Vec::new(), 0.0, true);
        let button = ctx
            .data(|data| data.get_temp::<egui::Rect>(egui::Id::new("chat-search-calendar-button")))
            .expect("the calendar button is drawn");
        step_output(
            &mut app,
            &ctx,
            click_events(button.center(), PointerButton::Primary),
            0.1,
            true,
        );
        assert!(app.chat_search_calendar, "the icon opens the calendar");
        step_output(
            &mut app,
            &ctx,
            click_events(pos2(80.0, 400.0), PointerButton::Primary),
            0.2,
            true,
        );
        assert!(
            !app.chat_search_calendar,
            "a click outside the popup closes it"
        );
    }

    #[test]
    fn search_calendar_sits_below_and_centers_on_the_icon() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        app.right_pane = Some(crate::model::RightPane::Search);
        app.chat_search_calendar = true;
        let ctx = egui::Context::default();
        app.attach(&ctx);
        step_output(&mut app, &ctx, Vec::new(), 0.0, true);
        step_output(&mut app, &ctx, Vec::new(), 0.05, true);
        let button = ctx
            .data(|data| data.get_temp::<egui::Rect>(egui::Id::new("chat-search-calendar-button")))
            .expect("the calendar button is drawn");
        let popup = ctx
            .data(|data| data.get_temp::<egui::Rect>(egui::Id::new("chat-search-calendar")))
            .expect("the calendar popup is drawn");
        assert!(
            popup.top() >= button.bottom(),
            "the popup sits below the icon"
        );
        assert!(
            (popup.center().x - button.center().x).abs() < 2.0,
            "the popup is centered on the icon"
        );
    }

    #[test]
    fn clicking_a_chat_search_hit_pulses_the_row() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        let chat = super::super::SAMPLES[0].id;
        let hit = app
            .conversations
            .get(chat)
            .and_then(|conversation| {
                conversation
                    .messages
                    .iter()
                    .find(|message| message.summary().to_lowercase().contains("engine"))
                    .cloned()
            })
            .expect("an engine message");
        app.right_pane = Some(crate::model::RightPane::Search);
        app.chat_search = "engine".into();
        app.chat_search_hits = vec![hit.clone()];
        let ctx = egui::Context::default();
        app.attach(&ctx);
        step_output(&mut app, &ctx, Vec::new(), 0.0, true);
        let id = egui::Id::new(("chat-search-hit", chat, hit.id.as_str()));
        let target = ctx
            .read_response(id)
            .expect("the hit is drawn in the pane")
            .rect
            .center();
        step_output(
            &mut app,
            &ctx,
            {
                let mut events = vec![Event::PointerMoved(target)];
                events.extend(click_events(target, PointerButton::Primary));
                events
            },
            0.1,
            true,
        );
        assert_eq!(
            app.highlight.as_ref().map(|(id, _)| id.as_str()),
            Some(hit.id.as_str())
        );
        assert_eq!(app.scroll_anchor.as_deref(), Some(hit.id.as_str()));
        // One frame registers the row, the next paints its wash.
        step_output(&mut app, &ctx, Vec::new(), 0.15, true);
        let output = step_output(&mut app, &ctx, Vec::new(), 0.2, true);
        let expected = app.palette.accent.gamma_multiply(0.16);
        assert!(
            output.shapes.iter().any(|clipped| {
                matches!(&clipped.shape, egui::Shape::Rect(rect) if rect.fill == expected)
            }),
            "the pulse washes the full message row"
        );
    }

    fn double_click(pos: Pos2) -> Vec<Event> {
        let mut events = vec![Event::PointerMoved(pos)];
        events.extend(click_events(pos, PointerButton::Primary));
        events.extend(click_events(pos, PointerButton::Primary));
        events
    }

    fn last_incoming_text(app: &crate::app::App) -> crate::model::Message {
        let chat = super::super::SAMPLES[0].id;
        app.conversations
            .get(chat)
            .and_then(|conversation| {
                conversation.messages.iter().rev().find(|message| {
                    !message.from_me && matches!(message.content, Content::Text { .. })
                })
            })
            .cloned()
            .expect("an incoming message")
    }

    #[test]
    fn double_clicking_a_bubble_replies_and_pulses_once() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        let chat = super::super::SAMPLES[0].id;
        let hit = last_incoming_text(&app);
        let ctx = egui::Context::default();
        app.attach(&ctx);
        step_output(&mut app, &ctx, Vec::new(), 0.0, true);
        let target = ctx
            .read_response(crate::ui::conversation::bubble_id(chat, &hit.id))
            .expect("the bubble is drawn")
            .rect
            .center();
        step_output(&mut app, &ctx, double_click(target), 0.10, true);
        assert_eq!(app.reply_to.as_deref(), Some(hit.id.as_str()));
        assert_eq!(app.highlight_ons, 1);
        let output = step_output(&mut app, &ctx, Vec::new(), 0.12, true);
        let expected = app.palette.accent.gamma_multiply(0.16);
        assert!(
            output.shapes.iter().any(|clipped| {
                matches!(&clipped.shape, egui::Shape::Rect(rect) if rect.fill == expected)
            }),
            "reply washes the full message row once"
        );
    }

    #[test]
    fn double_clicking_the_row_beside_a_bubble_replies() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        let chat = super::super::SAMPLES[0].id;
        let hit = last_incoming_text(&app);
        let ctx = egui::Context::default();
        app.attach(&ctx);
        step_output(&mut app, &ctx, Vec::new(), 0.0, true);
        let row = ctx
            .data(|data| {
                data.get_temp::<(Rect, bool)>(egui::Id::new(("message-row", chat, hit.id.as_str())))
            })
            .expect("the row stored its rect")
            .0;
        let bubble = ctx
            .read_response(crate::ui::conversation::bubble_id(chat, &hit.id))
            .expect("the bubble is drawn")
            .rect;
        let target = pos2(row.right() - 16.0, row.center().y);
        assert!(
            !bubble.contains(target),
            "the click sits in the empty strip beside the bubble"
        );
        step_output(&mut app, &ctx, double_click(target), 0.10, true);
        assert_eq!(app.reply_to.as_deref(), Some(hit.id.as_str()));
    }

    #[test]
    fn holding_a_pinned_row_starts_the_gesture_and_the_release_ends_it() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        let open = app.open_chat.clone();
        // Pinned but not the chat already open, so a click that slips
        // through shows up instead of looking like no change.
        for (index, chat) in app.chats.iter_mut().enumerate() {
            chat.pinned = index >= 1;
            chat.pinned_at = 1_000 - index as i64;
        }
        let ctx = egui::Context::default();
        app.attach(&ctx);
        let output = step_output(&mut app, &ctx, Vec::new(), 0.0, true);
        let title = app
            .visible_chats()
            .first()
            .map(|chat| app.chat_title(chat))
            .expect("a chat");
        let from = output
            .shapes
            .iter()
            .find_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text) if text.galley.text() == title && text.pos.x < 320.0 => {
                    Some(text.pos + vec2(20.0, 6.0))
                }
                _ => None,
            })
            .expect("the pinned row is drawn in the list");
        let press = [egui::Event::PointerButton {
            pos: from,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        }];
        step_output(&mut app, &ctx, press.to_vec(), 0.1, true);
        assert!(app.pin_drag.is_some(), "the press starts the gesture");
        // A still pointer sends no events: only the clock moves.
        step_output(&mut app, &ctx, Vec::new(), 0.5, true);
        assert!(
            app.pin_drag.as_ref().is_some_and(|drag| drag.active),
            "holding past the threshold turns the gesture on"
        );
        // The second pinned row, to have somewhere to move the first one.
        let second = app
            .visible_chats()
            .get(1)
            .map(|chat| app.chat_title(chat))
            .expect("a second pinned chat");
        let onto = output
            .shapes
            .iter()
            .find_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text) if text.galley.text() == second && text.pos.x < 320.0 => {
                    Some(text.pos + vec2(20.0, 6.0))
                }
                _ => None,
            })
            .expect("the second row is drawn in the list");
        let moved = [egui::Event::PointerMoved(onto)];
        let output = step_output(&mut app, &ctx, moved.to_vec(), 0.6, true);
        // The list already draws the order a release would write: the held
        // row sits where the second one was, and its slot shows the gap.
        let rows = list_rows(&output);
        let spots: Vec<f32> = rows
            .iter()
            .filter(|(_, text)| text == &title)
            .map(|(y, _)| *y)
            .collect();
        assert_eq!(
            spots.len(),
            2,
            "the held chat is drawn in its slot and under the pointer"
        );
        assert!(
            (spots[0] - (onto.y - 6.0)).abs() < 24.0,
            "the held chat took the slot under the pointer before the release"
        );
        let release = [egui::Event::PointerButton {
            pos: onto,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }];
        step_output(&mut app, &ctx, release.to_vec(), 0.7, true);
        assert!(app.pin_drag.is_none(), "the release ends the gesture");
        assert_eq!(
            app.open_chat.as_deref(),
            open.as_deref(),
            "a hold that ends elsewhere does not open the chat"
        );
    }

    /// The text the chat list draws in the left panel, top first.
    fn list_rows(output: &egui::FullOutput) -> Vec<(f32, String)> {
        let mut rows: Vec<(f32, String)> = output
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text) if text.pos.x < 320.0 => {
                    Some((text.pos.y, text.galley.text().to_owned()))
                }
                _ => None,
            })
            .collect();
        rows.sort_by(|left, right| left.0.total_cmp(&right.0));
        rows
    }

    #[test]
    fn the_rail_stays_with_a_chat_open() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        app.sidebar_visible = false;
        let ctx = egui::Context::default();
        app.attach(&ctx);
        let output = step_output(&mut app, &ctx, Vec::new(), 0.0, true);
        assert!(app.open_chat.is_some(), "a chat is open");
        assert!(
            output.shapes.iter().any(|clipped| matches!(
                &clipped.shape,
                egui::Shape::Circle(circle) if circle.center.x < 72.0 && circle.radius > 20.0
            )),
            "the rail draws the chat pictures with a chat open too"
        );
    }

    #[test]
    fn the_compact_sidebar_opens_a_chat_from_its_picture() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        app.sidebar_visible = false;
        app.open_chat = None;
        app.page = crate::model::Page::Chats;
        let ctx = egui::Context::default();
        app.attach(&ctx);
        let output = step_output(&mut app, &ctx, Vec::new(), 0.0, true);
        let expected = app.visible_chats().first().map(|chat| chat.id.clone());
        let spot = output
            .shapes
            .iter()
            .find_map(|clipped| match &clipped.shape {
                egui::Shape::Circle(circle) if circle.center.x < 72.0 && circle.radius > 20.0 => {
                    Some(circle.center)
                }
                _ => None,
            })
            .expect("the rail draws a chat picture");
        step_output(
            &mut app,
            &ctx,
            click_events(spot, egui::PointerButton::Primary),
            0.1,
            true,
        );
        assert_eq!(
            app.open_chat, expected,
            "clicking a picture in the rail opens that chat"
        );
    }

    #[test]
    fn a_short_click_on_a_pinned_row_still_opens_the_chat() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        // Pinned but not the chat already open, so a click that slips
        // through shows up instead of looking like no change.
        for (index, chat) in app.chats.iter_mut().enumerate() {
            chat.pinned = index >= 1;
            chat.pinned_at = 1_000 - index as i64;
        }
        let ctx = egui::Context::default();
        app.attach(&ctx);
        let output = step_output(&mut app, &ctx, Vec::new(), 0.0, true);
        let pinned = app
            .visible_chats()
            .first()
            .map(|chat| chat.id.clone())
            .expect("a chat");
        let title = app.chat_title(app.chat(&pinned).expect("the chat exists"));
        let spot = output
            .shapes
            .iter()
            .find_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text) if text.galley.text() == title && text.pos.x < 320.0 => {
                    Some(text.pos + vec2(20.0, 6.0))
                }
                _ => None,
            })
            .expect("the pinned row is drawn in the list");
        step_output(
            &mut app,
            &ctx,
            click_events(spot, egui::PointerButton::Primary),
            0.1,
            true,
        );
        assert_eq!(
            app.open_chat.as_deref(),
            Some(pinned.as_str()),
            "a short click on a pinned row opens its chat"
        );
    }

    #[test]
    fn a_second_click_on_settings_closes_it() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        let ctx = egui::Context::default();
        app.attach(&ctx);
        step_output(&mut app, &ctx, Vec::new(), 0.0, true);
        assert!(app.open_chat.is_some(), "a chat is open to start");
        app.actions.push(crate::model::Action::ToggleSettings);
        step_output(&mut app, &ctx, Vec::new(), 0.1, true);
        assert_eq!(
            app.page,
            crate::model::Page::Settings,
            "the first click opens settings"
        );
        app.actions.push(crate::model::Action::ToggleSettings);
        step_output(&mut app, &ctx, Vec::new(), 0.2, true);
        assert_eq!(
            app.page,
            crate::model::Page::Chats,
            "the second click closes settings"
        );
        assert!(
            app.open_chat.is_none(),
            "closing settings lands on the empty window a fresh start shows"
        );
    }

    #[test]
    fn the_pick_circle_sits_on_the_middle_of_the_row() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        let ctx = egui::Context::default();
        app.attach(&ctx);
        step_output(&mut app, &ctx, Vec::new(), 0.0, true);
        let chat = app.open_chat.clone().expect("a chat is open");
        app.selecting = Some(crate::model::Selecting {
            chat: chat.clone(),
            ids: Default::default(),
        });
        // One frame registers the rows, the next paints their circles.
        step_output(&mut app, &ctx, Vec::new(), 0.1, true);
        let row_id = |message: &str| egui::Id::new(("message-row", chat.as_str(), message));
        let message = app.conversations[&chat]
            .messages
            .iter()
            .find(|message| {
                ctx.data(|data| data.get_temp::<(Rect, bool)>(row_id(&message.id)).is_some())
            })
            .expect("a row registered its rect")
            .id
            .clone();
        let output = step_output(&mut app, &ctx, Vec::new(), 0.2, true);
        let (row, _) = ctx
            .data(|data| data.get_temp::<(Rect, bool)>(row_id(&message)))
            .expect("the row kept its rect");
        let inside: Vec<Pos2> = output
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Circle(circle) if (9.0..12.0).contains(&circle.radius) => {
                    Some(circle.center)
                }
                _ => None,
            })
            .filter(|center| row.contains(*center))
            .collect();
        assert_eq!(inside.len(), 1, "the row draws one pick circle");
        let center = inside[0];
        assert!(
            (center.y - row.center().y).abs() < 2.0,
            "the pick circle sits on the row's middle line, not at its top"
        );
        let margin = (center.x - row.left()).min(row.right() - center.x);
        assert!(
            margin >= 14.0,
            "the circle keeps a margin from the row's edge, inside its highlight"
        );
    }

    #[test]
    fn a_picked_row_keeps_its_highlight() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        let ctx = egui::Context::default();
        app.attach(&ctx);
        step_output(&mut app, &ctx, Vec::new(), 0.0, true);
        let chat = app.open_chat.clone().expect("a chat is open");
        app.selecting = Some(crate::model::Selecting {
            chat: chat.clone(),
            ids: Default::default(),
        });
        // One frame registers the rows, the next paints their highlight.
        step_output(&mut app, &ctx, Vec::new(), 0.1, true);
        let first = app.conversations[&chat]
            .messages
            .iter()
            .find(|message| {
                ctx.data(|data| {
                    data.get_temp::<(Rect, bool)>(egui::Id::new((
                        "message-row",
                        chat.as_str(),
                        message.id.as_str(),
                    )))
                })
                .is_some()
            })
            .expect("a row registered its rect")
            .id
            .clone();
        app.selecting = Some(crate::model::Selecting {
            chat: chat.clone(),
            ids: [first].into_iter().collect(),
        });
        let output = step_output(&mut app, &ctx, Vec::new(), 0.2, true);
        let expected = app.palette.accent.gamma_multiply(0.16);
        assert!(
            output.shapes.iter().any(|clipped| {
                matches!(&clipped.shape, egui::Shape::Rect(rect) if rect.fill == expected)
            }),
            "a picked row keeps a highlight of its own"
        );
    }

    #[test]
    fn the_forward_dialog_lists_several_chats() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        let ctx = egui::Context::default();
        app.attach(&ctx);
        step_output(&mut app, &ctx, Vec::new(), 0.0, true);
        let chat = app.open_chat.clone().expect("a chat is open");
        app.dialog = Some(crate::model::Dialog::Forward {
            chat,
            messages: vec!["m1".to_owned()],
        });
        step_output(&mut app, &ctx, Vec::new(), 0.1, true);
        let output = step_output(&mut app, &ctx, Vec::new(), 0.2, true);
        // Only the dialog's own column, so the chat list behind it does not count.
        let in_dialog: Vec<String> = output
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text) if text.pos.x > 400.0 && text.pos.x < 880.0 => {
                    Some(text.galley.text().to_owned())
                }
                _ => None,
            })
            .collect();
        let titles: Vec<String> = app.chats.iter().map(|chat| app.chat_title(chat)).collect();
        let shown = titles
            .iter()
            .filter(|title| in_dialog.iter().any(|text| text == *title))
            .count();
        assert!(
            shown >= 3,
            "the forward dialog lists several chats, not a single row"
        );
    }

    #[test]
    fn the_starred_list_shows_the_message_and_the_mark() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        let ctx = egui::Context::default();
        app.attach(&ctx);
        step_output(&mut app, &ctx, Vec::new(), 0.0, true);
        let chat = app.open_chat.clone().expect("a chat is open");
        let message = app.conversations[&chat]
            .messages
            .first()
            .expect("the chat has messages")
            .id
            .clone();
        // The star reaches the open chat: this is what the footer paints from.
        app.stars
            .insert(chat.clone(), [message.clone()].into_iter().collect());
        app.show_starred = true;
        app.starred = vec![crate::archive::Starred {
            chat: chat.clone(),
            id: message.clone(),
            starred_at: crate::util::now(),
            text: "See you at nine".to_owned(),
            from_me: false,
            sent_at: crate::util::now(),
        }];
        let output = step_output(&mut app, &ctx, Vec::new(), 0.1, true);
        let texts: Vec<String> = output
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
                _ => None,
            })
            .collect();
        assert!(
            texts.iter().any(|text| text == "Starred"),
            "the header names the panel"
        );
        assert!(
            texts.iter().any(|text| text.contains("See you at nine")),
            "the row shows the starred message itself"
        );
        assert!(
            app.stars[&chat].contains(&message),
            "the open chat knows which messages are starred"
        );
    }

    #[test]
    fn the_chat_list_chips_filter_unread() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        let ctx = egui::Context::default();
        app.attach(&ctx);
        let mut tour = Tour::new(None, None);
        for _ in 0..3 {
            frame(&mut app, &mut tour, &ctx, Vec::new());
        }
        for name in ["All", "Unread", "Favorites", "Groups"] {
            assert!(tour.labels.contains_key(name), "missing list chip {name}");
        }
        click(&mut app, &mut tour, &ctx, "Unread");
        assert_eq!(app.chat_list, crate::model::ChatListId::Unread);
        assert!(
            app.visible_chats().iter().all(|chat| chat.looks_unread()),
            "unread chip lists only unread chats"
        );
        click(&mut app, &mut tour, &ctx, "Favorites");
        assert!(app.visible_chats().iter().all(|chat| chat.favorite));
        app.chat_list = crate::model::ChatListId::Custom("l-work".into());
        assert!(
            app.visible_chats()
                .iter()
                .any(|chat| chat.name == "Rust Berlin")
        );
    }

    #[test]
    fn the_scheduled_list_shows_the_message() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        let ctx = egui::Context::default();
        app.attach(&ctx);
        step_output(&mut app, &ctx, Vec::new(), 0.0, true);
        let chat = app.open_chat.clone().expect("a chat is open");
        let now = crate::util::now();
        app.show_scheduled = true;
        app.scheduled = vec![crate::archive::Scheduled {
            id: "sched-1".to_owned(),
            chat,
            text: "See you at nine".to_owned(),
            kind: "daily".to_owned(),
            hour: 9,
            minute: 0,
            weekday: None,
            day_of_month: None,
            nth: None,
            next_at: now + 3600,
            state: "pending".to_owned(),
            message_id: None,
            last_error: None,
            created_at: now,
            last_fired_at: None,
        }];
        let output = step_output(&mut app, &ctx, Vec::new(), 0.1, true);
        let texts: Vec<String> = output
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
                _ => None,
            })
            .collect();
        assert!(
            texts.iter().any(|text| text == "Scheduled"),
            "the panel header names the list"
        );
        assert!(
            texts.iter().any(|text| text.contains("See you at nine")),
            "the row shows the scheduled message itself"
        );
        assert!(
            texts.iter().any(|text| text.contains("Every day")),
            "the row shows how it repeats"
        );
    }

    #[test]
    fn a_click_on_a_row_picks_its_message() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        let ctx = egui::Context::default();
        app.attach(&ctx);
        step(&mut app, &ctx, Vec::new(), 0.0);
        let chat = app.open_chat.clone().expect("a chat is open");
        app.selecting = Some(crate::model::Selecting {
            chat: chat.clone(),
            ids: Default::default(),
        });
        step(&mut app, &ctx, Vec::new(), 0.1);
        // A row that is on screen; the chat is scrolled to its end.
        let (first, row) = app.conversations[&chat]
            .messages
            .iter()
            .filter_map(|message| {
                let id = egui::Id::new(("message-row", chat.as_str(), message.id.as_str()));
                ctx.data(|data| data.get_temp::<(Rect, bool)>(id))
                    .map(|(rect, _)| (message.id.clone(), rect))
            })
            .find(|(_, rect)| {
                rect.height() > 0.0 && rect.center().y > 60.0 && rect.center().y < 700.0
            })
            .expect("a row is on screen");
        step(
            &mut app,
            &ctx,
            click_events(row.center(), PointerButton::Primary),
            0.2,
        );
        step(&mut app, &ctx, Vec::new(), 0.3);
        assert!(
            app.selecting
                .as_ref()
                .is_some_and(|selecting| selecting.ids.contains(&first)),
            "a click anywhere on the row picks its message"
        );
        step(
            &mut app,
            &ctx,
            click_events(row.center(), PointerButton::Primary),
            0.4,
        );
        step(&mut app, &ctx, Vec::new(), 0.5);
        assert!(
            app.selecting.is_none(),
            "unpicking the last message leaves the mode"
        );
    }

    #[test]
    fn a_right_click_away_from_a_bubble_opens_the_chat_menu() {
        let mut app = super::super::tests::app();
        prepare(&mut app);
        let ctx = egui::Context::default();
        app.attach(&ctx);
        step(&mut app, &ctx, Vec::new(), 0.0);
        let chat = app.open_chat.clone().expect("a chat is open");
        let (_, bubble) = app.conversations[&chat]
            .messages
            .iter()
            .filter(|message| message.from_me)
            .filter_map(|message| {
                let id = crate::ui::conversation::bubble_id(&chat, &message.id).with("rect");
                ctx.data(|data| data.get_temp::<Rect>(id))
                    .map(|rect| (message.id.clone(), rect))
            })
            .find(|(_, rect)| rect.center().y > 60.0 && rect.center().y < 700.0)
            .expect("a bubble of ours is on screen");
        // The space left of our own bubble belongs to the chat, not to a row.
        let pos = pos2(bubble.left() - 24.0, bubble.center().y);
        step(
            &mut app,
            &ctx,
            click_events(pos, PointerButton::Secondary),
            0.1,
        );
        step(&mut app, &ctx, Vec::new(), 0.2);
        assert!(
            egui::Popup::is_any_open(&ctx),
            "a right click away from a bubble opens the chat menu"
        );
    }

    #[test]
    fn next_wallpaper_from_the_chat_menu_advances_the_slot() {
        use crate::settings::ChatWallpaper;
        let mut app = super::super::tests::app();
        prepare(&mut app);
        let ctx = egui::Context::default();
        app.attach(&ctx);
        let mut tour = Tour::new(None, None);
        step(&mut app, &ctx, Vec::new(), 0.0);
        let chat = app.open_chat.clone().expect("a chat is open");
        let (_, bubble) = app.conversations[&chat]
            .messages
            .iter()
            .filter(|message| message.from_me)
            .filter_map(|message| {
                let id = crate::ui::conversation::bubble_id(&chat, &message.id).with("rect");
                ctx.data(|data| data.get_temp::<Rect>(id))
                    .map(|rect| (message.id.clone(), rect))
            })
            .find(|(_, rect)| rect.center().y > 60.0 && rect.center().y < 700.0)
            .expect("a bubble of ours is on screen");
        let pos = pos2(bubble.left() - 24.0, bubble.center().y);
        step(
            &mut app,
            &ctx,
            click_events(pos, PointerButton::Secondary),
            0.1,
        );
        frame(&mut app, &mut tour, &ctx, Vec::new());
        click(&mut app, &mut tour, &ctx, "Next wallpaper");
        assert_eq!(app.settings.chat_wallpaper, ChatWallpaper::Auto);
        assert_eq!(app.settings.chat_wallpaper_index, 1);
    }
}
