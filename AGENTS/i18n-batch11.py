"""Lote 11: audio.rs + backend/sticker_import.rs."""
import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
DATA = ROOT / "AGENTS" / "i18n-strings.json"
data = json.loads(DATA.read_text(encoding="utf-8"))

rows = [
    ("ToastAudioDecodeFailed", "Could not decode audio: {error}", "No se pudo decodificar el audio: {error}"),
    ("ToastNoSoundOutput", "No sound output: {error}", "Sin salida de sonido: {error}"),
    ("ToastAudioReadFailed", "Could not read the audio: {error}", "No se pudo leer el audio: {error}"),
    ("ToastAudioDecodeFailed2", "Could not decode the audio: {error}", "No se pudo decodificar el audio: {error}"),
    ("ToastClipEmpty", "The clip is empty", "El clip está vacío"),
    ("ToastNoAudioRecorded", "No audio was recorded", "No se grabó audio"),
    ("ToastMicMissing", "No microphone available: {error}", "No hay micrófono disponible: {error}"),
    ("ToastMicFormat", "The microphone has no supported format: {error}", "El micrófono no tiene un formato compatible: {error}"),
    ("ToastMicOpenFailed", "Could not open the microphone: {error}", "No se pudo abrir el micrófono: {error}"),
    ("ToastMicEmpty", "The microphone did not record any audio", "El micrófono no grabó audio"),
    ("StickerErrMissingPackId", "Missing pack_id. Copy the full signal.art link", "Falta pack_id. Copia el enlace completo de signal.art"),
    ("StickerErrMissingPackKey", "Missing or invalid pack_key. Copy the full signal.art link", "Falta pack_key o no es válido. Copia el enlace completo de signal.art"),
    ("StickerErrIncompleteData", "The sticker data is incomplete", "Los datos del sticker están incompletos"),
    ("StickerErrKeyDerive", "Could not derive the sticker key", "No se pudo derivar la clave del sticker"),
    ("StickerErrKeyMismatch", "The key does not match this pack", "La clave no coincide con este pack"),
    ("StickerErrDecrypt", "Could not decrypt the sticker pack", "No se pudo descifrar el pack de stickers"),
    ("StickerManifestIncomplete", "The sticker manifest is incomplete", "El manifiesto del sticker está incompleto"),
    ("StickerManifestNumber", "The sticker manifest contains an invalid number", "El manifiesto del sticker tiene un número inválido"),
    ("StickerManifestField", "The sticker manifest contains an unknown field", "El manifiesto del sticker tiene un campo desconocido"),
    ("StickerErrCertificate", "Could not read the Signal certificate: {error}", "No se pudo leer el certificado de Signal: {error}"),
    ("StickerErrRequest", "signal.art request failed: {error}", "Falló la solicitud a signal.art: {error}"),
    ("StickerErrResponse", "Could not read the signal.art response: {error}", "No se pudo leer la respuesta de signal.art: {error}"),
    ("StickerErrNoStickers", "This pack contains no stickers", "Este pack no tiene stickers"),
    ("StickerErrOpenFile", "Could not open the file: {error}", "No se pudo abrir el archivo: {error}"),
    ("StickerErrNotArchive", "This file is not a sticker archive: {error}", "Este archivo no es un archivo de stickers: {error}"),
    ("StickerErrNoneRead", "No stickers could be read from this pack", "No se pudo leer ningún sticker de este pack"),
    ("StickerErrWrite", "Could not write the sticker pack: {error}", "No se pudo escribir el pack de stickers: {error}"),
    ("StickerErrCreateFolder", "Could not create the pack folder: {error}", "No se pudo crear la carpeta del pack: {error}"),
    ("StickerErrTooMany", "Too many sticker packs have this name", "Hay demasiados packs de stickers con este nombre"),
    ("KindStickerPackTitle", "Sticker Pack", "Pack de stickers"),
    ("KindStickers", "Stickers", "Stickers"),
]

by_key = {r["key"]: r for r in data["strings"]}
for key, en, es in rows:
    if key in by_key:
        assert by_key[key]["en"] == en and by_key[key]["es"] == es, f"conflicting key {key}"
        continue
    data["strings"].append({"key": key, "en": en, "es": es})

sites = [
    {
        "file": "src/audio.rs",
        "old": "use rodio::",
        "new": "use crate::i18n::{self, Key};\nuse rodio::",
    },
    {
        "name": "audio-fmt",
        "file": "src/audio.rs",
        "fmt_items": [
            {"literal": "Could not decode audio: {error}", "new": "i18n::f(Key::ToastAudioDecodeFailed, &[(\"error\", &error.to_string())])"},
            {"literal": "No sound output: {error}", "new": "i18n::f(Key::ToastNoSoundOutput, &[(\"error\", &error.to_string())])"},
            {"literal": "Could not read the audio: {error}", "new": "i18n::f(Key::ToastAudioReadFailed, &[(\"error\", &error.to_string())])", "all": True},
            {"literal": "Could not decode the audio: {error}", "new": "i18n::f(Key::ToastAudioDecodeFailed2, &[(\"error\", &error.to_string())])"},
            {"literal": "No microphone available: {error}", "new": "i18n::f(Key::ToastMicMissing, &[(\"error\", &error.to_string())])"},
            {"literal": "The microphone has no supported format: {error}", "new": "i18n::f(Key::ToastMicFormat, &[(\"error\", &error.to_string())])"},
            {"literal": "Could not open the microphone: {error}", "new": "i18n::f(Key::ToastMicOpenFailed, &[(\"error\", &error.to_string())])"},
        ],
    },
    {
        "name": "audio-literals",
        "file": "src/audio.rs",
        "literals": [
            {"exact": "The clip is empty", "new": "i18n::t(Key::ToastClipEmpty)"},
            {"exact": "No audio was recorded", "new": "i18n::t(Key::ToastNoAudioRecorded)"},
            {"exact": "The microphone did not record any audio", "new": "i18n::t(Key::ToastMicEmpty)"},
        ],
    },
    {
        "file": "src/backend/sticker_import.rs",
        "old": "use anyhow::{Context, Result, anyhow, bail};",
        "new": "use anyhow::{Context, Result, anyhow, bail};\n\nuse crate::i18n::{self, Key};",
    },
    {
        "name": "sticker-fmt",
        "file": "src/backend/sticker_import.rs",
        "fmt_items": [
            {"literal": "Could not read the Signal certificate: {error}", "new": "i18n::f(Key::StickerErrCertificate, &[(\"error\", &error.to_string())])"},
            {"literal": "signal.art request failed: {error}", "new": "i18n::f(Key::StickerErrRequest, &[(\"error\", &error.to_string())])"},
            {"literal": "Could not read the signal.art response: {error}", "new": "i18n::f(Key::StickerErrResponse, &[(\"error\", &error.to_string())])"},
            {"literal": "Could not open the file: {error}", "new": "i18n::f(Key::StickerErrOpenFile, &[(\"error\", &error.to_string())])"},
            {"literal": "This file is not a sticker archive: {error}", "new": "i18n::f(Key::StickerErrNotArchive, &[(\"error\", &error.to_string())])"},
            {"literal": "Could not write the sticker pack: {error}", "new": "i18n::f(Key::StickerErrWrite, &[(\"error\", &error.to_string())])"},
            {"literal": "Could not create the pack folder: {error}", "new": "i18n::f(Key::StickerErrCreateFolder, &[(\"error\", &error.to_string())])"},
        ],
    },
    {
        "name": "sticker-literals",
        "file": "src/backend/sticker_import.rs",
        "literals": [
            {"exact": "Missing pack_id. Copy the full signal.art link", "new": "i18n::t(Key::StickerErrMissingPackId)"},
            {"exact": "Missing or invalid pack_key. Copy the full signal.art link", "new": "i18n::t(Key::StickerErrMissingPackKey)"},
            {"exact": "The sticker data is incomplete", "new": "i18n::t(Key::StickerErrIncompleteData)"},
            {"exact": "Could not derive the sticker key", "new": "i18n::t(Key::StickerErrKeyDerive)", "all": True},
            {"exact": "The key does not match this pack", "new": "i18n::t(Key::StickerErrKeyMismatch)"},
            {"exact": "Could not decrypt the sticker pack", "new": "i18n::t(Key::StickerErrDecrypt)"},
            {"exact": "The sticker manifest is incomplete", "new": "i18n::t(Key::StickerManifestIncomplete)", "all": True},
            {"exact": "The sticker manifest contains an invalid number", "new": "i18n::t(Key::StickerManifestNumber)"},
            {"exact": "The sticker manifest contains an unknown field", "new": "i18n::t(Key::StickerManifestField)"},
            {"exact": "This pack contains no stickers", "new": "i18n::t(Key::StickerErrNoStickers)"},
            {"exact": "No stickers could be read from this pack", "new": "i18n::t(Key::StickerErrNoneRead)"},
            {"exact": "Too many sticker packs have this name", "new": "i18n::t(Key::StickerErrTooMany)"},
            {"exact": "Sticker Pack", "new": "i18n::t(Key::KindStickerPackTitle)"},
            {"exact": "Stickers", "new": "i18n::t(Key::KindStickers)"},
        ],
    },
]

data["sites"].extend(sites)
DATA.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
print(f"rows: {len(data['strings'])}, sites: {len(data['sites'])}")
