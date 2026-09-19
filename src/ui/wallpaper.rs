//! Bundled doodle wallpapers behind the open chat.

use egui::{Color32, ColorImage, Pos2, Rect, TextureHandle, TextureOptions, Ui, pos2};

use crate::settings::{ChatWallpaper, WallpaperFamily};
use crate::theme::Palette;

/// Chooses a wallpaper family from the active palette.
pub fn family_for(palette: &Palette) -> WallpaperFamily {
    if !palette.dark {
        return WallpaperFamily::White;
    }
    let (hue, _sat, _) = rgb_hsl(palette.accent);
    if (80.0..180.0).contains(&hue) {
        return WallpaperFamily::Green;
    }
    if hue < 40.0 || hue >= 340.0 {
        return WallpaperFamily::Red;
    }
    let (_, _, window_l) = rgb_hsl(palette.window);
    if window_l >= 0.19 {
        WallpaperFamily::Gray
    } else {
        WallpaperFamily::Black
    }
}

/// Paints the wallpaper covering `ui.max_rect()`, cropping overflow.
pub fn paint(ui: &mut Ui, family: WallpaperFamily) {
    let Some(texture) = texture(ui, family) else {
        return;
    };
    let rect = ui.max_rect();
    if rect.width() <= 0.0 || rect.height() <= 0.0 {
        return;
    }
    let tex = texture.size_vec2();
    if tex.x <= 0.0 || tex.y <= 0.0 {
        return;
    }
    let scale = (rect.width() / tex.x).max(rect.height() / tex.y);
    let size = tex * scale;
    let dest = Rect::from_center_size(rect.center(), size);
    ui.painter().with_clip_rect(rect).image(
        texture.id(),
        dest,
        Rect::from_min_max(Pos2::ZERO, pos2(1.0, 1.0)),
        Color32::WHITE,
    );
}

fn texture(ui: &Ui, family: WallpaperFamily) -> Option<TextureHandle> {
    let id = egui::Id::new(("chat-wallpaper", family));
    if let Some(handle) = ui.ctx().data(|data| data.get_temp::<TextureHandle>(id)) {
        return Some(handle);
    }
    let image = decode(family)?;
    let handle = ui
        .ctx()
        .load_texture(family.asset_key(), image, TextureOptions::LINEAR);
    ui.ctx()
        .data_mut(|data| data.insert_temp(id, handle.clone()));
    Some(handle)
}

fn decode(family: WallpaperFamily) -> Option<ColorImage> {
    let bytes = family.png();
    let bitmap = image::load_from_memory(bytes).ok()?.to_rgba8();
    let size = [bitmap.width() as usize, bitmap.height() as usize];
    Some(ColorImage::from_rgba_unmultiplied(size, bitmap.as_raw()))
}

fn rgb_hsl(color: Color32) -> (f32, f32, f32) {
    let r = f32::from(color.r()) / 255.0;
    let g = f32::from(color.g()) / 255.0;
    let b = f32::from(color.b()) / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let lightness = (max + min) / 2.0;
    let delta = max - min;
    if delta < 1e-6 {
        return (0.0, 0.0, lightness);
    }
    let saturation = delta / (1.0 - (2.0 * lightness - 1.0).abs()).max(1e-6);
    let hue = if (max - r).abs() < 1e-6 {
        60.0 * ((g - b) / delta).rem_euclid(6.0)
    } else if (max - g).abs() < 1e-6 {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };
    (hue.rem_euclid(360.0), saturation, lightness)
}

impl WallpaperFamily {
    fn asset_key(self) -> &'static str {
        match self {
            Self::Black => "wallpaper-black",
            Self::Gray => "wallpaper-gray",
            Self::Green => "wallpaper-green",
            Self::Red => "wallpaper-red",
            Self::White => "wallpaper-white",
        }
    }

    fn png(self) -> &'static [u8] {
        match self {
            Self::Black => include_bytes!("../../assets/wallpapers/black/01.png"),
            Self::Gray => include_bytes!("../../assets/wallpapers/gray/01.png"),
            Self::Green => include_bytes!("../../assets/wallpapers/green/01.png"),
            Self::Red => include_bytes!("../../assets/wallpapers/red/01.png"),
            Self::White => include_bytes!("../../assets/wallpapers/white/01.png"),
        }
    }
}

impl ChatWallpaper {
    pub fn resolve(self, palette: &Palette) -> WallpaperFamily {
        match self {
            Self::Auto => family_for(palette),
            Self::Black => WallpaperFamily::Black,
            Self::Gray => WallpaperFamily::Gray,
            Self::Green => WallpaperFamily::Green,
            Self::Red => WallpaperFamily::Red,
            Self::White => WallpaperFamily::White,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Palette;

    #[test]
    fn family_for_follows_light_and_bundled_accents() {
        assert_eq!(family_for(&Palette::light()), WallpaperFamily::White);
        assert_eq!(family_for(&Palette::dark()), WallpaperFamily::Green);

        let mut ristretto = Palette::dark();
        ristretto.window = Color32::from_rgb(0x2c, 0x25, 0x25);
        ristretto.accent = Color32::from_rgb(0xf3, 0x8d, 0x70);
        assert_eq!(family_for(&ristretto), WallpaperFamily::Red);

        let mut nord = Palette::dark();
        nord.window = Color32::from_rgb(0x2e, 0x34, 0x40);
        nord.accent = Color32::from_rgb(0x81, 0xa1, 0xc1);
        assert_eq!(family_for(&nord), WallpaperFamily::Gray);

        let mut tokyo = Palette::dark();
        tokyo.window = Color32::from_rgb(0x1a, 0x1b, 0x26);
        tokyo.accent = Color32::from_rgb(0x7a, 0xa2, 0xf7);
        assert_eq!(family_for(&tokyo), WallpaperFamily::Black);

        let mut catppuccin = Palette::dark();
        catppuccin.window = Color32::from_rgb(0x1e, 0x1e, 0x2e);
        catppuccin.accent = Color32::from_rgb(0x89, 0xb4, 0xfa);
        assert_eq!(family_for(&catppuccin), WallpaperFamily::Black);
    }

    #[test]
    fn auto_defers_to_the_palette_and_manual_wins() {
        assert_eq!(
            ChatWallpaper::Auto.resolve(&Palette::light()),
            WallpaperFamily::White
        );
        assert_eq!(
            ChatWallpaper::Black.resolve(&Palette::light()),
            WallpaperFamily::Black
        );
    }
}
