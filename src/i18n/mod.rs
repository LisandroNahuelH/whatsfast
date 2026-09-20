//! The interface language: English and Spanish.
//!
//! `Language` is the preference stored in settings; `Locale` is a resolved
//! language, which is what every table lookup uses. The process-wide locale
//! lives in one atomic and is written in exactly two places: the startup in
//! `main.rs` (which resolves `System`) and the Settings selector (which
//! applies a change at once). Everything without an `App` in reach (the
//! worker, the tray, notifications) reads `t()` and follows the same value.
//!
//! Strings live in `key.rs` (the `Key` enum), `en.rs`, and `es.rs`. The match
//! in each locale file is exhaustive, so the compiler refuses to build a
//! language that misses a string.

mod en;
mod es;
mod key;
pub use key::Key;

use std::sync::OnceLock;
use std::sync::atomic::{AtomicU8, Ordering};

use serde::{Deserialize, Serialize};

/// A resolved interface language. `System` never reaches this type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locale {
    En,
    Es,
}

impl Locale {
    /// The locale tag reported in logs and about screens.
    pub fn code(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Es => "es",
        }
    }

    fn index(self) -> u8 {
        match self {
            Self::En => 0,
            Self::Es => 1,
        }
    }

    fn from_index(value: u8) -> Self {
        if value == 1 { Self::Es } else { Self::En }
    }
}

/// The language preference. `System` follows the operating system.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    /// Follow the operating system language.
    #[default]
    System,
    English,
    Spanish,
}

impl Language {
    pub const ALL: [Language; 3] = [Self::System, Self::English, Self::Spanish];

    /// The concrete locale for this preference. `System` asks the operating
    /// system once and caches the answer for the rest of the run.
    pub fn resolved(self) -> Locale {
        match self {
            Self::English => Locale::En,
            Self::Spanish => Locale::Es,
            Self::System => system_locale(),
        }
    }

    /// The label for the Settings selector. `English` and `Español` are
    /// endonyms and never change; the system option follows the interface.
    pub fn label(self) -> &'static str {
        match self {
            Self::System => t(Key::SettingsLanguageSystem),
            Self::English => t(Key::SettingsLanguageEnglish),
            Self::Spanish => t(Key::SettingsLanguageSpanish),
        }
    }
}

impl std::str::FromStr for Language {
    type Err = ();

    /// Parses `system`, `en`, or `es` (case-insensitive).
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text.to_ascii_lowercase().as_str() {
            "system" => Ok(Self::System),
            "en" | "english" => Ok(Self::English),
            "es" | "spanish" => Ok(Self::Spanish),
            _ => Err(()),
        }
    }
}

/// The locale every interface string renders in. Starts in English as the
/// pre-resolution state; every real start overwrites it from settings before
/// the interface is built.
static LANG: AtomicU8 = AtomicU8::new(0);

/// The locale the interface renders in now.
pub fn language() -> Locale {
    Locale::from_index(LANG.load(Ordering::Relaxed))
}

/// Applies a preference. Called at startup and from the Settings selector.
pub fn set_language(language: Language) {
    LANG.store(language.resolved().index(), Ordering::Relaxed);
}

/// The string of `key` in `locale`. Pure: table lookup only.
pub(crate) fn text(locale: Locale, key: Key) -> &'static str {
    match locale {
        Locale::En => en::text(key),
        Locale::Es => es::text(key),
    }
}

/// The string of `key` in the current locale.
pub fn t(key: Key) -> &'static str {
    text(language(), key)
}

/// The template of `key` with `{name}` placeholders filled in.
pub fn f(key: Key, args: &[(&str, &str)]) -> String {
    fill(t(key), args)
}

/// Picks the one or many form for `count` and fills `{count}` in the many one.
pub fn count(one: Key, many: Key, count: usize) -> String {
    if count == 1 {
        t(one).to_owned()
    } else {
        f(many, &[("count", &count.to_string())])
    }
}

/// Fills `{name}` placeholders in a single pass: a substituted value is never
/// scanned again, so a value that contains braces survives untouched. A name
/// with no argument stays literal (and trips a debug assertion while
/// developing).
pub fn fill(template: &str, args: &[(&str, &str)]) -> String {
    let mut out = String::with_capacity(template.len() + 16);
    let mut rest = template;
    while let Some(start) = rest.find('{') {
        out.push_str(&rest[..start]);
        let after = &rest[start + 1..];
        let Some(end) = after.find('}') else {
            out.push_str(&rest[start..]);
            return out;
        };
        let name = &after[..end];
        match args.iter().find(|(candidate, _)| *candidate == name) {
            Some((_, value)) => out.push_str(value),
            None => {
                debug_assert!(false, "missing placeholder: {name}");
                out.push('{');
                out.push_str(name);
                out.push('}');
            }
        }
        rest = &after[end + 1..];
    }
    out.push_str(rest);
    out
}

/// The system language, detected once per run.
fn system_locale() -> Locale {
    static SNAPSHOT: OnceLock<Locale> = OnceLock::new();
    *SNAPSHOT.get_or_init(detect_system_locale)
}

/// Maps a locale tag such as `es-AR`, `es_AR.UTF-8`, or `en-US` to a locale.
/// Anything that is not Spanish reads as English.
pub(crate) fn locale_from_tag(tag: &str) -> Locale {
    if tag.trim().to_ascii_lowercase().starts_with("es") {
        Locale::Es
    } else {
        Locale::En
    }
}

#[cfg(windows)]
fn detect_system_locale() -> Locale {
    use windows_sys::Win32::Globalization::GetUserDefaultLocaleName;

    // LOCALE_NAME_MAX_LENGTH written literally: the windows-sys constant lives
    // behind the Win32_System_SystemServices feature, which this crate does
    // not enable.
    let mut buffer = [0u16; 85];
    let written = unsafe { GetUserDefaultLocaleName(buffer.as_mut_ptr(), buffer.len() as i32) };
    if written <= 1 {
        return Locale::En;
    }
    let name = String::from_utf16_lossy(&buffer[..written as usize - 1]);
    locale_from_tag(&name)
}

/// Linux and macOS read the usual environment variables. Untested here: the
/// release target is Windows, and this branch exists so the other platforms
/// keep building and behaving sensibly.
#[cfg(not(windows))]
fn detect_system_locale() -> Locale {
    ["LC_ALL", "LC_MESSAGES", "LANG"]
        .iter()
        .find_map(|name| std::env::var(name).ok().filter(|value| !value.is_empty()))
        .map(|value| locale_from_tag(&value))
        .unwrap_or(Locale::En)
}

/// Strings that are the same in both languages on purpose: brand names,
/// protocol tokens, symbols, and words the interface keeps as they are.
#[cfg(test)]
pub(crate) const ALLOWED_IDENTICAL: &[Key] = &[
    Key::SettingsLanguageEnglish,
    Key::SettingsLanguageSpanish,
    // The interface word for "chats" and the "Zoom" label stay as they are.
    Key::SettingsSectionChats,
    Key::SettingsZoomLabel,
    // "Video" and "Videos" are the same word in both languages.
    Key::SettingsStorageVideos,
    Key::DateShort,
    Key::DateShortWeekday,
    Key::DateStampShort,
    Key::DateMonthAbbrMay,
    Key::PickerTabEmoji,
    Key::PickerTabGif,
    Key::PickerTabStickers,
    Key::ChatSearchSectionChats,
    Key::ChatInfo,
    Key::MarkerGif,
    Key::MarkerVideo,
    Key::MarkerAudio,
    Key::MarkerSticker,
    Key::ChatWhatAt,
    Key::KindGif,
    Key::KindVideo,
    Key::KindAudio,
    Key::KindSticker,
    Key::ErrDetail,
    Key::GifErrMessage,
    Key::KindStickers,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_key_has_text_in_both_locales() {
        for key in Key::ALL {
            for locale in [Locale::En, Locale::Es] {
                let text = text(locale, key);
                assert!(!text.is_empty(), "{key:?} is empty in {locale:?}");
                assert_eq!(
                    text.trim_start(),
                    text,
                    "{key:?} in {locale:?} has stray spaces"
                );
            }
        }
    }

    #[test]
    fn spanish_differs_from_english_unless_allowed() {
        let unexpected: Vec<Key> = Key::ALL
            .iter()
            .copied()
            .filter(|key| {
                text(Locale::En, *key) == text(Locale::Es, *key) && !ALLOWED_IDENTICAL.contains(key)
            })
            .collect();
        assert!(
            unexpected.is_empty(),
            "identical in both languages and not allowed: {unexpected:?}"
        );
    }

    #[test]
    fn placeholders_match_between_locales() {
        fn names(text: &str) -> Vec<&str> {
            let mut found = Vec::new();
            let mut rest = text;
            while let Some(start) = rest.find('{') {
                let after = &rest[start + 1..];
                let Some(end) = after.find('}') else { break };
                found.push(&after[..end]);
                rest = &after[end + 1..];
            }
            found.sort_unstable();
            found
        }
        for key in Key::ALL {
            assert_eq!(
                names(text(Locale::En, key)),
                names(text(Locale::Es, key)),
                "{key:?} uses different placeholders per language"
            );
        }
    }

    #[test]
    fn fill_replaces_in_one_pass_and_keeps_unknown_names() {
        assert_eq!(fill("a {x} b {y}", &[("x", "1"), ("y", "2")]), "a 1 b 2");
        assert_eq!(fill("a {x}", &[("x", "{y}")]), "a {y}");
        assert_eq!(fill("no braces", &[("x", "1")]), "no braces");
    }

    #[test]
    fn languages_resolve_and_read_back() {
        assert_eq!(Language::English.resolved(), Locale::En);
        assert_eq!(Language::Spanish.resolved(), Locale::Es);
        assert_eq!(Language::default(), Language::System);
        assert_eq!(Locale::Es.code(), "es");
    }

    #[test]
    fn tags_map_to_locales() {
        assert_eq!(locale_from_tag("es-AR"), Locale::Es);
        assert_eq!(locale_from_tag("es_AR.UTF-8"), Locale::Es);
        assert_eq!(locale_from_tag("ES"), Locale::Es);
        assert_eq!(locale_from_tag("en-US"), Locale::En);
        assert_eq!(locale_from_tag(""), Locale::En);
    }

    // `a_startup_language_is_read_back` lives in tests/i18n_language.rs: it
    // flips the process-wide language, which would race the tests that paint
    // English labels in this same binary.
}
