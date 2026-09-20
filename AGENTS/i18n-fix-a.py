"""Etapa 1 del plan de fix del consejo (I18N-VERIFY-2).

- safe_literal por declaracion en los dos scripts (regla exacta del asiento 2).
- El generador no marca un sitio sin reemplazos (las 3 rutas).
- Exclusiones documentadas en el escaner (wire_name + TLD).
- Filas nuevas + ES alineado al glosario + sites nuevos en el JSON.
- Borra las firmas falsas del state y re-corre el generador.
"""
import hashlib
import json
import pathlib
import re
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
GEN = ROOT / "AGENTS" / "i18n-gen.py"
SCAN = ROOT / "AGENTS" / "i18n-residual.py"
DATA = ROOT / "AGENTS" / "i18n-strings.json"
STATE = ROOT / "AGENTS" / "i18n-sites-state.json"

OLD_SAFE = '''def safe_literal(text: str, position: int) -> bool:
    """False for byte strings, raw strings, and const/static initializers.

    Those carry protocol bytes or compile-time data; translating the text
    inside them breaks the wire format or does not compile.
    """
    prefix = text[max(0, position - 3) : position]
    if prefix.endswith(("b", "r", "br", "rb")):
        return False
    window = text[max(0, position - 4000) : position]
    const_at = max(window.rfind("const "), window.rfind("static "))
    if const_at >= 0 and "}" not in window[const_at:]:
        return False
    return True'''

NEW_SAFE = '''DECL = re.compile(
    r"(?s)^(pub\\([^)]*\\)\\s+|pub\\s+)?(unsafe\\s+)?(const|static)\\s+[A-Za-z_][A-Za-z_0-9]*\\s*:"
)


def safe_literal(text: str, position: int) -> bool:
    """False for byte strings, raw strings, and const/static initializers.

    Those carry protocol bytes or compile-time data; translating the text
    inside them breaks the wire format or does not compile. A literal belongs
    to a const/static only when the statement holding it *starts* with that
    declaration: a `&'static str` in a signature, or an `=>` further up the
    line, must not hide interface text.
    """
    prefix = text[max(0, position - 3) : position]
    if prefix.endswith(("b", "r", "br", "rb")):
        return False
    start = max(text.rfind(";", 0, position), text.rfind("}", 0, position))
    return not DECL.match(text[start + 1 : position])'''

SCAN_EXCLUDE_NOTE = '''# Text collisions that are not interface: the protocol's own wire value and a
# TLD inside the link parser. Translating either one breaks the wire format or
# the parser, so the scan skips them by (file, text).
SCAN_EXCLUDE = {
    ("src/privacy.rs", "online"),
    ("src/markup.rs", "online"),
}


def safe_literal'''


def read(path):
    return path.read_text(encoding="utf-8")


def write(path, text):
    path.write_text(text, encoding="utf-8", newline="\n")


def patch_scripts():
    gen = read(GEN)
    assert OLD_SAFE in gen, "safe_literal de gen.py no encontrado"
    gen = gen.replace(OLD_SAFE, NEW_SAFE)
    # fmt_items: marcar solo con reemplazos
    old = '''        if "fmt_items" in site:
            error = fmt_sites(path, site.get("name", site["file"]), site["fmt_items"], [])
            if error:
                print(error, file=sys.stderr)
                return 1
            state.add(sig)
            applied += 1
            continue
        if "literals" in site:
            error = literal_sites(
                path, site.get("name", site["file"]), site["literals"], []
            )
            if error:
                print(error, file=sys.stderr)
                return 1
            state.add(sig)
            applied += 1
            continue'''
    new = '''        if "fmt_items" in site:
            changed = []
            error = fmt_sites(
                path, site.get("name", site["file"]), site["fmt_items"], changed
            )
            if error:
                print(error, file=sys.stderr)
                return 1
            if changed:
                state.add(sig)
                applied += 1
            else:
                unmarked += 1
                print(
                    f"note: {site.get('name', site['file'])}: nothing replaced; "
                    "left unmarked for the next run"
                )
            continue
        if "literals" in site:
            changed = []
            error = literal_sites(
                path, site.get("name", site["file"]), site["literals"], changed
            )
            if error:
                print(error, file=sys.stderr)
                return 1
            if changed:
                state.add(sig)
                applied += 1
            else:
                unmarked += 1
                print(
                    f"note: {site.get('name', site['file'])}: nothing replaced; "
                    "left unmarked for the next run"
                )
            continue'''
    assert old in gen, "bloque fmt/literals de gen.py no encontrado"
    gen = gen.replace(old, new)
    old = '''        if count == 0:
            # rustfmt reflows inserted code, so an applied site may no longer
            # match its own `new` text. Treat it as applied and say so.
            print(
                f"note: anchor gone in {site['file']}: {old[:60]!r} (assumed applied)",
                file=sys.stderr,
            )
            state.add(sig)
            skipped += 1
            continue'''
    new = '''        if count == 0:
            if new in text:
                # rustfmt reflows inserted code, so an applied site may no
                # longer match its own `old` text. The replacement is there:
                # the site is applied.
                state.add(sig)
                skipped += 1
            else:
                unmarked += 1
                print(
                    f"note: anchor gone in {site['file']}: {old[:60]!r} and no "
                    "replacement found; left unmarked for the next run",
                    file=sys.stderr,
                )
            continue'''
    assert old in gen, "bloque anchor-gone de gen.py no encontrado"
    gen = gen.replace(old, new)
    old = "    applied = 0\n    skipped = 0\n"
    assert old in gen, "contadores de gen.py no encontrados"
    gen = gen.replace(old, "    applied = 0\n    skipped = 0\n    unmarked = 0\n")
    old = '    print(f"call sites applied: {applied}, already applied: {skipped}")'
    assert old in gen
    gen = gen.replace(
        old,
        '    print(\n'
        '        f"call sites applied: {applied}, already applied: {skipped}, '
        'unmarked: {unmarked}"\n'
        '    )',
    )
    write(GEN, gen)

    scan = read(SCAN)
    assert OLD_SAFE in scan, "safe_literal del escaner no encontrado"
    scan = scan.replace(OLD_SAFE, NEW_SAFE)
    assert "\ndef safe_literal" in scan
    scan = scan.replace("\ndef safe_literal", "\n" + SCAN_EXCLUDE_NOTE, 1)
    old = '''                if content not in english:
                    continue
                if not safe_literal'''
    new = '''                if content not in english:
                    continue
                if (rel, content) in SCAN_EXCLUDE:
                    continue
                if not safe_literal'''
    assert old in scan
    scan = scan.replace(old, new)
    write(SCAN, scan)


ROWS = {
    "ChatEdited": ("edited", "editado"),
    "PollVoterOne": ("1 voter", "1 voto"),
    "PollVoterMany": ("{count} voters", "{count} votos"),
    "SearchClear": ("Clear", "Limpiar"),
}

ES_FIXES = {
    "MenuView": "Ver",
    "LoginTryAgain": "Reintentar",
    "ThemeErrReload": "No se pudieron cargar los temas personalizados. "
    "Ejecuta whatsfast reload-themes para volver a intentarlo.",
    "ThemeErrFolderUnreadable": "No se pudo leer la carpeta de temas. "
    "Consulta el registro para más detalles.",
    "ThemeErrOmarchyLoad": "No se pudo cargar la paleta de Omarchy. "
    "Se mantiene la última apariencia válida. Consulta el registro para más detalles.",
    "ThemeErrSelectedUnavailable": "El tema seleccionado no está disponible. "
    "Se mantiene la última apariencia válida. Consulta el registro para más detalles.",
    "SettingsStorageVideos": "Videos",
    "ChatUnpinMessage": "Dejar de fijar",
    "ToastUnpinned": "Se dejó de fijar el mensaje",
    "ChatListNoPinnedHint": "Fija un mensaje desde el chat o el visor.",
    "DialogAddPack": "Añadir un paquete de stickers",
    "DialogStickerPacks": "Paquetes de stickers",
    "ToastPackAdded": 'Se añadió el paquete de stickers "{name}"',
    "ToastPackFailed": "No se pudo añadir el paquete de stickers: {error}",
    "KindStickerPack": "paquete de stickers",
    "KindStickerPackTitle": "Paquete de stickers",
    "StickerErrKeyMismatch": "La clave no coincide con este paquete",
    "StickerErrDecrypt": "No se pudo descifrar el paquete de stickers",
    "StickerErrNoStickers": "Este paquete no tiene stickers",
    "StickerErrNoneRead": "No se pudo leer ningún sticker de este paquete",
    "StickerErrWrite": "No se pudo escribir el paquete de stickers: {error}",
    "StickerErrCreateFolder": "No se pudo crear la carpeta del paquete: {error}",
    "StickerErrTooMany": "Hay demasiados paquetes de stickers con este nombre",
}

NEW_SITES = [
    {
        "name": "chat-edited",
        "file": "src/ui/conversation.rs",
        "literals": [
            {"exact": "edited", "new": "i18n::t(I18nKey::ChatEdited)", "all": True}
        ],
    },
    {
        "name": "dialogs-online",
        "file": "src/ui/dialogs.rs",
        "literals": [{"exact": "online", "new": "i18n::t(Key::ChatOnline)"}],
    },
    {
        "name": "widgets-search-clear",
        "file": "src/ui/widgets.rs",
        "literals": [{"exact": "Clear", "new": "i18n::t(Key::SearchClear)"}],
    },
]


def signature(site):
    return hashlib.sha256(
        json.dumps(site, sort_keys=True, ensure_ascii=False).encode("utf-8")
    ).hexdigest()[:16]


def main():
    patch_scripts()
    print("scripts: safe_literal por declaracion + state sin marcas falsas")

    data = json.loads(read(DATA))
    rows = data["strings"]
    keys = [r["key"] for r in rows]
    for key, (en, es) in ROWS.items():
        if key in keys:
            print(f"row ya presente: {key}")
            continue
        prefix = re.match(r"[A-Z][a-z]+", key).group(0)
        last = max(i for i, r in enumerate(rows) if r["key"].startswith(prefix))
        rows.insert(last + 1, {"key": key, "en": en, "es": es})
        print(f"row nueva: {key} = {es!r}")
    fixed = 0
    for row in rows:
        if row["key"] in ES_FIXES and row["es"] != ES_FIXES[row["key"]]:
            row["es"] = ES_FIXES[row["key"]]
            fixed += 1
    print(f"ES alineado al glosario: {fixed} filas")

    sites = data["sites"]
    names = {s.get("name") for s in sites}
    # Model: los needles de media no se tocan por texto (los consts no pueden
    # llamar a t()); se convierten a funciones a mano. Se quitan del sitio.
    dropped = 0
    for site in sites:
        if site.get("name") == "model-literals":
            keep = [
                n
                for n in site["literals"]
                if "ChatRetryFile" not in n.get("new", "")
                and "ChatFileGone" not in n.get("new", "")
            ]
            dropped = len(site["literals"]) - len(keep)
            site["literals"] = keep
    print(f"model-literals: {dropped} needles de media quitados (van a funciones)")
    for site in NEW_SITES:
        if site["name"] in names:
            print(f"site ya presente: {site['name']}")
            continue
        sites.append(site)
        print(f"site nuevo: {site['name']} -> {site['file']}")

    write(DATA, json.dumps(data, indent=2, ensure_ascii=False) + "\n")

    # El state guarda hashes: se borran los de los sitios que hay que re-correr.
    state = set(json.loads(read(STATE))) if STATE.exists() else set()
    stale = {signature(s) for s in sites if s.get("name") in ("privacy-literals",)}
    # Firmas viejas conocidas (vigente y huerfana del informe del consejo).
    stale |= {"e74064022e89b20f"}
    before = len(state)
    state -= stale
    write(STATE, json.dumps(sorted(state), indent=0) + "\n")
    print(f"state: {before} -> {len(state)} firmas (borradas {before - len(state)})")

    result = subprocess.run(
        [sys.executable, str(GEN)], capture_output=True, text=True, cwd=str(ROOT)
    )
    tail = [
        line
        for line in result.stdout.splitlines()
        if "note:" not in line and "missing:" not in line
    ]
    print("\n".join(tail))
    if result.returncode != 0:
        print(result.stderr[-3000:])
    return result.returncode


if __name__ == "__main__":
    raise SystemExit(main())
