"""Lote 5: panel de búsqueda (pane) + picker (emoji/GIF/stickers)."""
import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
DATA = ROOT / "AGENTS" / "i18n-strings.json"
data = json.loads(DATA.read_text(encoding="utf-8"))

rows = [
    # panel de búsqueda
    ("PaneSearchTitle", "Search messages", "Buscar mensajes"),
    ("PaneFilterByDate", "Filter by date", "Filtrar por fecha"),
    ("PaneSearchHint", "Search", "Buscar"),
    ("PaneSearchWith", "Search messages with {title}", "Buscar mensajes con {title}"),
    ("PaneNoMessages", "No messages found", "No se encontraron mensajes"),
    ("PaneTryAnother", "Try another word or pick a different day.", "Prueba con otra palabra o elige otro día."),
    # picker: pestañas
    ("PickerTabEmoji", "Emoji", "Emoji"),
    ("PickerTabGif", "GIF", "GIF"),
    ("PickerTabStickers", "Stickers", "Stickers"),
    # picker: grupos de emoji (un solo origen: group_name)
    ("PickerGroupSmileys", "Smileys & Emotion", "Caritas y emociones"),
    ("PickerGroupPeople", "People & Body", "Personas y cuerpos"),
    ("PickerGroupAnimals", "Animals & Nature", "Animales y naturaleza"),
    ("PickerGroupFood", "Food & Drink", "Comida y bebida"),
    ("PickerGroupTravel", "Travel & Places", "Viajes y lugares"),
    ("PickerGroupActivities", "Activities", "Actividades"),
    ("PickerGroupObjects", "Objects", "Objetos"),
    ("PickerGroupSymbols", "Symbols", "Símbolos"),
    ("PickerGroupFlags", "Flags", "Banderas"),
    ("PickerRecent", "Recent", "Recientes"),
    ("PickerFrequentlyUsed", "Frequently Used", "Usados frecuentemente"),
    ("PickerNothingMatches", "Nothing matches", "Sin coincidencias"),
    ("PickerSearchEmoji", "Search emoji", "Buscar emoji"),
    # picker: GIF
    ("PickerSearchGifs", "Search GIFs via GIPHY", "Buscar GIF en GIPHY"),
    ("PickerSearching", "Searching…", "Buscando…"),
    ("PickerGifEmpty", "Search for a GIF or browse trending results.", "Busca un GIF o mira los resultados del momento."),
    ("PickerGiphyKeyRejected", "This GIPHY API key was rejected. Create a free key at developers.giphy.com and paste it here. It is saved in your settings.", "GIPHY rechazó esta clave de API. Crea una clave gratis en developers.giphy.com y pégala aquí. Se guarda en tus ajustes."),
    ("PickerGiphyKeyNeeded", "GIF search needs a GIPHY API key. Create a free key at developers.giphy.com and paste it here. It is saved in your settings.", "La búsqueda de GIF necesita una clave de API de GIPHY. Crea una clave gratis en developers.giphy.com y pégala aquí. Se guarda en tus ajustes."),
    # picker: stickers
    ("PickerStickersLoading", "Loading your stickers…", "Cargando tus stickers…"),
    ("PickerStickersEmpty", "Recent stickers appear here. Right-click one to save it. To import a pack, paste a signal.art link or open a .wastickers file.", "Los stickers recientes aparecen aquí. Haz clic derecho en uno para guardarlo. Para importar un paquete, pega un enlace de signal.art o abre un archivo .wastickers."),
    ("PickerStickersSaved", "Saved", "Guardados"),
    ("PickerStickerRemovePack", "Remove this pack", "Quitar este paquete"),
    ("PickerFindPacks", "Find packs", "Buscar paquetes"),
    ("CommonOpenFile", "Open file", "Abrir archivo"),
    ("PickerPasteLink", "Paste a signal.art link", "Pega un enlace de signal.art"),
    ("PickerBrowseStickers", "Browse signalstickers.org", "Explorar signalstickers.org"),
    ("PickerImportingPack", "Importing the pack…", "Importando el paquete…"),
    ("PickerRemoveFromSaved", "Remove from saved", "Quitar de guardados"),
    ("PickerSaveSticker", "Save sticker", "Guardar sticker"),
]

existing = {r["key"] for r in data["strings"]}
for key, en, es in rows:
    assert key not in existing, f"duplicate key {key}"
    data["strings"].append({"key": key, "en": en, "es": es})

sites = [
    {
        "file": "src/ui/picker.rs",
        "old": "use crate::app::App;",
        "new": "use crate::app::App;\nuse crate::i18n::{self, Key};",
    },
    # --- pane: expresión con format!
    {
        "file": "src/ui/pane.rs",
        "old": "                format!(\"Search messages with {title}\"),",
        "new": "                i18n::f(Key::PaneSearchWith, &[(\"title\", title.as_str())]),",
    },
    # --- picker: categorías como función (t() no vive en const)
    {
        "file": "src/ui/picker.rs",
        "old": """fn group_name(group: emojis::Group) -> &'static str {
    match group {
        emojis::Group::SmileysAndEmotion => "Smileys & Emotion",
        emojis::Group::PeopleAndBody => "People & Body",
        emojis::Group::AnimalsAndNature => "Animals & Nature",
        emojis::Group::FoodAndDrink => "Food & Drink",
        emojis::Group::TravelAndPlaces => "Travel & Places",
        emojis::Group::Activities => "Activities",
        emojis::Group::Objects => "Objects",
        emojis::Group::Symbols => "Symbols",
        emojis::Group::Flags => "Flags",
    }
}""",
        "new": """fn group_name(group: emojis::Group) -> &'static str {
    match group {
        emojis::Group::SmileysAndEmotion => i18n::t(Key::PickerGroupSmileys),
        emojis::Group::PeopleAndBody => i18n::t(Key::PickerGroupPeople),
        emojis::Group::AnimalsAndNature => i18n::t(Key::PickerGroupAnimals),
        emojis::Group::FoodAndDrink => i18n::t(Key::PickerGroupFood),
        emojis::Group::TravelAndPlaces => i18n::t(Key::PickerGroupTravel),
        emojis::Group::Activities => i18n::t(Key::PickerGroupActivities),
        emojis::Group::Objects => i18n::t(Key::PickerGroupObjects),
        emojis::Group::Symbols => i18n::t(Key::PickerGroupSymbols),
        emojis::Group::Flags => i18n::t(Key::PickerGroupFlags),
    }
}""",
    },
    {
        "file": "src/ui/picker.rs",
        "old": """/// Category tabs under the reaction emoji grid, WhatsApp-style.
const CATEGORIES: &[(Option<emojis::Group>, &str, &str)] = &[
    (None, "🕒", "Frequently Used"),
    (
        Some(emojis::Group::SmileysAndEmotion),
        "😀",
        "Smileys & Emotion",
    ),
    (Some(emojis::Group::PeopleAndBody), "👋", "People & Body"),
    (
        Some(emojis::Group::AnimalsAndNature),
        "🐻",
        "Animals & Nature",
    ),
    (Some(emojis::Group::FoodAndDrink), "🍔", "Food & Drink"),
    (
        Some(emojis::Group::TravelAndPlaces),
        "🚗",
        "Travel & Places",
    ),
    (Some(emojis::Group::Activities), "⚽", "Activities"),
    (Some(emojis::Group::Objects), "💡", "Objects"),
    (Some(emojis::Group::Symbols), "🔣", "Symbols"),
    (Some(emojis::Group::Flags), "🏁", "Flags"),
];""",
        "new": """/// Category tabs under the reaction emoji grid, WhatsApp-style. The labels
/// come from the same place the grid headers do, so a jump matches by text.
fn categories() -> [(Option<emojis::Group>, &'static str, &'static str); 10] {
    [
        (None, "🕒", i18n::t(Key::PickerFrequentlyUsed)),
        (
            Some(emojis::Group::SmileysAndEmotion),
            "😀",
            group_name(emojis::Group::SmileysAndEmotion),
        ),
        (
            Some(emojis::Group::PeopleAndBody),
            "👋",
            group_name(emojis::Group::PeopleAndBody),
        ),
        (
            Some(emojis::Group::AnimalsAndNature),
            "🐻",
            group_name(emojis::Group::AnimalsAndNature),
        ),
        (
            Some(emojis::Group::FoodAndDrink),
            "🍔",
            group_name(emojis::Group::FoodAndDrink),
        ),
        (
            Some(emojis::Group::TravelAndPlaces),
            "🚗",
            group_name(emojis::Group::TravelAndPlaces),
        ),
        (
            Some(emojis::Group::Activities),
            "⚽",
            group_name(emojis::Group::Activities),
        ),
        (
            Some(emojis::Group::Objects),
            "💡",
            group_name(emojis::Group::Objects),
        ),
        (
            Some(emojis::Group::Symbols),
            "🔣",
            group_name(emojis::Group::Symbols),
        ),
        (
            Some(emojis::Group::Flags),
            "🏁",
            group_name(emojis::Group::Flags),
        ),
    ]
}""",
    },
    {
        "file": "src/ui/picker.rs",
        "old": """    CATEGORIES
        .iter()
        .copied()
        .filter(move |(group, _, _)| group.is_some() || has_recent)""",
        "new": """    categories()
        .into_iter()
        .filter(move |(group, _, _)| group.is_some() || has_recent)""",
    },
    {
        "name": "picker-literals",
        "file": "src/ui/picker.rs",
        "literals": [
            {"exact": "Emoji", "new": "i18n::t(Key::PickerTabEmoji)"},
            {"exact": "GIF", "new": "i18n::t(Key::PickerTabGif)"},
            {"exact": "Stickers", "new": "i18n::t(Key::PickerTabStickers)"},
            {"exact": "Nothing matches", "new": "i18n::t(Key::PickerNothingMatches)"},
            {"exact": "Recent", "new": "i18n::t(Key::PickerRecent)", "all": True},
            {"exact": "Frequently Used", "new": "i18n::t(Key::PickerFrequentlyUsed)", "all": True},
            {"exact": "Search emoji", "new": "i18n::t(Key::PickerSearchEmoji)"},
            {"exact": "Search GIFs via GIPHY", "new": "i18n::t(Key::PickerSearchGifs)"},
            {"exact": "Searching…", "new": "i18n::t(Key::PickerSearching)"},
            {"starts_with": "Search for a GIF or browse", "new": "i18n::t(Key::PickerGifEmpty)"},
            {"starts_with": "This GIPHY API key was rejected.", "new": "i18n::t(Key::PickerGiphyKeyRejected)"},
            {"starts_with": "GIF search needs a GIPHY API key.", "new": "i18n::t(Key::PickerGiphyKeyNeeded)"},
            {"exact": "GIPHY API key", "new": "i18n::t(Key::SettingsGiphyKey)"},
            {"exact": "Loading your stickers…", "new": "i18n::t(Key::PickerStickersLoading)"},
            {"starts_with": "Recent stickers appear here.", "new": "i18n::t(Key::PickerStickersEmpty)"},
            {"exact": "Saved", "new": "i18n::t(Key::PickerStickersSaved)"},
            {"exact": "Remove this pack", "new": "i18n::t(Key::PickerStickerRemovePack)"},
            {"exact": "Find packs", "new": "i18n::t(Key::PickerFindPacks)", "all": True},
            {"exact": "Open file", "new": "i18n::t(Key::CommonOpenFile)", "all": True},
            {"exact": "Paste a signal.art link", "new": "i18n::t(Key::PickerPasteLink)"},
            {"exact": "Browse signalstickers.org", "new": "i18n::t(Key::PickerBrowseStickers)"},
            {"exact": "Importing the pack…", "new": "i18n::t(Key::PickerImportingPack)"},
            {"exact": "Remove from saved", "new": "i18n::t(Key::PickerRemoveFromSaved)", "all": True},
            {"exact": "Save sticker", "new": "i18n::t(Key::PickerSaveSticker)"},
        ],
    },
    {
        "name": "pane-literals",
        "file": "src/ui/pane.rs",
        "literals": [
            {"exact": "Close", "new": "i18n::t(Key::CommonClose)"},
            {"exact": "Search messages", "new": "i18n::t(Key::PaneSearchTitle)"},
            {"exact": "Filter by date", "new": "i18n::t(Key::PaneFilterByDate)"},
            {"exact": "Search", "new": "i18n::t(Key::PaneSearchHint)"},
            {"exact": "No messages found", "new": "i18n::t(Key::PaneNoMessages)"},
            {"starts_with": "Try another word or pick", "new": "i18n::t(Key::PaneTryAnother)"},
            {"exact": "Next month", "new": "i18n::t(Key::ScheduleNextMonth)"},
            {"exact": "Previous month", "new": "i18n::t(Key::SchedulePreviousMonth)"},
        ],
    },
]

data["sites"].extend(sites)
DATA.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
print(f"rows: {len(data['strings'])}, sites: {len(data['sites'])}")
