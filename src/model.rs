//! UI models for chats, messages, and view actions.
//!
//! The backend translates protocol types into these models, keeping protobufs
//! out of views and giving the archive a stable shape.

use std::path::PathBuf;
use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::i18n::{self, Key};

/// Chat JID string: `<phone>@s.whatsapp.net`, `<id>@g.us`, or `<id>@lid`.
pub type ChatId = String;

/// Sidebar chip that filters the chat list.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ChatListId {
    #[default]
    All,
    Unread,
    Favorites,
    Groups,
    Custom(String),
}

impl ChatListId {
    /// Key in `chat_list_pins`. All uses `chats.pinned` instead.
    pub fn pin_key(&self) -> Option<&str> {
        match self {
            Self::All => None,
            Self::Unread => Some("unread"),
            Self::Favorites => Some("favorites"),
            Self::Groups => Some("groups"),
            Self::Custom(id) => Some(id.as_str()),
        }
    }
}

/// A user-made chat list stored in the archive.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChatList {
    pub id: String,
    pub name: String,
    pub members: Vec<ChatId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatKind {
    Direct,
    Group,
    /// Read-only newsletter or broadcast list.
    Broadcast,
}

impl ChatKind {
    pub fn from_id(id: &str) -> Self {
        match id.rsplit('@').next() {
            Some("g.us") => Self::Group,
            Some("newsletter") | Some("broadcast") => Self::Broadcast,
            _ => Self::Direct,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Chat {
    pub id: ChatId,
    /// Best known address-book, push, or phone-number name.
    pub name: String,
    pub kind: ChatKind,
    /// Latest-message Unix timestamp used for ordering.
    pub last_activity: i64,
    pub unread: u32,
    /// Local reminder: show the empty unread dot with no pending count.
    pub marked_unread: bool,
    /// Local favorite flag. It is not a WhatsApp pin.
    pub favorite: bool,
    pub archived: bool,
    pub pinned: bool,
    /// Pin time in Unix milliseconds; zero for older archives with no ordering.
    pub pinned_at: i64,
    /// Mute end as Unix seconds; `Some(0)` means indefinite.
    pub muted_until: Option<i64>,
    /// Latest message shown in the chat list.
    pub last: Option<LastMessage>,
    /// Canonical group-member ids, empty until loaded.
    pub participants: Vec<String>,
    /// Whether this is an announcement group where we cannot post.
    pub read_only: bool,
    /// Disappearing-message duration in seconds, if enabled.
    pub ephemeral_expiration: Option<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LastMessage {
    pub from_me: bool,
    pub sender: String,
    /// Group-message sender.
    pub sender_name: Option<String>,
    pub summary: String,
    pub status: Delivery,
}

impl Chat {
    pub fn new(id: ChatId, name: String) -> Self {
        let kind = ChatKind::from_id(&id);
        Self {
            id,
            name,
            kind,
            last_activity: 0,
            unread: 0,
            marked_unread: false,
            favorite: false,
            archived: false,
            pinned: false,
            pinned_at: 0,
            muted_until: None,
            last: None,
            participants: Vec::new(),
            read_only: false,
            ephemeral_expiration: None,
        }
    }

    pub fn is_group(&self) -> bool {
        self.kind == ChatKind::Group
    }

    pub fn is_channel_id(id: &str) -> bool {
        id.rsplit('@').next() == Some("newsletter")
    }

    pub fn is_channel(&self) -> bool {
        Self::is_channel_id(&self.id)
    }

    /// Whether Leave is offered. An empty group member list means unknown.
    /// A channel stays leaveable until it is marked read-only after unfollow.
    pub fn can_leave(&self, me: Option<&str>) -> bool {
        if self.is_channel() {
            return !self.read_only;
        }
        if !self.is_group() {
            return false;
        }
        match me {
            None => true,
            Some(_) if self.participants.is_empty() => true,
            Some(me) => self.participants.iter().any(|id| id == me),
        }
    }

    /// Counted unread or a local empty-dot reminder.
    pub fn looks_unread(&self) -> bool {
        self.unread > 0 || self.marked_unread
    }

    pub fn muted(&self, now: i64) -> bool {
        matches!(self.muted_until, Some(0)) || self.muted_until.is_some_and(|until| until > now)
    }

    /// Direct-chat phone number as digits.
    pub fn phone(&self) -> Option<&str> {
        phone_of(&self.id)
    }
}

/// Extracts digits from a `<phone>@s.whatsapp.net` id.
pub fn phone_of(id: &str) -> Option<&str> {
    let (user, server) = id.split_once('@')?;
    (server == "s.whatsapp.net" && user.chars().all(|c| c.is_ascii_digit())).then_some(user)
}

/// Outgoing-message delivery state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Delivery {
    /// Incoming message without outgoing receipts.
    #[default]
    None,
    /// Sent to the backend but not acknowledged by the server.
    Pending,
    Sent,
    Delivered,
    Read,
    Played,
    Failed,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// WhatsApp message id, unique within a chat.
    pub id: String,
    pub chat: ChatId,
    /// Sender JID, including our own for outgoing messages.
    pub sender: String,
    /// Group sender's push name at receipt time.
    pub sender_name: Option<String>,
    pub from_me: bool,
    /// Unix seconds.
    pub timestamp: i64,
    pub content: Content,
    pub status: Delivery,
    /// First delivered-receipt Unix timestamp for outgoing messages.
    #[serde(default)]
    pub delivered_at: Option<i64>,
    /// First read or played receipt Unix timestamp.
    #[serde(default)]
    pub read_at: Option<i64>,
    pub quoted: Option<Quoted>,
    pub reactions: Vec<Reaction>,
    pub edited: bool,
    /// Mentions in the text or caption.
    #[serde(default)]
    pub mentions: Vec<MentionRef>,
    /// Forwarded from another chat.
    #[serde(default)]
    pub forwarded: bool,
    /// JPEG preview sent with an attachment or link.
    #[serde(default)]
    pub thumbnail: Option<Vec<u8>>,
}

/// Raw WhatsApp mention token and its canonical id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MentionRef {
    pub user: String,
    pub id: String,
}

/// Link metadata attached by WhatsApp.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkPreview {
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
}

impl Message {
    /// One-line summary used in chat rows and quotes.
    pub fn summary(&self) -> String {
        self.content.summary()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quoted {
    pub id: String,
    pub sender: String,
    pub sender_name: Option<String>,
    pub summary: String,
    /// Mentions in quoted text.
    #[serde(default)]
    pub mentions: Vec<MentionRef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reaction {
    pub sender: String,
    pub from_me: bool,
    pub emoji: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Content {
    Text {
        text: String,
        #[serde(default)]
        preview: Option<LinkPreview>,
    },
    Image {
        caption: Option<String>,
        media: Media,
    },
    Video {
        caption: Option<String>,
        media: Media,
        seconds: Option<u32>,
        gif: bool,
    },
    Audio {
        media: Media,
        seconds: Option<u32>,
        voice_note: bool,
        /// Sender-provided 64-bar voice waveform.
        #[serde(default)]
        waveform: Vec<u8>,
    },
    Document {
        media: Media,
        file_name: String,
        caption: Option<String>,
        pages: Option<u32>,
    },
    Sticker {
        media: Media,
        animated: bool,
    },
    Location {
        latitude: f64,
        longitude: f64,
        name: Option<String>,
        address: Option<String>,
    },
    Contact {
        display_name: String,
        vcard: String,
    },
    Poll {
        question: String,
        options: Vec<String>,
        #[serde(default)]
        state: PollState,
    },
    /// "This message was deleted."
    Revoked,
    /// Unsupported content with a user-facing description.
    Unsupported {
        what: String,
    },
}

/// Poll information safe to send to the interface; encryption keys stay in the worker.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PollState {
    pub selectable: usize,
    pub counts: Vec<usize>,
    pub selected: Vec<usize>,
    pub voters: usize,
    pub can_vote: bool,
    pub history_complete: bool,
    pub refresh_needed: bool,
    pub refreshing: bool,
    pub refresh_failed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PollDraft {
    pub question: String,
    pub options: Vec<String>,
    pub multiple: bool,
}

impl Default for PollDraft {
    fn default() -> Self {
        Self {
            question: String::new(),
            options: vec![String::new(); 2],
            multiple: true,
        }
    }
}

impl PollDraft {
    pub fn validated(&self) -> Result<Self, &'static str> {
        let question = self.question.trim().to_owned();
        let options: Vec<String> = self
            .options
            .iter()
            .map(|option| option.trim().to_owned())
            .collect();
        if question.is_empty() || question.chars().count() > 255 {
            return Err(i18n::t(Key::PollErrorQuestion));
        }
        if !(2..=12).contains(&options.len())
            || options
                .iter()
                .any(|option| option.is_empty() || option.chars().count() > 100)
        {
            return Err(i18n::t(Key::PollErrorAnswers));
        }
        let mut unique = std::collections::HashSet::new();
        if options.iter().any(|option| !unique.insert(option)) {
            return Err(i18n::t(Key::PollErrorDuplicates));
        }
        Ok(Self {
            question,
            options,
            multiple: self.multiple,
        })
    }

    pub fn selectable(&self) -> usize {
        if self.multiple { self.options.len() } else { 1 }
    }
}

impl Content {
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text {
            text: text.into(),
            preview: None,
        }
    }

    pub fn summary(&self) -> String {
        match self {
            Self::Text { text, .. } => text.lines().next().unwrap_or_default().to_owned(),
            Self::Image { caption, .. } => with_caption(i18n::t(Key::KindPhoto), caption),
            Self::Video { caption, gif, .. } => with_caption(
                if *gif {
                    i18n::t(Key::KindGif)
                } else {
                    i18n::t(Key::KindVideo)
                },
                caption,
            ),
            Self::Audio {
                voice_note,
                seconds,
                ..
            } => {
                let label = if *voice_note {
                    i18n::t(Key::KindVoice)
                } else {
                    i18n::t(Key::KindAudio)
                };
                match seconds {
                    Some(seconds) => format!("{label} ({})", crate::util::duration(*seconds)),
                    None => label.to_owned(),
                }
            }
            Self::Document { file_name, .. } => {
                i18n::f(Key::KindDocumentWith, &[("name", file_name)])
            }
            Self::Sticker { .. } => i18n::t(Key::KindSticker).to_owned(),
            Self::Location { name, .. } => match name {
                Some(name) => i18n::f(Key::KindLocationWith, &[("name", name)]),
                None => i18n::t(Key::KindLocation).to_owned(),
            },
            Self::Contact { display_name, .. } => {
                i18n::f(Key::KindContactWith, &[("name", display_name)])
            }
            Self::Poll { question, .. } => i18n::f(Key::KindPollWith, &[("question", question)]),
            Self::Revoked => i18n::t(Key::ChatMessageDeleted).to_owned(),
            Self::Unsupported { what } => i18n::f(Key::ChatUnsupportedMessage, &[("what", what)]),
        }
    }

    /// The message's own words, whole, for a place that draws it as the bubble
    /// the chat shows. [`summary`](Self::summary) cuts at the first line.
    pub fn body(&self) -> String {
        match self {
            Self::Text { text, .. } => text.clone(),
            Self::Image { caption, .. } => with_whole_caption(i18n::t(Key::KindPhoto), caption),
            Self::Video { caption, gif, .. } => with_whole_caption(
                if *gif {
                    i18n::t(Key::KindGif)
                } else {
                    i18n::t(Key::KindVideo)
                },
                caption,
            ),
            other => other.summary(),
        }
    }

    pub fn media(&self) -> Option<&Media> {
        match self {
            Self::Image { media, .. }
            | Self::Video { media, .. }
            | Self::Audio { media, .. }
            | Self::Document { media, .. }
            | Self::Sticker { media, .. } => Some(media),
            _ => None,
        }
    }

    pub fn media_mut(&mut self) -> Option<&mut Media> {
        match self {
            Self::Image { media, .. }
            | Self::Video { media, .. }
            | Self::Audio { media, .. }
            | Self::Document { media, .. }
            | Self::Sticker { media, .. } => Some(media),
            _ => None,
        }
    }
}

fn with_caption(label: &str, caption: &Option<String>) -> String {
    match caption
        .as_deref()
        .and_then(|caption| caption.lines().next())
    {
        Some(caption) if !caption.is_empty() => format!("{label}: {caption}"),
        _ => label.to_owned(),
    }
}

/// The caption as written, with its own line breaks, for a bubble.
fn with_whole_caption(label: &str, caption: &Option<String>) -> String {
    match caption.as_deref().filter(|caption| !caption.is_empty()) {
        Some(caption) => format!("{label}: {caption}"),
        None => label.to_owned(),
    }
}

/// Attachment metadata, download state, and optional local file. Download keys
/// remain in the archive's raw message.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Media {
    pub mime: String,
    pub size: u64,
    pub width: Option<u32>,
    pub height: Option<u32>,
    /// Decrypted downloaded file.
    #[serde(default)]
    pub path: Option<PathBuf>,
    /// First failed download (Unix seconds). Prefetch stops after 30 days.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_from: Option<i64>,
    /// Next prefetch attempt (Unix seconds).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_at: Option<i64>,
    #[serde(default, skip_serializing_if = "u32_is_zero")]
    pub retry_fails: u32,
    /// Non-persisted download state.
    #[serde(skip)]
    pub state: MediaState,
}

fn u32_is_zero(value: &u32) -> bool {
    *value == 0
}

/// Downloaded-attachment counts and sizes from the archive, for Settings.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StorageStats {
    pub messages: u64,
    pub images: u64,
    pub image_bytes: u64,
    pub videos: u64,
    pub video_bytes: u64,
    pub stickers_gifs: u64,
    pub sticker_gif_bytes: u64,
    pub other: u64,
    pub other_bytes: u64,
}

impl StorageStats {
    pub fn bytes_total(self) -> u64 {
        self.image_bytes
            .saturating_add(self.video_bytes)
            .saturating_add(self.sticker_gif_bytes)
            .saturating_add(self.other_bytes)
    }

    /// Filled bar width. Empty when `total` is 0.
    pub fn bar_width(bytes: u64, total: u64, track: f32) -> f32 {
        if total == 0 || track <= 0.0 {
            0.0
        } else {
            track * (bytes as f32 / total as f32)
        }
    }
}

/// Prefetch gives up this many seconds after the first failed download.
pub const MEDIA_RETRY_TTL_SECS: i64 = 30 * 24 * 60 * 60;
pub const MEDIA_STILL_TRYING: &str =
    "We are still trying to get this file automatically. Click to retry manually.";
pub const MEDIA_NO_LONGER: &str = "No longer available on WhatsApp's servers";

/// Backoff after `fails` attempts: 30 s doubling to 15 min, then 1 h.
pub fn media_retry_delay_secs(fails: u32) -> i64 {
    match fails {
        0 | 1 => 30,
        2 => 60,
        3 => 120,
        4 => 240,
        5 => 480,
        6 => 900,
        _ => 3600,
    }
}

pub fn media_retry_notice(retry_from: i64, now: i64) -> &'static str {
    if now.saturating_sub(retry_from) >= MEDIA_RETRY_TTL_SECS {
        MEDIA_NO_LONGER
    } else {
        MEDIA_STILL_TRYING
    }
}

impl Media {
    pub fn clear_retry(&mut self) {
        self.retry_from = None;
        self.retry_at = None;
        self.retry_fails = 0;
    }

    pub fn schedule_retry(&mut self, now: i64, reset: bool) {
        let from = if reset {
            now
        } else {
            self.retry_from.unwrap_or(now)
        };
        let fails = if reset {
            1
        } else {
            self.retry_fails.saturating_add(1)
        };
        self.retry_from = Some(from);
        self.retry_fails = fails;
        self.retry_at = Some(now.saturating_add(media_retry_delay_secs(fails)));
    }

    pub fn retry_given_up(&self, now: i64) -> bool {
        self.retry_from
            .is_some_and(|from| now.saturating_sub(from) >= MEDIA_RETRY_TTL_SECS)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum MediaState {
    #[default]
    Idle,
    Downloading,
    Failed(String),
}

/// Contact names from app-state sync and message push names.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Contact {
    pub id: String,
    pub full_name: Option<String>,
    pub push_name: Option<String>,
}

impl Contact {
    pub fn display_name(&self) -> Option<&str> {
        self.full_name
            .as_deref()
            .filter(|name| !name.is_empty())
            .or(self.push_name.as_deref().filter(|name| !name.is_empty()))
    }

    /// WhatsApp display name: address-book name or `~`-prefixed push name.
    pub fn label(&self) -> Option<String> {
        if let Some(name) = self.full_name.as_deref().filter(|name| !name.is_empty()) {
            return Some(name.to_owned());
        }
        self.push_name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| format!("~{name}"))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Page {
    Chats,
    Settings,
}

/// A pinned chat being held to move it, and where it would land.
#[derive(Clone, Debug)]
pub struct PinDrag {
    pub chat: ChatId,
    /// The row the gesture started on, and the slot under the pointer now.
    pub from: usize,
    pub to: usize,
    /// The frame time the press happened, for the hold threshold.
    pub since: f64,
    /// Set once the hold passed the threshold: the rows show their handles.
    pub active: bool,
    /// Where the pointer sat inside the row when it was pressed, so the held
    /// row keeps that spot under the cursor.
    pub grab_y: f32,
}

/// The tabs of the picker above the composer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PickerTab {
    Emoji,
    Gifs,
    Stickers,
}

/// Imported sticker pack stored as a named WebP directory.
#[derive(Clone, Debug, PartialEq)]
pub struct StickerPack {
    pub name: String,
    pub dir: PathBuf,
    pub stickers: Vec<PathBuf>,
}

/// GIF search failure.
#[derive(Clone, Debug, PartialEq)]
pub struct GifError {
    pub message: String,
    /// GIPHY rejected the API key.
    pub bad_key: bool,
}

/// A GIF found through GIPHY.
#[derive(Clone, Debug, PartialEq)]
pub struct Gif {
    pub id: String,
    /// Downloaded still-frame path.
    pub still: Option<PathBuf>,
    pub mp4: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RightPane {
    /// Search messages in the open chat.
    Search,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Dialog {
    Shortcuts,
    About,
    ConfirmUnlink,
    /// Leave a group, optionally archiving the chat.
    ConfirmLeaveGroup(ChatId),
    /// Phone number used for pairing-code linking.
    PairWithPhone,
    /// Manually entered number for messaging or saving a contact.
    NewContact,
    ChatInfo(ChatId),
    /// Chooses a destination for archived messages.
    Forward {
        chat: ChatId,
        messages: Vec<String>,
    },
    CreatePoll(ChatId),
    /// Picks when the composer's text is sent, and whether it repeats.
    ScheduleMessage(ChatId),
    /// Creates a chat list, or edits an existing one.
    EditChatList {
        id: Option<String>,
    },
    /// Picks 1:1 chats excluded from an account privacy category.
    PrivacyExcept {
        kind: crate::privacy::PrivacyKind,
    },
}

/// Messages picked in one chat while the selection bar is up.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Selecting {
    pub chat: ChatId,
    pub ids: std::collections::HashSet<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ToastKind {
    Info,
    Error,
}

#[derive(Clone, Debug)]
pub struct Toast {
    pub message: String,
    pub kind: ToastKind,
    pub created: Instant,
    /// Progress toasts share a key: a newer one replaces the older, and the
    /// key keeps it alive while its batch runs.
    pub key: Option<&'static str>,
}

/// Actions queued by views and applied after drawing.
#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Open(Page),
    /// Opens settings, or closes them when they are already showing.
    ToggleSettings,
    OpenChat(ChatId),
    /// Creates and opens a chat for a contact without one.
    StartChat {
        id: ChatId,
        name: String,
    },
    /// Opens a chat at a message search result.
    OpenMessage {
        chat: ChatId,
        message: String,
    },
    CloseChat,
    SendText {
        chat: ChatId,
        text: String,
        /// Quoted message id.
        quoting: Option<String>,
    },
    CreatePoll {
        chat: ChatId,
        draft: PollDraft,
    },
    RefreshPoll {
        chat: ChatId,
        message: String,
    },
    VotePoll {
        chat: ChatId,
        message: String,
        choices: Vec<usize>,
    },
    /// Updates our typing state in a chat.
    Composing {
        chat: ChatId,
        composing: bool,
    },
    MarkRead(ChatId),
    /// Local empty unread mark; does not invent a pending count.
    MarkUnread(ChatId),
    LoadOlder(ChatId),
    /// Requests messages older than the local archive.
    FetchOlder(ChatId),
    Download {
        chat: ChatId,
        message: String,
    },
    /// Plays or pauses a downloaded voice or audio message.
    PlayVoice {
        message: String,
        path: PathBuf,
    },
    /// Seeks to a fraction from 0 to 1 and starts playback.
    SeekVoice {
        message: String,
        path: PathBuf,
        fraction: f32,
    },
    /// Cycles voice playback speed between 1x, 1.5x, and 2x.
    CycleVoiceSpeed,
    /// Starts, cancels, or sends a voice recording.
    StartRecording,
    CancelRecording,
    SendRecording,
    OpenFile(PathBuf),
    /// Opens a downloaded chat photo in the in-app viewer.
    ViewImage {
        chat: ChatId,
        message: String,
    },
    CloseImageViewer,
    /// Steps to another downloaded photo in the same chat. Does not wrap.
    StepImage(i8),
    OpenUrl(String),
    CopyText(String),
    /// Starts a reply to a message in the open chat.
    Reply(String),
    CancelReply,
    /// Forwards archived messages to another chat.
    Forward {
        from_chat: ChatId,
        messages: Vec<String>,
        to_chat: ChatId,
    },
    /// Enters multi-message selection, picking this message when there is one.
    StartSelecting {
        message: Option<String>,
    },
    /// Sends the composer's text now, without waiting for Enter.
    ScheduleText {
        chat: ChatId,
        text: String,
        kind: String,
        hour: i8,
        minute: i8,
        weekday: Option<i8>,
        day_of_month: Option<i8>,
        nth: Option<i8>,
        next_at: i64,
    },
    /// Switches the left panel to the scheduled list, or back to the chats.
    ToggleScheduled,
    /// Shows or hides the starred messages in the left panel.
    ToggleStarred,
    /// Writes the new order of the pinned chats, top first.
    ReorderPinned(Vec<ChatId>),
    /// Removes a scheduled message.
    CancelScheduled {
        id: String,
    },
    /// Adds or removes a message from the open selection.
    ToggleSelected {
        message: String,
    },
    /// Leaves the selection and drops it.
    ClearSelection,
    /// Picks every message in the open chat.
    SelectAllMessages,
    /// Saves the picked messages' attachments to the Downloads folder.
    DownloadSelected {
        chat: ChatId,
        messages: Vec<String>,
    },
    /// Stars or unstars the picked messages.
    StarSelected {
        chat: ChatId,
        messages: Vec<String>,
        starred: bool,
    },
    /// Loads an outgoing message into the composer for editing.
    Edit(String),
    CancelEdit,
    /// Revokes an outgoing message for everyone.
    DeleteForEveryone(String),
    /// Deletes a message locally.
    DeleteForMe(String),
    /// Opens the attachment picker for the current chat.
    Attach,
    SendFiles(Vec<PathBuf>),
    /// Clipboard image as straight-alpha RGBA.
    PasteImage {
        width: usize,
        height: usize,
        rgba: Vec<u8>,
    },
    /// Toggles a picker tab.
    TogglePicker(PickerTab),
    ClosePicker,
    /// Opens the full emoji picker to react to a message.
    OpenReactionPicker {
        chat: ChatId,
        message: String,
    },
    /// Inserts an emoji at the composer cursor.
    InsertEmoji(String),
    /// Replaces an active `:query` with its selected emoji.
    InsertEmojiCompletion {
        emoji: String,
        start: usize,
        end: usize,
    },
    CloseEmojiSuggestions,
    /// Replaces the active `@` query with a selected group member.
    InsertMention {
        id: String,
        name: String,
        start: usize,
        end: usize,
    },
    CloseMentions,
    SendSticker(PathBuf),
    /// Saves a sticker for the picker.
    SaveSticker(PathBuf),
    /// Removes a saved sticker.
    ForgetSticker(PathBuf),
    /// Imports a sticker pack from a signal.art link.
    ImportStickerUrl(String),
    /// Selects and imports a .wastickers or zip file.
    PickStickerArchive,
    /// Deletes an imported pack directory.
    DeleteStickerPack(PathBuf),
    /// Opens the prefilled contact-name editor.
    EditContact(String),
    /// Saves a contact through WhatsApp contact sync. `first` is the short
    /// display name and `last` completes the full name.
    SaveContact {
        id: String,
        first: String,
        last: String,
    },
    /// Checks a number, optionally saves it, and opens its chat.
    NewContact {
        phone: String,
        first: String,
        last: String,
    },
    /// Searches GIFs or lists trending results for an empty query.
    SearchGifs(String),
    SendGif(Gif),
    React {
        chat: ChatId,
        message: String,
        emoji: String,
    },
    SetArchived(ChatId, bool),
    /// Leaves a group or channel. `archive` also hides the chat in Archived.
    LeaveGroup {
        chat: ChatId,
        archive: bool,
    },
    SetPinned(ChatId, bool),
    SetFavorite(ChatId, bool),
    SetChatList(ChatListId),
    SaveChatList {
        id: Option<String>,
        name: String,
        members: Vec<ChatId>,
    },
    DeleteChatList(String),
    SetListPinned {
        list: String,
        chat: ChatId,
        pinned: bool,
    },
    ReorderListPinned {
        list: String,
        order: Vec<ChatId>,
    },
    ShowDialog(Dialog),
    CloseDialog,
    ToggleSidebar,
    /// Opens or focuses the right inspector. Header Search toggles it.
    OpenRightPane(RightPane),
    CloseRightPane,
    FocusSearch,
    FocusComposer,
    HideShortcutHints,
    ScrollToBottom,
    /// Scrolls the open chat to a message.
    ScrollTo(String),
    /// Updates chat-list search text.
    Search(String),
    /// Updates in-chat search text in the right pane.
    SearchInChat(String),
    /// Restricts in-chat search to a local calendar day.
    SetChatSearchDay(Option<jiff::civil::Date>),
    InstallUpdate,
    SetTheme(crate::settings::ThemeChoice),
    SetCustomTheme(String),
    SetHistoryPrefetch(crate::settings::HistoryPrefetch),
    SetChatWallpaper {
        choice: crate::settings::ChatWallpaper,
        index: u8,
    },
    NextWallpaper,
    ReloadThemes,
    OpenThemesFolder,
    SettingsChanged,
    /// Writes one WhatsApp account privacy category.
    SetAccountPrivacy {
        kind: crate::privacy::PrivacyKind,
        choice: crate::privacy::PrivacyChoice,
    },
    /// Saves the Except list for one account privacy category.
    SavePrivacyExcept {
        kind: crate::privacy::PrivacyKind,
        ids: Vec<ChatId>,
    },
    ZoomBy(f32),
    ResetZoom,
    /// Requests a pairing code for a phone number.
    PairWithPhone(String),
    /// Unlinks the device remotely and locally.
    Unlink,
    Reconnect,
    Quit,
    /// Shows the window, creating it when running headless.
    ShowWindow,
    /// Closes the window while keeping the app in the tray.
    HideWindow,
    /// Applies the configured close-button behavior.
    CloseWindow,
    /// Mutes until Unix time, indefinitely with `Some(0)`, or unmutes with `None`.
    SetMuted(ChatId, Option<i64>),
    /// Sends pending attachments with the composer text as caption.
    SendPending {
        chat: ChatId,
        caption: String,
    },
    /// Removes one pending attachment.
    RemovePending(usize),
    /// Removes all pending attachments.
    ClearPending,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn looks_unread_covers_counts_and_the_empty_dot() {
        let mut chat = Chat::new("1@s.whatsapp.net".into(), "A".into());
        assert!(!chat.looks_unread());
        chat.marked_unread = true;
        assert!(chat.looks_unread());
        chat.marked_unread = false;
        chat.unread = 2;
        assert!(chat.looks_unread());
    }

    #[test]
    fn polls_validate_trimmed_questions_and_distinct_bounded_answers() {
        let mut draft = PollDraft {
            question: " Lunch? ".into(),
            options: vec![" Pizza ".into(), "Pasta".into()],
            multiple: false,
        };
        let valid = draft.validated().unwrap();
        assert_eq!(valid.question, "Lunch?");
        assert_eq!(valid.options, ["Pizza", "Pasta"]);
        assert_eq!(valid.selectable(), 1);
        draft.options[1] = "Pizza".into();
        assert!(draft.validated().is_err());
        draft.options[1].clear();
        assert!(draft.validated().is_err());
        draft.options = (0..13).map(|i| format!("Answer {i}")).collect();
        assert!(draft.validated().is_err());
        draft.options.pop();
        draft.multiple = true;
        assert_eq!(draft.validated().unwrap().selectable(), 12);
        draft.question = "🍕".repeat(256);
        assert!(draft.validated().is_err());
    }

    fn media() -> Media {
        Media {
            mime: "image/jpeg".into(),
            size: 1,
            ..Default::default()
        }
    }

    #[test]
    fn kinds_come_from_the_server_part() {
        assert_eq!(ChatKind::from_id("1@s.whatsapp.net"), ChatKind::Direct);
        assert_eq!(ChatKind::from_id("1@lid"), ChatKind::Direct);
        assert_eq!(ChatKind::from_id("1-2@g.us"), ChatKind::Group);
        assert_eq!(ChatKind::from_id("1@newsletter"), ChatKind::Broadcast);
    }

    #[test]
    fn a_group_can_be_left_until_we_are_no_longer_a_member() {
        let me = "me@s.whatsapp.net";
        let mut chat = Chat::new("1-2@g.us".into(), "Rust".into());
        assert!(
            chat.can_leave(Some(me)),
            "unknown membership still offers leave"
        );
        chat.participants = vec![me.into(), "other@s.whatsapp.net".into()];
        assert!(chat.can_leave(Some(me)));
        chat.participants.retain(|id| id != me);
        assert!(!chat.can_leave(Some(me)));
        assert!(!Chat::new("1@s.whatsapp.net".into(), "Ada".into()).can_leave(Some(me)));
        assert!(!Chat::new("1@broadcast".into(), "List".into()).can_leave(Some(me)));
    }

    #[test]
    fn a_channel_can_be_left_until_it_is_read_only() {
        let me = "me@s.whatsapp.net";
        let mut chat = Chat::new("1@newsletter".into(), "News".into());
        assert!(chat.is_channel());
        assert!(chat.can_leave(Some(me)));
        chat.read_only = true;
        assert!(!chat.can_leave(Some(me)));
    }

    #[test]
    fn summaries_read_like_whatsapp() {
        assert_eq!(Content::text("hi\nthere").summary(), "hi");
        assert_eq!(
            Content::Image {
                caption: Some("look".into()),
                media: media()
            }
            .summary(),
            "Photo: look"
        );
        assert_eq!(
            Content::Image {
                caption: None,
                media: media()
            }
            .summary(),
            i18n::t(Key::KindPhoto)
        );
        assert_eq!(
            Content::Audio {
                media: media(),
                seconds: Some(65),
                voice_note: true,
                waveform: Vec::new()
            }
            .summary(),
            "Voice message (1:05)"
        );
    }

    #[test]
    fn phones_only_come_from_phone_ids() {
        assert_eq!(
            phone_of("393331234567@s.whatsapp.net"),
            Some("393331234567")
        );
        assert_eq!(phone_of("12345@lid"), None);
        assert_eq!(phone_of("1-2@g.us"), None);
    }

    #[test]
    fn labels_mark_names_people_chose_themselves() {
        let saved = Contact {
            id: "1".into(),
            full_name: Some("Ada".into()),
            push_name: Some("ada l".into()),
        };
        assert_eq!(saved.label().as_deref(), Some("Ada"));
        let stranger = Contact {
            id: "2".into(),
            full_name: None,
            push_name: Some("Bob".into()),
        };
        assert_eq!(stranger.label().as_deref(), Some("~Bob"));
        assert_eq!(Contact::default().label(), None);
    }

    #[test]
    fn old_text_content_still_parses() {
        let old: Content = serde_json::from_str(r#"{"kind":"text","text":"hi"}"#).expect("parses");
        assert_eq!(old, Content::text("hi"));
    }

    #[test]
    fn content_survives_json() {
        let content = Content::Document {
            media: media(),
            file_name: "a.pdf".into(),
            caption: None,
            pages: Some(3),
        };
        let json = serde_json::to_string(&content).expect("serializes");
        let back: Content = serde_json::from_str(&json).expect("parses");
        assert_eq!(back, content);
    }

    #[test]
    fn media_retry_backs_off_then_gives_up_after_thirty_days() {
        assert_eq!(media_retry_delay_secs(1), 30);
        assert_eq!(media_retry_delay_secs(6), 900);
        assert_eq!(media_retry_delay_secs(7), 3600);
        assert_eq!(media_retry_notice(10, 10), MEDIA_STILL_TRYING);
        assert_eq!(
            media_retry_notice(10, 10 + MEDIA_RETRY_TTL_SECS),
            MEDIA_NO_LONGER
        );
        let mut media = media();
        media.schedule_retry(1_000, false);
        assert_eq!(media.retry_from, Some(1_000));
        assert_eq!(media.retry_fails, 1);
        assert_eq!(media.retry_at, Some(1_030));
        media.schedule_retry(1_040, false);
        assert_eq!(media.retry_from, Some(1_000));
        assert_eq!(media.retry_fails, 2);
        assert_eq!(media.retry_at, Some(1_100));
        media.schedule_retry(2_000, true);
        assert_eq!(media.retry_from, Some(2_000));
        assert_eq!(media.retry_fails, 1);
        media.clear_retry();
        assert!(media.retry_from.is_none());
        assert_eq!(media.retry_fails, 0);
    }

    #[test]
    fn storage_bar_is_empty_when_total_is_zero() {
        assert_eq!(StorageStats::bar_width(0, 0, 100.0), 0.0);
        assert_eq!(StorageStats::bar_width(50, 0, 100.0), 0.0);
        assert_eq!(StorageStats::bar_width(50, 100, 200.0), 100.0);
    }
}
