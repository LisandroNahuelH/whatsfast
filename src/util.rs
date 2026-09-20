//! Small formatting helpers shared by the views.

use jiff::civil::Date;
use jiff::{Timestamp, Zoned};

use crate::i18n::{self, Key};
use crate::schedule::{month_abbr, month_name, weekday_abbr, weekday_name};

/// File-loader identifier for a native path. egui requires a slash after
/// `file://` on Windows or it interprets a drive path as a UNC hostname.
/// Keep native characters: egui's loader does not percent-decode URLs.
pub fn image_uri(path: &std::path::Path) -> String {
    image_uri_for_platform(&path.to_string_lossy(), cfg!(windows))
}

fn image_uri_for_platform(path: &str, windows: bool) -> String {
    format!("file://{}{path}", if windows { "/" } else { "" })
}

/// Converts a Unix timestamp to local time.
fn zoned(unix_seconds: i64) -> Option<Zoned> {
    let timestamp = Timestamp::from_second(unix_seconds).ok()?;
    Some(timestamp.to_zoned(jiff::tz::TimeZone::system()))
}

fn today() -> Date {
    Zoned::now().date()
}

/// Local message time such as "14:05".
pub fn clock(unix_seconds: i64) -> String {
    zoned(unix_seconds)
        .map(|when| format!("{:02}:{:02}", when.hour(), when.minute()))
        .unwrap_or_default()
}

/// WhatsApp transcript timestamp such as `22:41, 8/18/2026`.
pub fn copy_stamp(unix_seconds: i64) -> String {
    zoned(unix_seconds)
        .map(|when| {
            let time = format!("{:02}:{:02}", when.hour(), when.minute());
            i18n::f(
                Key::CopyStamp,
                &[
                    ("time", &time),
                    ("day", &when.date().day().to_string()),
                    ("month", &when.date().month().to_string()),
                    ("year", &when.date().year().to_string()),
                ],
            )
        })
        .unwrap_or_default()
}

/// Chat-row timestamp: time today, weekday this week, or date.
pub fn chat_stamp(unix_seconds: i64) -> String {
    let Some(when) = zoned(unix_seconds) else {
        return String::new();
    };
    stamp_relative_to(when.date(), today(), &when)
}

fn stamp_relative_to(date: Date, today: Date, when: &Zoned) -> String {
    let days = today
        .since(date)
        .map(|span| span.get_days())
        .unwrap_or(i32::MAX);
    match days {
        0 => format!("{:02}:{:02}", when.hour(), when.minute()),
        1 => i18n::t(Key::DateYesterday).to_owned(),
        2..=6 => weekday_name(date.weekday()).to_owned(),
        _ => short_date(date),
    }
}

/// Splits a display name into first name and surname for editor defaults.
pub fn split_name(name: &str) -> (String, String) {
    let name = name.trim();
    match name.split_once(' ') {
        Some((first, rest)) => (first.to_owned(), rest.trim().to_owned()),
        None => (name.to_owned(), String::new()),
    }
}

/// Message-info timestamp with date and minute.
pub fn moment_stamp(unix_seconds: i64) -> String {
    let Some(when) = zoned(unix_seconds) else {
        return String::new();
    };
    let time = format!("{:02}:{:02}", when.hour(), when.minute());
    let days = today()
        .since(when.date())
        .map(|span| span.get_days())
        .unwrap_or(i32::MAX);
    match days {
        0 => time,
        1 => i18n::f(Key::DateYesterdayAt, &[("time", time.as_str())]),
        2..=6 => i18n::f(
            Key::DateAt,
            &[
                ("moment", weekday_name(when.date().weekday())),
                ("time", time.as_str()),
            ],
        ),
        _ => i18n::f(
            Key::DateAt,
            &[
                ("moment", short_date(when.date()).as_str()),
                ("time", time.as_str()),
            ],
        ),
    }
}

/// Short stamp for lists, such as "Fri 18 Sep, 21:00".
pub fn short_stamp(unix_seconds: i64) -> String {
    let Some(when) = zoned(unix_seconds) else {
        return String::new();
    };
    i18n::f(
        Key::DateStampShort,
        &[
            ("weekday", weekday_abbr(when.date().weekday())),
            ("day", &when.date().day().to_string()),
            ("month", month_abbr(when.date().month())),
            ("time", &format!("{:02}:{:02}", when.hour(), when.minute())),
        ],
    )
}

/// Conversation day-separator label.
pub fn day_label(unix_seconds: i64) -> String {
    let Some(when) = zoned(unix_seconds) else {
        return String::new();
    };
    let date = when.date();
    let today = today();
    let days = today
        .since(date)
        .map(|span| span.get_days())
        .unwrap_or(i32::MAX);
    match days {
        0 => i18n::t(Key::DateToday).to_owned(),
        1 => i18n::t(Key::DateYesterday).to_owned(),
        2..=6 => weekday_name(date.weekday()).to_owned(),
        _ => long_date(date),
    }
}

/// Local calendar day used to group messages.
pub fn day_key(unix_seconds: i64) -> Option<Date> {
    zoned(unix_seconds).map(|when| when.date())
}

/// Unix-second half-open range for a local calendar day.
pub fn day_bounds(date: Date) -> Option<(i64, i64)> {
    let start = crate::schedule::instant_of(date, 0, 0)?;
    let end = crate::schedule::instant_of(date.tomorrow().ok()?, 0, 0)?;
    Some((start, end))
}

/// `on_slices` ON slices of 200 ms, with 200 ms OFF between them.
pub fn highlight_flash(elapsed_ms: u64, on_slices: u64) -> Option<bool> {
    let total = on_slices.max(1).saturating_mul(400);
    if elapsed_ms >= total {
        None
    } else {
        Some((elapsed_ms / 200).is_multiple_of(2))
    }
}

fn short_date(date: Date) -> String {
    i18n::f(
        Key::DateShort,
        &[
            ("day", &date.day().to_string()),
            ("month", month_abbr(date.month())),
            ("year", &date.year().to_string()),
        ],
    )
}

fn long_date(date: Date) -> String {
    i18n::f(
        Key::DateLong,
        &[
            ("weekday", weekday_name(date.weekday())),
            ("day", &date.day().to_string()),
            ("month", month_name(date.month())),
            ("year", &date.year().to_string()),
        ],
    )
}

/// The current time as a Unix timestamp.
pub fn now() -> i64 {
    Timestamp::now().as_second()
}

/// Case- and accent-insensitive matching without changing displayed names.
pub fn search_key(text: &str) -> String {
    use icu_normalizer::DecomposingNormalizerBorrowed;
    use icu_properties::{CodePointMapData, props::GeneralCategory};

    if text.is_ascii() {
        return text.to_ascii_lowercase();
    }
    DecomposingNormalizerBorrowed::new_nfd()
        .normalize_iter(text.chars())
        .filter(|c| {
            CodePointMapData::<GeneralCategory>::new().get(*c) != GeneralCategory::NonspacingMark
        })
        .flat_map(char::to_lowercase)
        .collect()
}

/// Duration such as "0:12".
pub fn duration(seconds: u32) -> String {
    format!("{}:{:02}", seconds / 60, seconds % 60)
}

/// File size such as "1.2 MB".
pub fn bytes(size: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
    let mut value = size as f64;
    let mut unit = 0;
    while value >= 1000.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{size} B")
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

/// Up to two initials for a fallback avatar.
pub fn initials(name: &str) -> String {
    let mut words = name
        .split(|character: char| character.is_whitespace() || character == '-')
        .filter(|word| word.chars().any(char::is_alphanumeric));
    let first = words.next();
    let last = words.next_back();
    let mut initials = String::new();
    for word in [first, last].into_iter().flatten() {
        if let Some(character) = word.chars().find(|character| character.is_alphanumeric()) {
            initials.extend(character.to_uppercase());
        }
    }
    if initials.is_empty() {
        initials.push('#');
    }
    initials
}

/// Formats a phone number with a plus sign and grouped digits.
pub fn phone(digits: &str) -> String {
    let digits: String = digits.chars().filter(char::is_ascii_digit).collect();
    if digits.is_empty() {
        return String::new();
    }
    let mut out = String::from("+");
    for (index, character) in digits.chars().enumerate() {
        // Approximate a country code followed by groups of three digits.
        if index == 2 || (index > 2 && (index - 2) % 3 == 0) {
            out.push(' ');
        }
        out.push(character);
    }
    out
}

/// Stable id-derived avatar hue.
pub fn hue(seed: &str) -> f32 {
    let mut hash: u32 = 2_166_136_261;
    for byte in seed.bytes() {
        hash ^= u32::from(byte);
        hash = hash.wrapping_mul(16_777_619);
    }
    (hash % 360) as f32
}

/// Embedded SVG app logo used across platform surfaces.
const MARK: &[u8] = include_bytes!("../packaging/icons/whatsfast.svg");

fn raster_svg_rgba(svg: &[u8], size: usize) -> Option<Vec<u8>> {
    let side = size.max(1) as u32;
    let tree = resvg::usvg::Tree::from_data(svg, &resvg::usvg::Options::default()).ok()?;
    let mut pixmap = resvg::tiny_skia::Pixmap::new(side, side)?;
    let scale = side as f32 / tree.size().width();
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::from_scale(scale, scale),
        &mut pixmap.as_mut(),
    );
    Some(
        pixmap
            .pixels()
            .iter()
            .flat_map(|pixel| {
                let color = pixel.demultiply();
                [color.red(), color.green(), color.blue(), color.alpha()]
            })
            .collect(),
    )
}

/// Rasterizes the logo to straight-alpha RGBA.
pub fn app_icon_rgba(size: usize) -> Vec<u8> {
    raster_svg_rgba(MARK, size).unwrap_or_else(|| plain_disc(size))
}

fn plain_disc(size: usize) -> Vec<u8> {
    let mut rgba = vec![0u8; size * size * 4];
    let center = size as f32 / 2.0;
    let radius = center - 2.0;
    for y in 0..size {
        for x in 0..size {
            let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
            let distance = ((px - center).powi(2) + (py - center).powi(2)).sqrt();
            let coverage = (radius - distance + 0.5).clamp(0.0, 1.0);
            let index = (y * size + x) * 4;
            rgba[index] = 0xe8;
            rgba[index + 1] = 0x5d;
            rgba[index + 2] = 0x04;
            rgba[index + 3] = (coverage * 255.0) as u8;
        }
    }
    rgba
}

/// Converts the logo to a monochrome macOS menu-bar template.
pub fn tray_template_rgba(size: usize) -> Vec<u8> {
    let mut rgba = app_icon_rgba(size);
    for pixel in rgba.as_chunks_mut::<4>().0 {
        if pixel[0] > 200 && pixel[1] > 200 && pixel[2] > 200 {
            pixel[3] = 0;
        }
        pixel[0] = 0;
        pixel[1] = 0;
        pixel[2] = 0;
    }
    rgba
}

#[cfg(test)]
mod tests {
    #[test]
    fn image_paths_keep_the_native_path_after_loader_conversion() {
        for path in [
            r"C:\Users\Ada\photo.jpg",
            r"C:\Users\A B\100% #猫.png",
            r"\\server\share\photo.jpg",
            r"\\?\C:\cache\photo.jpg",
        ] {
            let uri = super::image_uri_for_platform(path, true);
            // Mirrors egui_extras' Windows file loader: its first slash
            // selects a native path, otherwise it prepends a UNC prefix.
            assert_eq!(uri.strip_prefix("file:///").unwrap(), path);
        }
        assert_eq!(
            super::image_uri_for_platform("/home/ada/猫 #1.png", false),
            "file:///home/ada/猫 #1.png"
        );
    }

    #[test]
    fn image_loader_reads_native_paths() {
        use egui::load::BytesPoll;
        let dir =
            std::env::temp_dir().join(format!("whatsfast-image-paths-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("猫 photo 100% #1.png");
        std::fs::write(&path, b"image bytes").unwrap();
        let ctx = egui::Context::default();
        egui_extras::install_image_loaders(&ctx);
        let uri = super::image_uri(&path);
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        loop {
            match ctx.try_load_bytes(&uri).unwrap() {
                BytesPoll::Ready { bytes, .. } => {
                    assert_eq!(bytes.as_ref(), b"image bytes");
                    break;
                }
                BytesPoll::Pending { .. } => {
                    assert!(std::time::Instant::now() < deadline);
                    std::thread::sleep(std::time::Duration::from_millis(5));
                }
            }
        }
        std::fs::remove_dir_all(dir).unwrap();
    }

    use super::*;

    #[test]
    fn initials_take_first_and_last_word() {
        assert_eq!(initials("Ada Lovelace"), "AL");
        assert_eq!(initials("ada"), "A");
        assert_eq!(initials("  "), "#");
        assert_eq!(initials("🎉 Party Planning"), "PP");
    }

    #[test]
    fn phone_numbers_are_grouped() {
        assert_eq!(phone("393331234567"), "+39 333 123 456 7");
        assert_eq!(phone("15551234567"), "+15 551 234 567");
        assert_eq!(phone(""), "");
    }

    #[test]
    fn stamps_fall_back_to_dates() {
        let when = Timestamp::from_second(1_700_000_000)
            .expect("valid")
            .to_zoned(jiff::tz::TimeZone::UTC);
        let date = when.date();
        assert_eq!(stamp_relative_to(date, date, &when), "22:13");
        assert_eq!(
            stamp_relative_to(date, date.tomorrow().expect("date"), &when),
            "Yesterday"
        );
        assert_eq!(
            stamp_relative_to(
                date,
                date.checked_add(jiff::Span::new().days(3)).expect("date"),
                &when
            ),
            "Tuesday"
        );
        assert_eq!(
            stamp_relative_to(
                date,
                date.checked_add(jiff::Span::new().days(30)).expect("date"),
                &when
            ),
            "14 Nov 2023"
        );
    }

    #[test]
    fn sizes_and_durations_read_naturally() {
        assert_eq!(bytes(512), "512 B");
        assert_eq!(bytes(2_048), "2.0 KB");
        assert_eq!(bytes(5 * 1024 * 1024), "5.0 MB");
        assert_eq!(duration(75), "1:15");
    }

    #[test]
    fn icon_is_opaque_in_the_middle_and_clear_at_the_corners() {
        let icon = app_icon_rgba(32);
        assert_eq!(icon[3], 0);
        let middle = (16 * 32 + 16) * 4;
        assert_eq!(icon[middle + 3], 255);
        assert!(
            icon[middle] > 180 && icon[middle + 2] < 80,
            "the disc is amber, not WhatsApp green"
        );
    }

    #[test]
    fn highlight_flash_is_three_on_slices() {
        assert_eq!(crate::util::highlight_flash(0, 3), Some(true));
        assert_eq!(crate::util::highlight_flash(199, 3), Some(true));
        assert_eq!(crate::util::highlight_flash(200, 3), Some(false));
        assert_eq!(crate::util::highlight_flash(400, 3), Some(true));
        assert_eq!(crate::util::highlight_flash(1199, 3), Some(false));
        assert_eq!(crate::util::highlight_flash(1200, 3), None);
    }

    #[test]
    fn highlight_flash_one_slice_lasts_four_hundred_ms() {
        assert_eq!(crate::util::highlight_flash(0, 1), Some(true));
        assert_eq!(crate::util::highlight_flash(199, 1), Some(true));
        assert_eq!(crate::util::highlight_flash(200, 1), Some(false));
        assert_eq!(crate::util::highlight_flash(399, 1), Some(false));
        assert_eq!(crate::util::highlight_flash(400, 1), None);
    }

    #[test]
    #[ignore = "run with --ignored to refresh packaging PNG assets from the SVG mark"]
    fn write_packaging_pngs() {
        let circle = app_icon_rgba(1024);
        image::save_buffer(
            "packaging/icons/whatsfast-1024.png",
            &circle,
            1024,
            1024,
            image::ExtendedColorType::Rgba8,
        )
        .expect("circle png");
        let macos = raster_svg_rgba(include_bytes!("../packaging/macos/icon-1024.svg"), 1024)
            .expect("macos svg");
        image::save_buffer(
            "packaging/macos/icon-1024.png",
            &macos,
            1024,
            1024,
            image::ExtendedColorType::Rgba8,
        )
        .expect("macos png");
    }
}
