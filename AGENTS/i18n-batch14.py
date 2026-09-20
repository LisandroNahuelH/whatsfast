"""Lote 14: archive/encryption.rs (llavero) + notify/windows.rs."""
import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
DATA = ROOT / "AGENTS" / "i18n-strings.json"
data = json.loads(DATA.read_text(encoding="utf-8"))

rows = [
    ("ArchiveErrBadKeyringKey", "The archive key in the OS keyring is invalid", "La clave del archivo en el llavero del sistema no es válida"),
    ("ArchiveErrNoParentDir", "Archive has no parent directory", "El archivo no tiene carpeta contenedora"),
    ("ArchiveErrUnlockKeyring", "Unlock your OS keyring and restart WhatsFast", "Desbloquea el llavero del sistema y reinicia WhatsFast"),
    ("ArchiveErrKeyringOpen", "The OS keyring could not open WhatsFast's archive key", "El llavero del sistema no pudo abrir la clave del archivo de WhatsFast"),
    ("ArchiveErrKeySaveMigrated", "Could not save the migrated archive key in the OS keyring", "No se pudo guardar la clave migrada del archivo en el llavero del sistema"),
    ("ArchiveErrKeyMissing", "The archive is encrypted but its OS keyring key is missing. Restore the original keyring; the archive has not been changed", "El archivo está cifrado pero falta su clave en el llavero del sistema. Restaura el llavero original; el archivo no cambió"),
    ("ArchiveErrKeyGenerate", "Could not generate an archive key", "No se pudo generar una clave para el archivo"),
    ("ArchiveErrKeySave", "Could not save the archive key in the OS keyring", "No se pudo guardar la clave del archivo en el llavero del sistema"),
    ("ArchiveErrKeyVerify", "Could not verify the saved archive key", "No se pudo verificar la clave guardada del archivo"),
    ("ArchiveErrKeyNotRetained", "The OS keyring did not retain the archive key", "El llavero del sistema no conservó la clave del archivo"),
    ("ArchiveErrNoCipher", "This build does not support encrypted archives", "Esta compilación no admite archivos cifrados"),
    ("ArchiveErrUnlockFailed", "The archive could not be unlocked with its OS keyring key", "No se pudo desbloquear el archivo con su clave del llavero del sistema"),
    ("ArchiveErrClosePrograms", "Close other programs using the archive before migrating it", "Cierra los otros programas que usan el archivo antes de migrarlo"),
    ("ArchiveErrPathNotUtf8", "Archive path is not UTF-8", "La ruta del archivo no es UTF-8"),
    ("ArchiveErrIntegrity", "The encrypted archive failed its integrity check", "El archivo cifrado no pasó la verificación de integridad"),
    ("ArchiveErrReplaceFailed", "Could not replace the archive with its encrypted copy", "No se pudo reemplazar el archivo por su copia cifrada"),
    ("NotifyErrIdentity", "notification identity unavailable: {error}", "identidad de notificación no disponible: {error}"),
]

by_key = {r["key"]: r for r in data["strings"]}
for key, en, es in rows:
    if key in by_key:
        assert by_key[key]["en"] == en and by_key[key]["es"] == es, f"conflicting key {key}"
        continue
    data["strings"].append({"key": key, "en": en, "es": es})

sites = [
    {
        "file": "src/archive/encryption.rs",
        "old": "use anyhow::{",
        "new": "use crate::i18n::{self, Key};\nuse anyhow::{",
    },
    {
        "name": "archive-literals",
        "file": "src/archive/encryption.rs",
        "literals": [
            {"exact": "The archive key in the OS keyring is invalid", "new": "i18n::t(Key::ArchiveErrBadKeyringKey)"},
            {"exact": "Archive has no parent directory", "new": "i18n::t(Key::ArchiveErrNoParentDir)"},
            {"exact": "Unlock your OS keyring and restart WhatsFast", "new": "i18n::t(Key::ArchiveErrUnlockKeyring)", "all": True},
            {"exact": "The OS keyring could not open WhatsFast's archive key", "new": "i18n::t(Key::ArchiveErrKeyringOpen)", "all": True},
            {"exact": "Could not save the migrated archive key in the OS keyring", "new": "i18n::t(Key::ArchiveErrKeySaveMigrated)"},
            {"starts_with": "The archive is encrypted but its OS keyring key is missing", "new": "i18n::t(Key::ArchiveErrKeyMissing)"},
            {"exact": "Could not generate an archive key", "new": "i18n::t(Key::ArchiveErrKeyGenerate)"},
            {"exact": "Could not save the archive key in the OS keyring", "new": "i18n::t(Key::ArchiveErrKeySave)"},
            {"exact": "Could not verify the saved archive key", "new": "i18n::t(Key::ArchiveErrKeyVerify)"},
            {"exact": "The OS keyring did not retain the archive key", "new": "i18n::t(Key::ArchiveErrKeyNotRetained)"},
            {"exact": "This build does not support encrypted archives", "new": "i18n::t(Key::ArchiveErrNoCipher)", "all": True},
            {"exact": "The archive could not be unlocked with its OS keyring key", "new": "i18n::t(Key::ArchiveErrUnlockFailed)"},
            {"exact": "Close other programs using the archive before migrating it", "new": "i18n::t(Key::ArchiveErrClosePrograms)"},
            {"exact": "Archive path is not UTF-8", "new": "i18n::t(Key::ArchiveErrPathNotUtf8)"},
            {"exact": "The encrypted archive failed its integrity check", "new": "i18n::t(Key::ArchiveErrIntegrity)"},
            {"exact": "Could not replace the archive with its encrypted copy", "new": "i18n::t(Key::ArchiveErrReplaceFailed)"},
        ],
    },
    {
        "file": "src/notify/windows.rs",
        "old": "use winrt_notification::{IconCrop, Toast};",
        "new": "use winrt_notification::{IconCrop, Toast};\n\nuse crate::i18n::{self, Key};",
    },
    {
        "name": "notifywin-fmt",
        "file": "src/notify/windows.rs",
        "fmt_items": [
            {"literal": "notification identity unavailable: {error}", "new": "i18n::f(Key::NotifyErrIdentity, &[(\"error\", &error.to_string())])"},
        ],
    },
]

data["sites"].extend(sites)
DATA.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
print(f"rows: {len(data['strings'])}, sites: {len(data['sites'])}")
