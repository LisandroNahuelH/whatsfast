//! The interface language is a process-wide switch, so this test lives in its
//! own binary: it flips the language and never races tests that paint English
//! text (the module tests run in parallel threads of the lib binary).

use whatsfast::i18n::{self, Key, Language, Locale};

#[test]
fn a_startup_language_is_read_back() {
    let previous = i18n::language();
    i18n::set_language(Language::Spanish);
    assert_eq!(i18n::language(), Locale::Es);
    assert_eq!(i18n::t(Key::SettingsThemeDark), "Oscuro");
    i18n::set_language(Language::English);
    assert_eq!(i18n::language(), Locale::En);
    assert_eq!(i18n::t(Key::SettingsThemeDark), "Dark");
    assert_eq!(previous, Locale::En);
}
