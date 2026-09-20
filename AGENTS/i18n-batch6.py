"""Lote 6: diálogos, bandeja y notificaciones. + helper i18n::count."""
import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
DATA = ROOT / "AGENTS" / "i18n-strings.json"
data = json.loads(DATA.read_text(encoding="utf-8"))

rows = [
    ("DialogForwardOne", "Forward message", "Reenviar mensaje"),
    ("DialogForwardMany", "Forward {count} messages", "Reenviar {count} mensajes"),
    ("DialogNoWritableChats", "No writable chats found", "No se encontraron chats donde escribir"),
    ("DialogEditList", "Edit list", "Editar lista"),
    ("DialogNewList", "New list", "Nueva lista"),
    ("DialogListName", "List name", "Nombre de la lista"),
    ("DialogSave", "Save", "Guardar"),
    ("DialogCreate", "Create", "Crear"),
    ("DialogShortcutsTitle", "Keyboard shortcuts", "Atajos de teclado"),
    ("DialogAboutLine", "A native WhatsApp client written in Rust with egui. It connects through whatsapp-rust. Messages are end-to-end encrypted on this device.", "Un cliente de WhatsApp nativo escrito en Rust con egui. Se conecta a través de whatsapp-rust. Los mensajes están cifrados de extremo a extremo en este dispositivo."),
    ("DialogUnofficialWarning", "This is an unofficial client. Using it may be against WhatsApp's terms of service and could get an account suspended.", "Este es un cliente no oficial. Usarlo puede ir contra los términos de servicio de WhatsApp y podría suspender una cuenta."),
    ("DialogSourceCode", "Source code", "Código fuente"),
    ("DialogVersion", "Version {version}", "Versión {version}"),
    ("DialogUnlinkTitle", "Unlink this computer?", "¿Desvincular este equipo?"),
    ("DialogUnlinkBody", "This removes the device from WhatsApp and deletes the chats stored here. You can link again with a new code.", "Esto quita el dispositivo de WhatsApp y elimina los chats guardados aquí. Puedes vincular de nuevo con un código nuevo."),
    ("DialogUnlinkButton", "Unlink", "Desvincular"),
    ("DialogLeaveChannelTitle", "Leave this channel?", "¿Salir de este canal?"),
    ("DialogLeaveGroupTitle", "Leave this group?", "¿Salir de este grupo?"),
    ("DialogLeaveBody", "You will not receive new messages. The local history stays on this computer.", "No recibirás mensajes nuevos. El historial local queda en este equipo."),
    ("DialogLeaveChannelAction", "Leave channel", "Salir del canal"),
    ("DialogLeaveGroupAction", "Leave group", "Salir del grupo"),
    ("DialogLeaveChannelArchive", "Leave channel and archive", "Salir del canal y archivar"),
    ("DialogLeaveGroupArchive", "Leave group and archive", "Salir del grupo y archivar"),
    ("DialogLinkPhoneTitle", "Link with a phone number", "Vincular con un número de teléfono"),
    ("DialogPhoneInstructions", "Enter the WhatsApp phone number with its country code. Do not include a plus sign or leading zero. You will get a code to enter on the phone.", "Escribe el número de teléfono de WhatsApp con su código de país. No incluyas el signo más ni un cero inicial. Recibirás un código para escribir en el teléfono."),
    ("DialogGetCode", "Get a code", "Obtener un código"),
    ("DialogNewContactTitle", "New contact", "Contacto nuevo"),
    ("DialogNewContactBody", "Enter a phone number with its country code, without a plus sign or leading zero. Add a name to save the contact, or leave it blank to open the chat. WhatsApp uses the first name as the display name.", "Escribe un número de teléfono con su código de país, sin el signo más ni un cero inicial. Añade un nombre para guardar el contacto, o déjalo en blanco para abrir el chat. WhatsApp usa el nombre de pila como nombre visible."),
    ("DialogFirstName", "First name", "Nombre"),
    ("DialogSurname", "Surname", "Apellido"),
    ("DialogCheckingNumber", "Checking the number…", "Verificando el número…"),
    ("DialogSaveContact", "Save contact", "Guardar contacto"),
    ("DialogMessage", "Message", "Mensaje"),
    ("KindGroup", "Group", "Grupo"),
    ("KindChannel", "Channel", "Canal"),
    ("KindContact", "Contact", "Contacto"),
    ("DialogSaveName", "Save name (Enter)", "Guardar nombre (Enter)"),
    ("DialogMembersOne", "1 member", "1 miembro"),
    ("DialogMembersMany", "{count} members", "{count} miembros"),
    ("DialogMembersTitle", "Members ({count})", "Miembros ({count})"),
    ("DialogLastSeen", "last seen {when}", "Última vez: {when}"),
    ("DialogMuted", "Muted", "Silenciado"),
    ("DialogMutedUntil", "Muted until {when}", "Silenciado hasta {when}"),
    ("DialogRename", "Rename", "Cambiar nombre"),
    ("DialogAddToContacts", "Add to contacts", "Añadir a contactos"),
    ("DialogCopyNumber", "Copy number", "Copiar número"),
    ("DialogUnmute", "Unmute", "Reactivar notificaciones"),
    ("DialogMute", "Mute", "Silenciar notificaciones"),
    ("DialogUnfavorite", "Unfavorite", "Quitar de favoritos"),
    ("DialogFavorite", "Favorite", "Añadir a favoritos"),
    ("DialogUnpin", "Unpin", "Dejar de fijar"),
    ("DialogPin", "Pin", "Fijar"),
    ("ChatArchive", "Archive", "Archivar"),
    ("ChatUnarchive", "Unarchive", "Desarchivar"),
    # bandeja y notificaciones
    ("TrayShowHide", "Show or hide WhatsFast", "Mostrar u ocultar WhatsFast"),
    ("TrayThreadStopped", "The tray thread stopped responding", "La bandeja dejó de responder"),
]

existing = {r["key"] for r in data["strings"]}
for key, en, es in rows:
    assert key not in existing, f"duplicate key {key}"
    data["strings"].append({"key": key, "en": en, "es": es})

sites = [
    # --- expresiones con format! (dialogs)
    {
        "file": "src/ui/dialogs.rs",
        "old": "format!(\"Forward {} messages\", messages.len())",
        "new": "i18n::f(Key::DialogForwardMany, &[(\"count\", &messages.len().to_string())])",
    },
    {
        "file": "src/ui/dialogs.rs",
        "old": "format!(\"Version {}\", env!(\"CARGO_PKG_VERSION\"))",
        "new": "i18n::f(Key::DialogVersion, &[(\"version\", env!(\"CARGO_PKG_VERSION\"))])",
    },
    {
        "file": "src/ui/dialogs.rs",
        "old": "format!(\"{} members\", chat.participants.len())",
        "new": "i18n::count(\n            Key::DialogMembersOne,\n            Key::DialogMembersMany,\n            chat.participants.len(),\n        )",
    },
    {
        "file": "src/ui/dialogs.rs",
        "old": "format!(\"Members ({})\", members.len())",
        "new": "i18n::f(Key::DialogMembersTitle, &[(\"count\", &members.len().to_string())])",
    },
    {
        "file": "src/ui/dialogs.rs",
        "old": "format!(\"last seen {}\", crate::util::chat_stamp(seen).to_lowercase())",
        "new": "i18n::f(Key::DialogLastSeen, &[(\"when\", &crate::util::chat_stamp(seen))])",
    },
    {
        "file": "src/ui/dialogs.rs",
        "old": "format!(\"Muted until {}\", crate::util::chat_stamp(until))",
        "new": "i18n::f(Key::DialogMutedUntil, &[(\"when\", &crate::util::chat_stamp(until))])",
    },
    # --- línea con dos literales
    {
        "file": "src/ui/dialogs.rs",
        "old": "if id.is_some() { \"Save\" } else { \"Create\" },",
        "new": "if id.is_some() {\n            i18n::t(Key::DialogSave)\n        } else {\n            i18n::t(Key::DialogCreate)\n        },",
    },
    {
        "file": "src/ui/dialogs.rs",
        "old": "if known { \"Rename\" } else { \"Add to contacts\" },",
        "new": "if known {\n            i18n::t(Key::DialogRename)\n        } else {\n            i18n::t(Key::DialogAddToContacts)\n        },",
    },
    {
        "file": "src/ui/dialogs.rs",
        "old": "if chat.pinned { \"Unpin\" } else { \"Pin\" },",
        "new": "if chat.pinned {\n            i18n::t(Key::DialogUnpin)\n        } else {\n            i18n::t(Key::DialogPin)\n        },",
    },
    # --- import
    {
        "file": "src/ui/dialogs.rs",
        "old": "use crate::app::App;",
        "new": "use crate::app::App;\nuse crate::i18n::{self, Key};",
    },
    {
        "file": "src/tray.rs",
        "old": "use ksni::blocking::TrayMethods;",
        "new": "use ksni::blocking::TrayMethods;\n\nuse crate::i18n::{self, Key};",
    },
    {
        "file": "src/tray_native.rs",
        "old": "use tray_icon::{Icon, TrayIcon, TrayIconBuilder};",
        "new": "use tray_icon::{Icon, TrayIcon, TrayIconBuilder};\n\nuse crate::i18n::{self, Key};",
    },
    {
        "file": "src/notify.rs",
        "old": "use std::sync::{Arc, Mutex};",
        "new": "use std::sync::{Arc, Mutex};\n\nuse crate::i18n::{self, Key};",
    },
    # --- literales
    {
        "name": "dialogs-literals",
        "file": "src/ui/dialogs.rs",
        "literals": [
            {"exact": "Forward message", "new": "i18n::t(Key::DialogForwardOne)"},
            {"exact": "Search chats", "new": "i18n::t(Key::ShortcutSearchChats)", "all": True},
            {"exact": "No writable chats found", "new": "i18n::t(Key::DialogNoWritableChats)"},
            {"exact": "Edit list", "new": "i18n::t(Key::DialogEditList)"},
            {"exact": "New list", "new": "i18n::t(Key::DialogNewList)"},
            {"exact": "List name", "new": "i18n::t(Key::DialogListName)"},
            {"exact": "Save", "new": "i18n::t(Key::DialogSave)"},
            {"exact": "Close", "new": "i18n::t(Key::CommonClose)"},
            {"exact": "Keyboard shortcuts", "new": "i18n::t(Key::DialogShortcutsTitle)"},
            {"exact": "About", "new": "i18n::t(Key::SettingsSectionAbout)"},
            {"starts_with": "A native WhatsApp client written in Rust", "new": "i18n::t(Key::DialogAboutLine)"},
            {"starts_with": "This is an unofficial client.", "new": "i18n::t(Key::DialogUnofficialWarning)"},
            {"exact": "Source code", "new": "i18n::t(Key::DialogSourceCode)"},
            {"exact": "Unlink this computer?", "new": "i18n::t(Key::DialogUnlinkTitle)"},
            {"starts_with": "This removes the device from WhatsApp", "new": "i18n::t(Key::DialogUnlinkBody)"},
            {"exact": "Unlink", "new": "i18n::t(Key::DialogUnlinkButton)"},
            {"exact": "Cancel", "new": "i18n::t(Key::DialogCancel)", "all": True},
            {"exact": "Leave this channel?", "new": "i18n::t(Key::DialogLeaveChannelTitle)"},
            {"exact": "Leave this group?", "new": "i18n::t(Key::DialogLeaveGroupTitle)"},
            {"starts_with": "You will not receive new messages.", "new": "i18n::t(Key::DialogLeaveBody)"},
            {"exact": "Leave channel", "new": "i18n::t(Key::DialogLeaveChannelAction)", "all": True},
            {"exact": "Leave group", "new": "i18n::t(Key::DialogLeaveGroupAction)", "all": True},
            {"exact": "Leave channel and archive", "new": "i18n::t(Key::DialogLeaveChannelArchive)"},
            {"exact": "Leave group and archive", "new": "i18n::t(Key::DialogLeaveGroupArchive)"},
            {"exact": "Link with a phone number", "new": "i18n::t(Key::DialogLinkPhoneTitle)"},
            {"starts_with": "Enter the WhatsApp phone number", "new": "i18n::t(Key::DialogPhoneInstructions)"},
            {"exact": "Get a code", "new": "i18n::t(Key::DialogGetCode)"},
            {"exact": "New contact", "new": "i18n::t(Key::DialogNewContactTitle)"},
            {"starts_with": "Enter a phone number with its country code", "new": "i18n::t(Key::DialogNewContactBody)"},
            {"exact": "First name", "new": "i18n::t(Key::DialogFirstName)", "all": True},
            {"exact": "Surname", "new": "i18n::t(Key::DialogSurname)", "all": True},
            {"exact": "Checking the number…", "new": "i18n::t(Key::DialogCheckingNumber)"},
            {"exact": "Save contact", "new": "i18n::t(Key::DialogSaveContact)"},
            {"exact": "Message", "new": "i18n::t(Key::DialogMessage)", "all": True},
            {"exact": "Group", "new": "i18n::t(Key::KindGroup)"},
            {"exact": "Channel", "new": "i18n::t(Key::KindChannel)"},
            {"exact": "Contact", "new": "i18n::t(Key::KindContact)"},
            {"exact": "Save name (Enter)", "new": "i18n::t(Key::DialogSaveName)"},
            {"exact": "Muted", "new": "i18n::t(Key::DialogMuted)"},
            {"exact": "Copy number", "new": "i18n::t(Key::DialogCopyNumber)"},
            {"exact": "Unmute", "new": "i18n::t(Key::DialogUnmute)"},
            {"exact": "Mute", "new": "i18n::t(Key::DialogMute)"},
            {"exact": "Unfavorite", "new": "i18n::t(Key::DialogUnfavorite)"},
            {"exact": "Favorite", "new": "i18n::t(Key::DialogFavorite)"},
            {"exact": "Unarchive", "new": "i18n::t(Key::ChatUnarchive)"},
            {"exact": "Archive", "new": "i18n::t(Key::ChatArchive)"},
        ],
    },
    {
        "name": "tray-literals",
        "file": "src/tray.rs",
        "literals": [
            {"exact": "Show or hide WhatsFast", "new": "i18n::t(Key::TrayShowHide)"},
            {"exact": "Quit", "new": "i18n::t(Key::CommonQuit)"},
        ],
    },
    {
        "name": "tray-native-literals",
        "file": "src/tray_native.rs",
        "literals": [
            {"exact": "Show or hide WhatsFast", "new": "i18n::t(Key::TrayShowHide)"},
            {"exact": "Quit", "new": "i18n::t(Key::CommonQuit)"},
            {"exact": "The tray thread stopped responding", "new": "i18n::t(Key::TrayThreadStopped)"},
        ],
    },
    {
        "name": "notify-literals",
        "file": "src/notify.rs",
        "literals": [
            {"exact": "Open", "new": "i18n::t(Key::CommonOpen)"},
        ],
    },
]

data["sites"].extend(sites)
DATA.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
print(f"rows: {len(data['strings'])}, sites: {len(data['sites'])}")
