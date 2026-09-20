"""Lote 12: menus de macOS + restos (chats, conversation, app)."""
import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
DATA = ROOT / "AGENTS" / "i18n-strings.json"
data = json.loads(DATA.read_text(encoding="utf-8"))

rows = [
    ("MenuAbout", "About WhatsFast", "Acerca de WhatsFast"),
    ("MenuSettings", "Settings…", "Ajustes…"),
    ("MenuHide", "Hide WhatsFast", "Ocultar WhatsFast"),
    ("MenuQuit", "Quit WhatsFast", "Salir de WhatsFast"),
    ("MenuFile", "File", "Archivo"),
    ("MenuNewContact", "New Contact…", "Contacto nuevo…"),
    ("MenuCloseWindow", "Close Window", "Cerrar ventana"),
    ("MenuEdit", "Edit", "Edición"),
    ("MenuUndo", "Undo", "Deshacer"),
    ("MenuRedo", "Redo", "Rehacer"),
    ("MenuCut", "Cut", "Cortar"),
    ("MenuCopy", "Copy", "Copiar"),
    ("MenuPaste", "Paste", "Pegar"),
    ("MenuSelectAll", "Select All", "Seleccionar todo"),
    ("MenuFind", "Find…", "Buscar…"),
    ("MenuView", "View", "Visualización"),
    ("MenuToggleSidebar", "Toggle Sidebar", "Mostrar u ocultar la barra lateral"),
    ("MenuZoomIn", "Zoom In", "Acercar"),
    ("MenuZoomOut", "Zoom Out", "Alejar"),
    ("MenuActualSize", "Actual Size", "Tamaño real"),
    ("MenuShowWhatsFast", "Show WhatsFast", "Mostrar WhatsFast"),
    ("MenuHelp", "Help", "Ayuda"),
    ("MenuKeyboardShortcuts", "Keyboard Shortcuts", "Atajos de teclado"),
    ("MenuWhatsFastHelp", "WhatsFast Help", "Ayuda de WhatsFast"),
]

by_key = {r["key"]: r for r in data["strings"]}
for key, en, es in rows:
    if key in by_key:
        assert by_key[key]["en"] == en and by_key[key]["es"] == es, f"conflicting key {key}"
        continue
    data["strings"].append({"key": key, "en": en, "es": es})

sites = [
    {
        "file": "src/macos.rs",
        "old": "use crate::model::{Action, Dialog, Page};",
        "new": "use crate::i18n::{self, Key};\nuse crate::model::{Action, Dialog, Page};",
    },
    {
        "name": "macos-menus",
        "file": "src/macos.rs",
        "literals": [
            {"exact": "About WhatsFast", "new": "i18n::t(Key::MenuAbout)"},
            {"exact": "Settings…", "new": "i18n::t(Key::MenuSettings)"},
            {"exact": "Hide WhatsFast", "new": "i18n::t(Key::MenuHide)"},
            {"exact": "Quit WhatsFast", "new": "i18n::t(Key::MenuQuit)"},
            {"exact": "File", "new": "i18n::t(Key::MenuFile)"},
            {"exact": "New Contact…", "new": "i18n::t(Key::MenuNewContact)"},
            {"exact": "Close Window", "new": "i18n::t(Key::MenuCloseWindow)"},
            {"exact": "Edit", "new": "i18n::t(Key::MenuEdit)"},
            {"exact": "Undo", "new": "i18n::t(Key::MenuUndo)"},
            {"exact": "Redo", "new": "i18n::t(Key::MenuRedo)"},
            {"exact": "Cut", "new": "i18n::t(Key::MenuCut)"},
            {"exact": "Copy", "new": "i18n::t(Key::MenuCopy)"},
            {"exact": "Paste", "new": "i18n::t(Key::MenuPaste)"},
            {"exact": "Select All", "new": "i18n::t(Key::MenuSelectAll)"},
            {"exact": "Find…", "new": "i18n::t(Key::MenuFind)"},
            {"exact": "View", "new": "i18n::t(Key::MenuView)"},
            {"exact": "Toggle Sidebar", "new": "i18n::t(Key::MenuToggleSidebar)"},
            {"exact": "Zoom In", "new": "i18n::t(Key::MenuZoomIn)"},
            {"exact": "Zoom Out", "new": "i18n::t(Key::MenuZoomOut)"},
            {"exact": "Actual Size", "new": "i18n::t(Key::MenuActualSize)"},
            {"exact": "Window", "new": "i18n::t(Key::SettingsSectionWindow)"},
            {"exact": "Zoom", "new": "i18n::t(Key::SettingsZoomLabel)"},
            {"exact": "Show WhatsFast", "new": "i18n::t(Key::MenuShowWhatsFast)"},
            {"exact": "Help", "new": "i18n::t(Key::MenuHelp)"},
            {"exact": "Keyboard Shortcuts", "new": "i18n::t(Key::MenuKeyboardShortcuts)"},
            {"exact": "WhatsFast Help", "new": "i18n::t(Key::MenuWhatsFastHelp)"},
        ],
    },
    {
        "file": "src/ui/chats.rs",
        "old": "                \"typing…\".to_owned()",
        "new": "                i18n::t(Key::TypingShort).to_owned()",
    },
    {
        "file": "src/ui/conversation.rs",
        "old": "                            i18n::t(Key::ChatInfo),\n                            \"Pin to top\",",
        "new": "                            i18n::t(Key::ChatInfo),\n                            i18n::t(Key::ChatPinToTop),",
    },
    {
        "file": "src/ui/conversation.rs",
        "old": "            label.push(\"GIF\".to_owned());",
        "new": "            label.push(i18n::t(Key::KindGif).to_owned());",
    },
    {
        "file": "src/app.rs",
        "old": "self.toast_error(format!(\"Could not record: {error}\"))",
        "new": "self.toast_error(i18n::f(\n                Key::ToastCouldNotRecord,\n                &[(\"error\", &error.to_string())],\n            ))",
    },
]

data["sites"].extend(sites)
DATA.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
print(f"rows: {len(data['strings'])}, sites: {len(data['sites'])}")
