"""Lote 9: app.rs, model.rs, privacy.rs."""
import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
DATA = ROOT / "AGENTS" / "i18n-strings.json"
data = json.loads(DATA.read_text(encoding="utf-8"))

rows = [
    # app.rs
    ("DisplayUnknown", "Unknown", "Desconocido"),
    ("ToastHistoryLoaded", "History loaded", "Historial cargado"),
    ("ToastBackOnline", "Back online", "De vuelta en línea"),
    ("ToastUnlinked", "This device was unlinked from your phone", "Este equipo se desvinculó de tu teléfono"),
    ("ToastOpenChatFirst", "Open a chat first", "Abre un chat primero"),
    ("ToastSendingFilesOne", "Sending 1 file…", "Enviando 1 archivo…"),
    ("ToastSendingFilesMany", "Sending {count} files…", "Enviando {count} archivos…"),
    ("ToastCouldNotOpen", "Could not open {what}: {error}", "No se pudo abrir {what}: {error}"),
    ("ToastCopied", "Copied", "Copiado"),
    ("ToastStickerSaved", "Sticker saved", "Sticker guardado"),
    ("ToastSendingGif", "Sending GIF…", "Enviando GIF…"),
    ("ToastCouldNotRecord", "Could not record: {error}", "No se pudo grabar: {error}"),
    ("DialogDigitsOnly", "Enter the phone number with its country code, using digits only", "Escribe el número de teléfono con su código de país, solo con dígitos"),
    # model.rs
    ("PollErrorQuestion", "Enter a question of up to 255 characters.", "Escribe una pregunta de hasta 255 caracteres."),
    ("PollErrorAnswers", "Add 2–12 answers, each with 1–100 characters.", "Añade de 2 a 12 respuestas, de 1 a 100 caracteres cada una."),
    ("PollErrorDuplicates", "Each answer must be different.", "Cada respuesta debe ser distinta."),
    ("KindPhoto", "Photo", "Foto"),
    ("KindVoice", "Voice message", "Mensaje de voz"),
    ("KindAudio", "Audio", "Audio"),
    ("KindSticker", "Sticker", "Sticker"),
    ("KindDocumentWith", "Document: {name}", "Documento: {name}"),
    ("KindLocationWith", "Location: {name}", "Ubicación: {name}"),
    ("KindContactWith", "Contact: {name}", "Contacto: {name}"),
    ("KindPollWith", "Poll: {question}", "Encuesta: {question}"),
    ("ChatUnsupportedMessage", "Unsupported message ({what})", "Mensaje no compatible ({what})"),
    ("ChatRetryFile", "We are still trying to get this file automatically. Click to retry manually.", "Seguimos intentando conseguir este archivo automáticamente. Haz clic para reintentar."),
    ("ChatFileGone", "No longer available on WhatsApp's servers", "Ya no está disponible en los servidores de WhatsApp"),
    # privacy.rs
    ("PrivacyLastSeen", "Last seen", "Última vez"),
    ("PrivacyOnline", "Online", "En línea"),
    ("PrivacyProfilePhoto", "Profile photo", "Foto del perfil"),
    ("PrivacyAbout", "About", "Información"),
    ("PrivacyGroupsAdd", "Who can add me to groups", "Quién puede añadirme a grupos"),
    ("PrivacyReadReceipts", "Read receipts", "Confirmaciones de lectura"),
    ("PrivacyCalls", "Who can call me", "Quién puede llamarme"),
    ("PrivacyMessages", "Who can message me", "Quién puede enviarme mensajes"),
    ("PrivacyLastSeenHint", "When people can see you were last using WhatsApp.", "Cuándo pueden ver que usaste WhatsApp por última vez."),
    ("PrivacyOnlineHint", "When people can see you are online now.", "Cuándo pueden ver que estás en línea."),
    ("PrivacyProfilePhotoHint", "Who can see your profile photo.", "Quién puede ver tu foto del perfil."),
    ("PrivacyAboutHint", "Who can see your About text. This is not the Status tab.", "Quién puede ver tu información. No es la pestaña Estado."),
    ("PrivacyGroupsHint", "Who can add you to a group.", "Quién puede añadirte a un grupo."),
    ("PrivacyReceiptsHint", "Everyone or nobody on this WhatsApp account. The Chats switch still applies to this copy.", "Todos o nadie en tu cuenta de WhatsApp. El interruptor de Chats sigue aplicando a esta copia."),
    ("PrivacyCallsHint", "Who can call you on WhatsApp.", "Quién puede llamarte por WhatsApp."),
    ("PrivacyMessagesHint", "Who can start a chat with you.", "Quién puede iniciar un chat contigo."),
    ("PrivacyHideLastSeen", "Hide last seen from", "Ocultar Última vez para"),
    ("PrivacyHidePhoto", "Hide profile photo from", "Ocultar foto del perfil para"),
    ("PrivacyHideAbout", "Hide About from", "Ocultar Información para"),
    ("PrivacyWhoCannotAdd", "Who cannot add you to groups", "Quién no puede añadirte a grupos"),
    ("PrivacyExcept", "Except", "Excepto"),
    ("PrivacyEveryone", "Everyone", "Todos"),
    ("PrivacyMyContacts", "My contacts", "Mis contactos"),
    ("PrivacyExceptEllipsis", "Except…", "Excepto…"),
    ("PrivacyNobody", "Nobody", "Nadie"),
    ("PrivacySameAsLastSeen", "Same as last seen", "Igual que Última vez"),
    ("PrivacyContactsWithNumber", "My contacts and other people with my number", "Mis contactos y otras personas con mi número"),
]

existing = {r["key"] for r in data["strings"]}
for key, en, es in rows:
    assert key not in existing, f"duplicate key {key}"
    data["strings"].append({"key": key, "en": en, "es": es})

sites = [
    {
        "file": "src/app.rs",
        "old": "use crate::backend::Backend;",
        "new": "use crate::backend::Backend;\nuse crate::i18n::{self, Key};",
    },
    # expresiones
    {
        "file": "src/app.rs",
        "old": """        self.toast(format!(
            "Sending {} file{}…",
            paths.len(),
            if paths.len() == 1 { "" } else { "s" }
        ));""",
        "new": """        self.toast(i18n::count(
            Key::ToastSendingFilesOne,
            Key::ToastSendingFilesMany,
            paths.len(),
        ));""",
    },
    {
        "file": "src/app.rs",
        "old": "self.toast_error(format!(\"Could not open {}: {error}\", path.display()));",
        "new": "self.toast_error(i18n::f(\n                        Key::ToastCouldNotOpen,\n                        &[(\"what\", &path.display().to_string()), (\"error\", &error.to_string())],\n                    ));",
    },
    {
        "file": "src/app.rs",
        "old": "self.toast_error(format!(\"Could not record: {error}\"));",
        "new": "self.toast_error(i18n::f(\n                Key::ToastCouldNotRecord,\n                &[(\"error\", &error.to_string())],\n            ));",
        "all": True,
    },
    {
        "name": "app-literals",
        "file": "src/app.rs",
        "literals": [
            {"exact": "You", "new": "i18n::t(Key::DisplayYou)", "all": True},
            {"exact": "Unknown", "new": "i18n::t(Key::DisplayUnknown)", "all": True},
            {"exact": "History loaded", "new": "i18n::t(Key::ToastHistoryLoaded)"},
            {"exact": "Back online", "new": "i18n::t(Key::ToastBackOnline)"},
            {"exact": "This device was unlinked from your phone", "new": "i18n::t(Key::ToastUnlinked)"},
            {"exact": "Open a chat first", "new": "i18n::t(Key::ToastOpenChatFirst)", "all": True},
            {"exact": "Copied", "new": "i18n::t(Key::ToastCopied)"},
            {"exact": "Sticker saved", "new": "i18n::t(Key::ToastStickerSaved)"},
            {"exact": "Sending GIF…", "new": "i18n::t(Key::ToastSendingGif)"},
            {"starts_with": "Enter the phone number with its country code", "new": "i18n::t(Key::DialogDigitsOnly)"},
        ],
    },
    {
        "file": "src/model.rs",
        "old": "use crate::archive::ChatId;",
        "new": "use crate::archive::ChatId;\nuse crate::i18n::{self, Key};",
    },
    {
        "file": "src/model.rs",
        "old": "format!(\"Document: {file_name}\")",
        "new": "i18n::f(Key::KindDocumentWith, &[(\"name\", file_name)])",
    },
    {
        "file": "src/model.rs",
        "old": "format!(\"Location: {name}\")",
        "new": "i18n::f(Key::KindLocationWith, &[(\"name\", name)])",
    },
    {
        "file": "src/model.rs",
        "old": "format!(\"Contact: {display_name}\")",
        "new": "i18n::f(Key::KindContactWith, &[(\"name\", display_name)])",
    },
    {
        "file": "src/model.rs",
        "old": "format!(\"Poll: {question}\")",
        "new": "i18n::f(Key::KindPollWith, &[(\"question\", question)])",
    },
    {
        "name": "model-literals",
        "file": "src/model.rs",
        "literals": [
            {"starts_with": "Enter a question of up to", "new": "i18n::t(Key::PollErrorQuestion)"},
            {"starts_with": "Add 2–12 answers", "new": "i18n::t(Key::PollErrorAnswers)"},
            {"exact": "Each answer must be different.", "new": "i18n::t(Key::PollErrorDuplicates)"},
            {"exact": "Photo", "new": "i18n::t(Key::KindPhoto)", "all": True},
            {"exact": "GIF", "new": "i18n::t(Key::KindGif)", "all": True},
            {"exact": "Video", "new": "i18n::t(Key::KindVideo)", "all": True},
            {"exact": "Voice message", "new": "i18n::t(Key::KindVoice)"},
            {"exact": "Audio", "new": "i18n::t(Key::KindAudio)"},
            {"exact": "Sticker", "new": "i18n::t(Key::KindSticker)"},
            {"exact": "Location", "new": "i18n::t(Key::KindLocation)"},
            {"exact": "This message was deleted", "new": "i18n::t(Key::ChatMessageDeleted)"},
            {"starts_with": "Unsupported message", "new": "i18n::t(Key::ChatUnsupportedMessage)"},
            {"starts_with": "We are still trying to get this file", "new": "i18n::t(Key::ChatRetryFile)"},
            {"starts_with": "No longer available on WhatsApp", "new": "i18n::t(Key::ChatFileGone)"},
        ],
    },
    {
        "file": "src/privacy.rs",
        "old": "use crate::model::ChatId;",
        "new": "use crate::i18n::{self, Key};\nuse crate::model::ChatId;",
    },
    {
        "name": "privacy-literals",
        "file": "src/privacy.rs",
        "literals": [
            {"exact": "Last seen", "new": "i18n::t(Key::PrivacyLastSeen)"},
            {"exact": "Online", "new": "i18n::t(Key::PrivacyOnline)"},
            {"exact": "Profile photo", "new": "i18n::t(Key::PrivacyProfilePhoto)"},
            {"exact": "About", "new": "i18n::t(Key::PrivacyAbout)"},
            {"exact": "Who can add me to groups", "new": "i18n::t(Key::PrivacyGroupsAdd)"},
            {"exact": "Read receipts", "new": "i18n::t(Key::PrivacyReadReceipts)"},
            {"exact": "Who can call me", "new": "i18n::t(Key::PrivacyCalls)"},
            {"exact": "Who can message me", "new": "i18n::t(Key::PrivacyMessages)"},
            {"starts_with": "When people can see you were last", "new": "i18n::t(Key::PrivacyLastSeenHint)"},
            {"starts_with": "When people can see you are online", "new": "i18n::t(Key::PrivacyOnlineHint)"},
            {"exact": "Who can see your profile photo.", "new": "i18n::t(Key::PrivacyProfilePhotoHint)"},
            {"starts_with": "Who can see your About text.", "new": "i18n::t(Key::PrivacyAboutHint)"},
            {"exact": "Who can add you to a group.", "new": "i18n::t(Key::PrivacyGroupsHint)"},
            {"starts_with": "Everyone or nobody on this WhatsApp account.", "new": "i18n::t(Key::PrivacyReceiptsHint)"},
            {"exact": "Who can call you on WhatsApp.", "new": "i18n::t(Key::PrivacyCallsHint)"},
            {"exact": "Who can start a chat with you.", "new": "i18n::t(Key::PrivacyMessagesHint)"},
            {"exact": "Hide last seen from", "new": "i18n::t(Key::PrivacyHideLastSeen)"},
            {"exact": "Hide profile photo from", "new": "i18n::t(Key::PrivacyHidePhoto)"},
            {"exact": "Hide About from", "new": "i18n::t(Key::PrivacyHideAbout)"},
            {"exact": "Who cannot add you to groups", "new": "i18n::t(Key::PrivacyWhoCannotAdd)"},
            {"exact": "Except", "new": "i18n::t(Key::PrivacyExcept)"},
            {"exact": "Everyone", "new": "i18n::t(Key::PrivacyEveryone)"},
            {"exact": "My contacts", "new": "i18n::t(Key::PrivacyMyContacts)"},
            {"exact": "Except…", "new": "i18n::t(Key::PrivacyExceptEllipsis)"},
            {"exact": "Nobody", "new": "i18n::t(Key::PrivacyNobody)"},
            {"exact": "Same as last seen", "new": "i18n::t(Key::PrivacySameAsLastSeen)"},
            {"starts_with": "My contacts and other people", "new": "i18n::t(Key::PrivacyContactsWithNumber)"},
        ],
    },
]

data["sites"].extend(sites)
DATA.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
print(f"rows: {len(data['strings'])}, sites: {len(data['sites'])}")
