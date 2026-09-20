//! User preferences stored in JSON.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::i18n::{self, Key, Language};

use crate::i18n::{self, Key, Language};

use crate::i18n::{self, Key, Language};

use crate::i18n::{self, Key, Language};

use crate::i18n::{self, Key, Language};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeChoice {
    #[default]
    Dark,
    Light,
    System,
}

impl ThemeChoice {
    pub const ALL: [ThemeChoice; 3] = [Self::System, Self::Light, Self::Dark];

    pub fn label(self) -> &'static str {
        match self {
            Self::Dark => i18n::t(Key::SettingsThemeDark),
            Self::Light => i18n::t(Key::SettingsThemeLight),
            Self::System => i18n::t(Key::SettingsThemeSystem),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryPrefetch {
    Off,
    Focused,
    #[default]
    RecentAndPinned,
}

impl HistoryPrefetch {
    pub const ALL: [HistoryPrefetch; 3] = [Self::Off, Self::Focused, Self::RecentAndPinned];

    pub fn label(self) -> &'static str {
        match self {
            Self::Off => i18n::t(Key::SettingsHistoryOff),
            Self::Focused => i18n::t(Key::SettingsHistoryCurrent),
            Self::RecentAndPinned => i18n::t(Key::SettingsHistoryRecent),
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Self::Off => i18n::t(Key::SettingsHistoryOffHint),
            Self::Focused => i18n::t(Key::SettingsHistoryCurrentHint),
            Self::RecentAndPinned => i18n::t(Key::SettingsHistoryRecentHint),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum WallpaperFamily {
    Black,
    Gray,
    Green,
    Red,
    #[default]
    White,
}

impl WallpaperFamily {
    pub const ALL: [WallpaperFamily; 5] =
        [Self::Black, Self::Gray, Self::Green, Self::Red, Self::White];

    pub fn label(self) -> &'static str {
        match self {
            Self::Black => i18n::t(Key::SettingsWallpaperBlack),
            Self::Gray => i18n::t(Key::SettingsWallpaperGray),
            Self::Green => i18n::t(Key::SettingsWallpaperGreen),
            Self::Red => i18n::t(Key::SettingsWallpaperRed),
            Self::White => i18n::t(Key::SettingsWallpaperWhite),
        }
    }

    pub fn as_choice(self) -> ChatWallpaper {
        match self {
            Self::Black => ChatWallpaper::Black,
            Self::Gray => ChatWallpaper::Gray,
            Self::Green => ChatWallpaper::Green,
            Self::Red => ChatWallpaper::Red,
            Self::White => ChatWallpaper::White,
        }
    }
}

/// Slots per family (`01.png` … `03.png`).
pub const WALLPAPER_SLOTS: u8 = 3;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatWallpaper {
    #[default]
    Auto,
    Black,
    Gray,
    Green,
    Red,
    White,
}

impl ChatWallpaper {
    pub fn family(self) -> Option<WallpaperFamily> {
        match self {
            Self::Auto => None,
            Self::Black => Some(WallpaperFamily::Black),
            Self::Gray => Some(WallpaperFamily::Gray),
            Self::Green => Some(WallpaperFamily::Green),
            Self::Red => Some(WallpaperFamily::Red),
            Self::White => Some(WallpaperFamily::White),
        }
    }

    pub fn combo_label(self, index: u8) -> String {
        match self.family() {
            None => i18n::t(Key::SettingsWallpaperAuto).to_owned(),
            Some(family) => format!("{} {}", family.label(), index.min(WALLPAPER_SLOTS - 1) + 1),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub theme: ThemeChoice,
    /// Interface language. `System` follows the operating system.
    #[serde(default)]
    pub language: Language,
    /// Interface language. `System` follows the operating system.
    #[serde(default)]
    pub language: Language,
    /// Interface language. `System` follows the operating system.
    #[serde(default)]
    pub language: Language,
    /// Interface language. `System` follows the operating system.
    #[serde(default)]
    pub language: Language,
    /// Interface language. `System` follows the operating system.
    #[serde(default)]
    pub language: Language,
    /// Filename of the selected local JSON palette.
    pub custom_theme: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::theme::custom::read_cached_theme",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_theme_cache: Option<crate::theme::custom::CustomTheme>,
    #[serde(
        default,
        deserialize_with = "crate::theme::custom::read_cached_theme",
        skip_serializing_if = "Option::is_none"
    )]
    pub system_theme_cache: Option<crate::theme::custom::CustomTheme>,
    /// egui zoom factor.
    pub zoom: f32,
    pub sidebar_width: f32,
    /// Width of the right inspector pane.
    #[serde(default = "default_inspector_width")]
    pub inspector_width: f32,
    /// Whether Enter sends and Shift+Enter adds a line. Off swaps them.
    pub enter_sends: bool,
    /// Send read receipts, subject to the account privacy setting.
    pub send_read_receipts: bool,
    /// Send typing state while composing.
    pub send_typing: bool,
    /// Download attachments when they enter view instead of on click.
    #[serde(alias = "auto_download_images")]
    pub auto_download: bool,
    /// Show sender avatars outside groups too.
    pub show_sender_pictures: bool,
    /// Last open chat, restored at startup.
    pub last_chat: Option<String>,
    pub show_shortcut_hints: bool,
    /// Recently used emoji, newest first.
    pub recent_emoji: Vec<String>,
    /// User GIPHY API key. Empty uses the optional built-in key.
    pub giphy_key: String,
    /// Keep the app linked in the tray when the window closes.
    pub keep_running_in_background: bool,
    /// Desktop notifications while away from the chat.
    pub notifications: bool,
    /// Ask GitHub once a day whether a newer release exists.
    pub check_for_updates: bool,
    /// Download verified updates in the background. Install starts from the Update toast.
    #[serde(default = "default_true")]
    pub download_updates_automatically: bool,
    /// Prefer address-book names over public profile names.
    pub names_from_contacts: bool,
    /// Voice and audio playback speed multiplier.
    pub voice_speed: f32,
    /// Send a forwarded batch one at a time, waiting for each first tick.
    pub forward_in_order: bool,
    /// Show the Create poll button beside the composer.
    pub show_poll_button: bool,
    /// Ask where to save each attachment, instead of always using Downloads.
    pub ask_where_to_save: bool,
    /// Also add saved contacts to the phone's address book.
    pub save_contacts_to_phone: bool,
    /// Hide the sidebar outright instead of narrowing it to the chat pictures.
    #[serde(default)]
    pub hide_sidebar_fully: bool,
    /// Slowly fetch older phone history and files in the background.
    pub history_prefetch: HistoryPrefetch,
    /// Doodle wallpaper behind the open chat. Auto follows the theme colours.
    pub chat_wallpaper: ChatWallpaper,
    /// Which bundled doodle in the family (`0` is `01.png`).
    #[serde(default)]
    pub chat_wallpaper_index: u8,
}

fn default_true() -> bool {
    true
}

fn default_inspector_width() -> f32 {
    380.0
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: ThemeChoice::Dark,
            language: Language::default(),
            language: Language::default(),
            language: Language::default(),
            language: Language::default(),
            language: Language::default(),
            custom_theme: None,
            custom_theme_cache: None,
            system_theme_cache: None,
            zoom: 1.0,
            sidebar_width: 320.0,
            inspector_width: default_inspector_width(),
            enter_sends: true,
            send_read_receipts: true,
            send_typing: true,
            auto_download: true,
            show_sender_pictures: false,
            last_chat: None,
            show_shortcut_hints: true,
            recent_emoji: Vec::new(),
            giphy_key: String::new(),
            keep_running_in_background: true,
            notifications: true,
            check_for_updates: true,
            download_updates_automatically: true,
            names_from_contacts: true,
            save_contacts_to_phone: true,
            forward_in_order: true,
            show_poll_button: true,
            ask_where_to_save: false,
            voice_speed: 1.0,
            hide_sidebar_fully: false,
            history_prefetch: HistoryPrefetch::RecentAndPinned,
            chat_wallpaper: ChatWallpaper::Auto,
            chat_wallpaper_index: 0,
        }
    }
}

/// Optional build-time GIPHY key from `ZAPFAST_GIPHY_KEY`.
/// The previous name remains accepted for existing build setups.
pub const BUILT_IN_GIPHY_KEY: Option<&str> = match option_env!("ZAPFAST_GIPHY_KEY") {
    Some(key) if !key.is_empty() => Some(key),
    _ => option_env!("FASTSAPP_GIPHY_KEY"),
};

impl Settings {
    pub(crate) fn cached_palette(&self) -> Option<crate::theme::Palette> {
        let theme = if self.custom_theme.is_some() {
            self.custom_theme_cache.as_ref()
        } else if self.theme == ThemeChoice::System {
            self.system_theme_cache.as_ref()
        } else {
            None
        };
        theme.map(|theme| theme.palette)
    }

    /// Returns the user key, built-in key, or `None`.
    pub fn effective_giphy_key(&self) -> Option<String> {
        let own = self.giphy_key.trim();
        if !own.is_empty() {
            return Some(own.to_owned());
        }
        BUILT_IN_GIPHY_KEY
            .map(str::trim)
            .filter(|key| !key.is_empty())
            .map(str::to_owned)
    }

    pub fn wallpaper_slot(&self) -> u8 {
        self.chat_wallpaper_index.min(WALLPAPER_SLOTS - 1)
    }

    pub fn next_wallpaper(&mut self) {
        self.chat_wallpaper_index = (self.wallpaper_slot() + 1) % WALLPAPER_SLOTS;
    }

    pub fn load(path: &Path) -> Self {
        match std::fs::read_to_string(path) {
            Ok(contents) => match serde_json::from_str(&contents) {
                Ok(settings) => settings,
                Err(error) => {
                    log::warn!("settings file is unreadable, using defaults: {error}");
                    Self::default()
                }
            },
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Self::default(),
            Err(error) => {
                log::warn!("could not read settings: {error}");
                Self::default()
            }
        }
    }

    /// Atomically replaces the settings file through a temporary file.
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let contents = serde_json::to_string_pretty(self).map_err(std::io::Error::other)?;
        let temp = path.with_extension("json.tmp");
        std::fs::write(&temp, contents)?;
        std::fs::rename(&temp, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_and_missing_fields_are_tolerated() {
        let parsed: Settings =
            serde_json::from_str(r#"{"theme":"light","future_field":1}"#).expect("parses");
        assert_eq!(parsed.theme, ThemeChoice::Light);
        assert!(parsed.enter_sends);
        assert!(parsed.check_for_updates);
        assert!(parsed.download_updates_automatically);
        assert_eq!(parsed.history_prefetch, HistoryPrefetch::RecentAndPinned);
        assert_eq!(parsed.chat_wallpaper, ChatWallpaper::Auto);
        assert_eq!(parsed.chat_wallpaper_index, 0);
    }

    #[test]
    fn history_prefetch_names_the_open_chat_current_chat() {
        assert_eq!(HistoryPrefetch::Focused.label(), "Current Chat");
        assert!(HistoryPrefetch::Off.hint().contains("Do not fetch"));
        assert!(HistoryPrefetch::Focused.hint().contains("open chat"));
        assert!(HistoryPrefetch::RecentAndPinned.hint().contains("pinned"));
    }

    #[test]
    fn next_wallpaper_wraps_the_slot_and_keeps_the_family() {
        let mut settings = Settings::default();
        settings.next_wallpaper();
        assert_eq!(settings.chat_wallpaper, ChatWallpaper::Auto);
        assert_eq!(settings.chat_wallpaper_index, 1);
        settings.next_wallpaper();
        settings.next_wallpaper();
        assert_eq!(settings.chat_wallpaper_index, 0);
        settings.chat_wallpaper = ChatWallpaper::Black;
        settings.chat_wallpaper_index = 2;
        settings.next_wallpaper();
        assert_eq!(settings.chat_wallpaper, ChatWallpaper::Black);
        assert_eq!(settings.chat_wallpaper_index, 0);
    }

    #[test]
    fn a_legacy_family_choice_is_the_first_slot() {
        let parsed: Settings =
            serde_json::from_str(r#"{"chat_wallpaper":"black"}"#).expect("parses");
        assert_eq!(parsed.chat_wallpaper, ChatWallpaper::Black);
        assert_eq!(parsed.chat_wallpaper_index, 0);
    }

    #[test]
    fn damaged_theme_cache_does_not_discard_other_settings() {
        let settings: Settings = serde_json::from_str(r#"{"custom_theme":"mine.json","custom_theme_cache":{"damaged":true},"enter_sends":false}"#).unwrap();
        assert!(!settings.enter_sends);
        assert!(settings.custom_theme_cache.is_none());
        assert_eq!(settings.custom_theme.as_deref(), Some("mine.json"));
    }

    #[test]
    fn round_trips_through_disk() {
        let dir = std::env::temp_dir().join(format!("whatsfast-settings-{}", std::process::id()));
        let path = dir.join("settings.json");
        let settings = Settings {
            zoom: 1.25,
            enter_sends: false,
            voice_speed: 1.5,
            ..Settings::default()
        };
        settings.save(&path).expect("saves");
        assert_eq!(Settings::load(&path), settings);
        let _ = std::fs::remove_dir_all(dir);
    }
}

#[cfg(test)]
mod giphy_tests {
    use super::*;

    #[test]
    fn the_users_key_wins_and_is_trimmed() {
        let settings = Settings {
            giphy_key: "  abc  ".into(),
            ..Settings::default()
        };
        assert_eq!(settings.effective_giphy_key().as_deref(), Some("abc"));
    }

    #[test]
    fn without_a_key_of_their_own_the_built_in_one_is_used() {
        let settings = Settings {
            giphy_key: "   ".into(),
            ..Settings::default()
        };
        let expected = BUILT_IN_GIPHY_KEY
            .map(str::trim)
            .filter(|key| !key.is_empty())
            .map(str::to_owned);
        assert_eq!(settings.effective_giphy_key(), expected);
    }
}
