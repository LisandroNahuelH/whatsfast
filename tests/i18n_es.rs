//! The Spanish interface covers every demo page.
//!
//! Separate test binary: the interface language is a process-wide global, so
//! this test must not share a process with tests that expect the English
//! default.
#![cfg(feature = "demo")]

use std::collections::HashSet;

use whatsfast::{
    demo::tour::harvest_labels,
    i18n::{self, Key, Language},
};

/// Demo pages from `--demo-page`, plus the default chat page (`None`).
const PAGES: &[Option<&str>] = &[
    None,
    Some("empty"),
    Some("settings"),
    Some("login"),
    Some("logged-out"),
    Some("pair"),
    Some("shortcuts"),
    Some("about"),
    Some("info"),
    Some("mention"),
    Some("light"),
    Some("privacy-except"),
    Some("image-viewer"),
    Some("poll"),
    Some("update-failed"),
];

/// Least number of labels each page must paint, measured on a green run. A
/// page that stops painting its rows fails here instead of passing silently.
const FLOOR: &[(Option<&str>, usize)] = &[
    (None, 81),
    (Some("empty"), 39),
    (Some("settings"), 108),
    (Some("login"), 11),
    (Some("logged-out"), 5),
    (Some("pair"), 13),
    (Some("shortcuts"), 114),
    (Some("about"), 89),
    (Some("info"), 90),
    (Some("mention"), 66),
    (Some("light"), 81),
    (Some("privacy-except"), 113),
    (Some("image-viewer"), 78),
    (Some("poll"), 66),
    (Some("update-failed"), 84),
];

/// English templates whose Spanish text differs: a label equal to one of these
/// is untranslated text still painted on the page.
fn english_only() -> HashSet<String> {
    i18n::set_language(Language::English);
    let english: Vec<String> = Key::ALL
        .iter()
        .map(|key| i18n::t(*key).to_owned())
        .collect();
    i18n::set_language(Language::Spanish);
    let spanish: Vec<String> = Key::ALL
        .iter()
        .map(|key| i18n::t(*key).to_owned())
        .collect();
    english
        .into_iter()
        .zip(spanish)
        .filter(|(en, es)| en != es)
        .map(|(en, _)| en)
        .collect()
}

#[test]
fn no_english_label_is_left_on_a_spanish_page() {
    let forbidden = english_only();
    let mut failures = Vec::new();
    for page in PAGES {
        let labels = harvest_labels(*page);
        let floor = FLOOR
            .iter()
            .find(|(name, _)| *name == *page)
            .map_or(3, |(_, count)| *count);
        assert!(
            labels.len() >= floor,
            "page {page:?} painted {} labels; the floor is {floor}, so the harvest lost coverage: {labels:?}",
            labels.len()
        );
        for label in &labels {
            if forbidden.contains(label) {
                failures.push(format!("{page:?}: {label}"));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "English labels on Spanish pages:\n{}",
        failures.join("\n")
    );
}

#[test]
fn spanish_pages_share_the_english_layout_labels() {
    // Labels that are the same in both languages must still be painted, so the
    // page keeps its anchors for the tour and for screenshots.
    i18n::set_language(Language::Spanish);
    let labels = harvest_labels(Some("settings"));
    let expected = ["Zoom", "Chats"];
    for label in expected {
        assert!(
            labels.iter().any(|painted| painted == label),
            "settings page lost the shared label {label:?}: {labels:?}"
        );
    }
}
