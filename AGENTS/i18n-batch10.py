"""Lote 10: backend/worker.rs + backend/worker/polls.rs (errores y toasts)."""
import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
DATA = ROOT / "AGENTS" / "i18n-strings.json"
data = json.loads(DATA.read_text(encoding="utf-8"))

rows = [
    ("ToastNotConnected", "Not connected to WhatsApp", "Sin conexión a WhatsApp"),
    ("ToastNotConnectedYet", "Not connected to WhatsApp yet", "Todavía no hay conexión a WhatsApp"),
    ("PollErrCantSendHere", "Polls cannot be sent to this chat.", "No se pueden enviar encuestas a este chat."),
    ("PollErrDisappearing", "Poll creation in disappearing-message chats is not supported yet.", "Aún no se pueden crear encuestas en chats con mensajes que desaparecen."),
    ("PollErrRecipients", "Could not load the group recipients", "No se pudo cargar la lista de destinatarios del grupo"),
    ("PollErrSend", "Could not send the poll. Please try again.", "No se pudo enviar la encuesta. Inténtalo de nuevo."),
    ("PollErrAccountChanged", "The account changed while the poll was being sent.", "La cuenta cambió mientras se enviaba la encuesta."),
    ("PollErrKeyNotSaved", "The poll was sent, but its voting key could not be saved.", "La encuesta se envió, pero no se pudo guardar su clave de votación."),
    ("PollErrNotReady", "This poll is not ready for voting. Reconnect and try again.", "Esta encuesta todavía no está lista para votar. Vuelve a conectar e inténtalo de nuevo."),
    ("PollErrVoteSend", "Could not send your vote. Please try again.", "No se pudo enviar tu voto. Inténtalo de nuevo."),
    ("PollErrVoteNotSaved", "Your vote was sent, but could not be saved locally.", "Tu voto se envió, pero no se pudo guardar en este equipo."),
    ("ChatWaitingMessage", "Waiting for this message. Open WhatsApp on your phone", "Esperando este mensaje. Abre WhatsApp en tu teléfono"),
    ("ToastHistoryPartFailed", "Could not read part of the chat history: {error}", "No se pudo leer parte del historial: {error}"),
    ("ToastNoOlder", "Your phone did not send older messages. Check that it is online", "Tu teléfono no envió mensajes anteriores. Comprueba que esté en línea"),
    ("ToastOlderFailed", "Could not request older messages from your phone: {error}", "No se pudieron pedir mensajes anteriores a tu teléfono: {error}"),
    ("ToastStarred", "Starred", "Destacado"),
    ("ToastStarRemoved", "Star removed", "Se quitó el destacado"),
    ("ToastStarredProgress", "Starred {done} of {total}", "Destacados {done} de {total}"),
    ("ChatSendToWhatsApp", "Send to WhatsApp", "Enviar a WhatsApp"),
    ("ToastStickerSaveFailed", "Could not save sticker: {error}", "No se pudo guardar el sticker: {error}"),
    ("DialogAddPack", "Add a sticker pack", "Añadir un pack de stickers"),
    ("DialogStickerPacks", "Sticker packs", "Packs de stickers"),
    ("ToastContactSaveFailed", "Could not save contact: {error}", "No se pudo guardar el contacto: {error}"),
    ("ToastContactAdded", "Added {name} to contacts", "Se añadió {name} a los contactos"),
    ("ToastNotOnWhatsApp", "{name} is not on WhatsApp", "{name} no está en WhatsApp"),
    ("ToastPackAdded", "Added sticker pack \"{name}\"", "Se añadió el pack de stickers \"{name}\""),
    ("ToastPackFailed", "Could not add sticker pack: {error}", "No se pudo añadir el pack de stickers: {error}"),
    ("ToastPrivacyFailed", "Could not update privacy settings.", "No se pudieron actualizar los ajustes de privacidad."),
    ("ToastLinkPhoneFailed", "Could not link by phone number: {error}", "No se pudo vincular con el número de teléfono: {error}"),
    ("ToastMessageNotSent", "Message not sent: {error}", "No se envió el mensaje: {error}"),
    ("ToastSavedTo", "Saved to {path}", "Guardado en {path}"),
    ("ToastDownloadNotSaved", "The download failed, so it was not saved", "La descarga falló, así que no se guardó"),
    ("ToastLeaveChannelFailed", "Could not leave the channel.", "No se pudo salir del canal."),
    ("ToastLeaveGroupFailed", "Could not leave the group.", "No se pudo salir del grupo."),
    ("ToastScheduled", "Message scheduled", "Mensaje programado"),
    ("ToastForwardBusy", "Wait for the current forward to finish", "Espera a que termine el reenvío actual"),
    ("ToastForwardedProgress", "Forwarded {done} of {total}", "Reenviados {done} de {total}"),
    ("ToastNotStored", "This message is not stored on this computer", "Este mensaje no está en este equipo"),
    ("ToastNotOnComputer", "This message is not on this computer", "Este mensaje no está en este equipo"),
    ("ToastCannotForward", "This message cannot be forwarded", "Este mensaje no se puede reenviar"),
    ("ToastForwardNoData", "The original message data is not available to forward", "Los datos del mensaje original no están disponibles para reenviar"),
    ("ToastForwardUnreadable", "The original message data could not be read", "No se pudieron leer los datos del mensaje original"),
    ("ToastReadChatFailed", "Could not read the chat: {error}", "No se pudo leer el chat: {error}"),
    ("ToastKeysMissing", "Attachment download keys are missing", "Faltan las claves de descarga del adjunto"),
    ("ToastNoFile", "This message has no downloadable file", "Este mensaje no tiene un archivo para descargar"),
    ("ToastNoDownloads", "No Downloads folder on this computer", "No hay una carpeta de Descargas en este equipo"),
    ("ToastSaveDialogOpen", "A save dialog is already open", "Ya hay un diálogo de guardado abierto"),
    ("DialogSaveAttachment", "Save attachment", "Guardar adjunto"),
    ("DialogSingleFile", "a single file", "un solo archivo"),
    ("ToastSaveCancelled", "Save cancelled", "Guardado cancelado"),
    ("ToastSavingProgress", "Saving {done} of {total}", "Guardando {done} de {total}"),
    ("ToastSearchFailed", "Could not search: {error}", "No se pudo buscar: {error}"),
    ("ToastEditFailed", "Could not send the edit: {error}", "No se pudo enviar la edición: {error}"),
    ("ToastDeleteFailed", "Could not delete the message for everyone: {error}", "No se pudo eliminar el mensaje para todos: {error}"),
    ("ToastFileFailed", "Could not send the file: {error}", "No se pudo enviar el archivo: {error}"),
    ("ToastClipboardInvalid", "Clipboard image data is invalid", "Los datos de imagen del portapapeles no son válidos"),
    ("ToastPictureFailed", "Could not send the picture: {error}", "No se pudo enviar la imagen: {error}"),
    ("ToastVoiceFailed", "Could not send the voice message: {error}", "No se pudo enviar el mensaje de voz: {error}"),
    ("ToastStickerSendFailed", "Could not send the sticker: {error}", "No se pudo enviar el sticker: {error}"),
    ("ToastGifFailed", "Could not send the GIF: {error}", "No se pudo enviar el GIF: {error}"),
    ("ToastEncodeFailed", "Could not encode the attachment", "No se pudo codificar el adjunto"),
    ("ToastShuttingDown", "The application is shutting down", "La aplicación se está cerrando"),
    ("ToastRecipientsSaveFailed", "Could not save the group message recipients", "No se pudieron guardar los destinatarios del mensaje de grupo"),
    ("ToastStartFailed", "Could not start WhatsApp: {error}", "No se pudo iniciar WhatsApp: {error}"),
    ("ToastDeviceStoreFailed", "Could not open the device store: {error}", "No se pudo abrir el almacén del dispositivo: {error}"),
    ("ErrDetail", ": {message}", ": {message}"),
    ("ErrConnectFailed", "WhatsApp connection failed ({reason}){detail}", "Falló la conexión con WhatsApp ({reason}){detail}"),
    ("ErrStreamReplaced", "Another WhatsApp Web session replaced this one", "Otra sesión de WhatsApp Web reemplazó a esta"),
    ("ErrTemporaryBan", "WhatsApp has temporarily blocked this account ({code})", "WhatsApp bloqueó esta cuenta temporalmente ({code})"),
    ("ErrClientOutdated", "WhatsApp rejected this version of WhatsFast. Update the app", "WhatsApp rechazó esta versión de WhatsFast. Actualiza la aplicación"),
    ("KindDocument", "Document", "Documento"),
    ("KindLiveLocation", "Live location", "Ubicación en vivo"),
    ("KindContact", "Contact", "Contacto"),
    ("KindContactsOne", "1 contact", "1 contacto"),
    ("KindContactsMany", "{count} contacts", "{count} contactos"),
    ("KindPoll", "Poll", "Encuesta"),
    ("KindGroupInvite", "group invite", "invitación a un grupo"),
    ("KindStickerPack", "sticker pack", "pack de stickers"),
    ("KindInteractive", "interactive message", "mensaje interactivo"),
    ("KindAnimatedSticker", "animated sticker", "sticker animado"),
    ("GifErrNoKey", "GIF search needs a GIPHY API key.", "La búsqueda de GIF necesita una clave de API de GIPHY."),
    ("GifErrRequest", "GIPHY request failed: {error}", "Falló la solicitud a GIPHY: {error}"),
    ("GifErrKeyRejected", "GIPHY rejected the API key (error {code}).", "GIPHY rechazó la clave de API (error {code})."),
    ("GifErrResponse", "Invalid GIPHY response: {error}", "Respuesta inválida de GIPHY: {error}"),
    ("GifErrMessage", "GIPHY: {message}", "GIPHY: {message}"),
    ("GifErrNoResults", "GIPHY response contained no results", "La respuesta de GIPHY no tuvo resultados"),
]

by_key = {r["key"]: r for r in data["strings"]}
for key, en, es in rows:
    if key in by_key:
        assert by_key[key]["en"] == en and by_key[key]["es"] == es, f"conflicting key {key}"
        continue
    data["strings"].append({"key": key, "en": en, "es": es})

sites = [
    {
        "file": "src/backend/worker.rs",
        "old": "use crate::archive::{",
        "new": "use crate::i18n::{self, Key};\nuse crate::archive::{",
    },
    {
        "file": "src/backend/worker/polls.rs",
        "old": "use crate::archive::polls::",
        "new": "use crate::i18n::{self, Key};\nuse crate::archive::polls::",
    },
    # --- casos especiales
    {
        "file": "src/backend/worker.rs",
        "old": """                    let detail = failure
                        .message
                        .as_ref()
                        .map(|message| format!(": {message}"))
                        .unwrap_or_default();
                    self.emit(Event::Error(format!(
                        "WhatsApp connection failed ({:?}){detail}",
                        failure.reason
                    )));""",
        "new": """                    let detail = failure
                        .message
                        .as_ref()
                        .map(|message| i18n::f(Key::ErrDetail, &[("message", message)]))
                        .unwrap_or_default();
                    self.emit(Event::Error(i18n::f(
                        Key::ErrConnectFailed,
                        &[
                            ("reason", &format!("{:?}", failure.reason)),
                            ("detail", &detail),
                        ],
                    )));""",
    },
    {
        "file": "src/backend/worker.rs",
        "old": """                self.set_status(LinkStatus::Failed(format!(
                    "WhatsApp has temporarily blocked this account ({:?})",
                    ban.code
                )));""",
        "new": """                self.set_status(LinkStatus::Failed(i18n::f(
                    Key::ErrTemporaryBan,
                    &[("code", &format!("{:?}", ban.code))],
                )));""",
    },
    {
        "file": "src/backend/worker.rs",
        "old": """                self.emit(Event::Error(format!(
                    "Could not link by phone number: {}",
                    error.error
                )));""",
        "new": """                self.emit(Event::Error(i18n::f(
                    Key::ToastLinkPhoneFailed,
                    &[("error", &error.error.to_string())],
                )));""",
    },
    {
        "file": "src/backend/worker.rs",
        "old": """                    self.emit(Event::Error(format!(
                        "Could not link by phone number: {error}"
                    )));""",
        "new": """                    self.emit(Event::Error(i18n::f(
                        Key::ToastLinkPhoneFailed,
                        &[("error", &error.to_string())],
                    )));""",
    },
    {
        "file": "src/backend/worker.rs",
        "old": "format!(\"Starred {done} of {total}\")",
        "new": "i18n::f(\n                            Key::ToastStarredProgress,\n                            &[(\"done\", &done.to_string()), (\"total\", &total.to_string())],\n                        )",
    },
    {
        "file": "src/backend/worker.rs",
        "old": "format!(\"Forwarded {done} of {total}\")",
        "new": "i18n::f(\n                Key::ToastForwardedProgress,\n                &[(\"done\", &done.to_string()), (\"total\", &total.to_string())],\n            )",
    },
    {
        "file": "src/backend/worker.rs",
        "old": "format!(\"{count} contacts\")",
        "new": "i18n::count(Key::KindContactsOne, Key::KindContactsMany, count)",
    },
    # --- plantillas con placeholders
    {
        "name": "worker-fmt",
        "file": "src/backend/worker.rs",
        "fmt_items": [
            {"literal": "Could not read part of the chat history: {error}", "new": "i18n::f(Key::ToastHistoryPartFailed, &[(\"error\", &error.to_string())])"},
            {"literal": "Could not request older messages from your phone: {error}", "new": "i18n::f(Key::ToastOlderFailed, &[(\"error\", &error.to_string())])"},
            {"literal": "Could not save sticker: {error}", "new": "i18n::f(Key::ToastStickerSaveFailed, &[(\"error\", &error.to_string())])"},
            {"literal": "Could not save contact: {error}", "new": "i18n::f(Key::ToastContactSaveFailed, &[(\"error\", &error.to_string())])"},
            {"literal": "Added {name} to contacts", "new": "i18n::f(Key::ToastContactAdded, &[(\"name\", name)])"},
            {"literal": "{} is not on WhatsApp", "new": "i18n::f(Key::ToastNotOnWhatsApp, &[(\"name\", &number.to_string())])"},
            {"literal": "Added sticker pack \\\"{name}\\\"", "new": "i18n::f(Key::ToastPackAdded, &[(\"name\", name)])"},
            {"literal": "Could not add sticker pack: {error}", "new": "i18n::f(Key::ToastPackFailed, &[(\"error\", &error.to_string())])"},
            {"literal": "Message not sent: {error}", "new": "i18n::f(Key::ToastMessageNotSent, &[(\"error\", &error.to_string())])"},
            {"literal": "Saved to {}", "new": "i18n::f(Key::ToastSavedTo, &[(\"path\", &path.to_string())])", "all": True},
            {"literal": "Could not read the chat: {error}", "new": "i18n::f(Key::ToastReadChatFailed, &[(\"error\", &error.to_string())])", "all": True},
            {"literal": "Could not search: {error}", "new": "i18n::f(Key::ToastSearchFailed, &[(\"error\", &error.to_string())])", "all": True},
            {"literal": "Could not send the edit: {error}", "new": "i18n::f(Key::ToastEditFailed, &[(\"error\", &error.to_string())])"},
            {"literal": "Could not delete the message for everyone: {error}", "new": "i18n::f(Key::ToastDeleteFailed, &[(\"error\", &error.to_string())])"},
            {"literal": "Could not send the file: {error}", "new": "i18n::f(Key::ToastFileFailed, &[(\"error\", &error.to_string())])"},
            {"literal": "Could not send the picture: {error}", "new": "i18n::f(Key::ToastPictureFailed, &[(\"error\", &error.to_string())])"},
            {"literal": "Could not send the voice message: {error}", "new": "i18n::f(Key::ToastVoiceFailed, &[(\"error\", &error.to_string())])"},
            {"literal": "Could not send the sticker: {error}", "new": "i18n::f(Key::ToastStickerSendFailed, &[(\"error\", &error.to_string())])"},
            {"literal": "Could not send the GIF: {error}", "new": "i18n::f(Key::ToastGifFailed, &[(\"error\", &error.to_string())])"},
            {"literal": "Could not start WhatsApp: {error}", "new": "i18n::f(Key::ToastStartFailed, &[(\"error\", &error.to_string())])"},
            {"literal": "Could not open the device store: {error}", "new": "i18n::f(Key::ToastDeviceStoreFailed, &[(\"error\", &error.to_string())])"},
            {"literal": "GIPHY request failed: {error}", "new": "i18n::f(Key::GifErrRequest, &[(\"error\", &error.to_string())])", "all": True},
            {"literal": "GIPHY rejected the API key (error {code}).", "new": "i18n::f(Key::GifErrKeyRejected, &[(\"code\", &code.to_string())])"},
            {"literal": "Invalid GIPHY response: {error}", "new": "i18n::f(Key::GifErrResponse, &[(\"error\", &error.to_string())])"},
            {"literal": "GIPHY: {message}", "new": "i18n::f(Key::GifErrMessage, &[(\"message\", message)])"},
            {"literal": "Saving {done} of {total}", "new": "i18n::f(Key::ToastSavingProgress, &[(\"done\", &done.to_string()), (\"total\", &total.to_string())])"},
        ],
    },
    {
        "name": "worker-literals",
        "file": "src/backend/worker.rs",
        "literals": [
            {"exact": "Not connected to WhatsApp", "new": "i18n::t(Key::ToastNotConnected)", "all": True},
            {"exact": "Not connected to WhatsApp yet", "new": "i18n::t(Key::ToastNotConnectedYet)"},
            {"exact": "Polls cannot be sent to this chat.", "new": "i18n::t(Key::PollErrCantSendHere)"},
            {"exact": "Poll creation in disappearing-message chats is not supported yet.", "new": "i18n::t(Key::PollErrDisappearing)"},
            {"exact": "Could not load the group recipients", "new": "i18n::t(Key::PollErrRecipients)"},
            {"exact": "Could not send the poll. Please try again.", "new": "i18n::t(Key::PollErrSend)"},
            {"exact": "The account changed while the poll was being sent.", "new": "i18n::t(Key::PollErrAccountChanged)"},
            {"exact": "The poll was sent, but its voting key could not be saved.", "new": "i18n::t(Key::PollErrKeyNotSaved)"},
            {"exact": "This poll is not ready for voting. Reconnect and try again.", "new": "i18n::t(Key::PollErrNotReady)"},
            {"exact": "Could not send your vote. Please try again.", "new": "i18n::t(Key::PollErrVoteSend)"},
            {"exact": "Your vote was sent, but could not be saved locally.", "new": "i18n::t(Key::PollErrVoteNotSaved)"},
            {"exact": "Waiting for this message. Open WhatsApp on your phone", "new": "i18n::t(Key::ChatWaitingMessage)"},
            {"exact": "Your phone did not send older messages. Check that it is online", "new": "i18n::t(Key::ToastNoOlder)"},
            {"exact": "Starred", "new": "i18n::t(Key::ToastStarred)"},
            {"exact": "Star removed", "new": "i18n::t(Key::ToastStarRemoved)"},
            {"exact": "Send to WhatsApp", "new": "i18n::t(Key::ChatSendToWhatsApp)"},
            {"exact": "Add a sticker pack", "new": "i18n::t(Key::DialogAddPack)"},
            {"exact": "Sticker packs", "new": "i18n::t(Key::DialogStickerPacks)"},
            {"exact": "Could not update privacy settings.", "new": "i18n::t(Key::ToastPrivacyFailed)"},
            {"exact": "The download failed, so it was not saved", "new": "i18n::t(Key::ToastDownloadNotSaved)"},
            {"exact": "Could not leave the channel.", "new": "i18n::t(Key::ToastLeaveChannelFailed)"},
            {"exact": "Could not leave the group.", "new": "i18n::t(Key::ToastLeaveGroupFailed)"},
            {"exact": "Message scheduled", "new": "i18n::t(Key::ToastScheduled)"},
            {"exact": "Wait for the current forward to finish", "new": "i18n::t(Key::ToastForwardBusy)"},
            {"exact": "This message is not stored on this computer", "new": "i18n::t(Key::ToastNotStored)", "all": True},
            {"exact": "This message is not on this computer", "new": "i18n::t(Key::ToastNotOnComputer)"},
            {"exact": "This message cannot be forwarded", "new": "i18n::t(Key::ToastCannotForward)"},
            {"exact": "The original message data is not available to forward", "new": "i18n::t(Key::ToastForwardNoData)"},
            {"exact": "The original message data could not be read", "new": "i18n::t(Key::ToastForwardUnreadable)"},
            {"exact": "Attachment download keys are missing", "new": "i18n::t(Key::ToastKeysMissing)"},
            {"exact": "This message has no downloadable file", "new": "i18n::t(Key::ToastNoFile)"},
            {"exact": "No Downloads folder on this computer", "new": "i18n::t(Key::ToastNoDownloads)", "all": True},
            {"exact": "A save dialog is already open", "new": "i18n::t(Key::ToastSaveDialogOpen)"},
            {"exact": "Save attachment", "new": "i18n::t(Key::DialogSaveAttachment)"},
            {"exact": "a single file", "new": "i18n::t(Key::DialogSingleFile)"},
            {"exact": "Save cancelled", "new": "i18n::t(Key::ToastSaveCancelled)"},
            {"exact": "No longer available on WhatsApp's servers", "new": "i18n::t(Key::ChatFileGone)", "all": True},
            {"exact": "Clipboard image data is invalid", "new": "i18n::t(Key::ToastClipboardInvalid)"},
            {"exact": "Could not encode the attachment", "new": "i18n::t(Key::ToastEncodeFailed)"},
            {"exact": "The application is shutting down", "new": "i18n::t(Key::ToastShuttingDown)"},
            {"exact": "Could not save the group message recipients", "new": "i18n::t(Key::ToastRecipientsSaveFailed)"},
            {"exact": "Another WhatsApp Web session replaced this one", "new": "i18n::t(Key::ErrStreamReplaced)"},
            {"exact": "WhatsApp rejected this version of WhatsFast. Update the app", "new": "i18n::t(Key::ErrClientOutdated)"},
            {"exact": "You", "new": "i18n::t(Key::DisplayYou)", "all": True},
            {"exact": "Group", "new": "i18n::t(Key::KindGroup)", "all": True},
            {"exact": "Document", "new": "i18n::t(Key::KindDocument)"},
            {"exact": "Live location", "new": "i18n::t(Key::KindLiveLocation)"},
            {"exact": "Contact", "new": "i18n::t(Key::KindContact)"},
            {"exact": "Poll", "new": "i18n::t(Key::KindPoll)"},
            {"exact": "group invite", "new": "i18n::t(Key::KindGroupInvite)"},
            {"exact": "sticker pack", "new": "i18n::t(Key::KindStickerPack)"},
            {"exact": "interactive message", "new": "i18n::t(Key::KindInteractive)"},
            {"exact": "animated sticker", "new": "i18n::t(Key::KindAnimatedSticker)"},
            {"exact": "GIF search needs a GIPHY API key.", "new": "i18n::t(Key::GifErrNoKey)"},
            {"exact": "GIPHY response contained no results", "new": "i18n::t(Key::GifErrNoResults)"},
        ],
    },
    {
        "name": "polls-literals",
        "file": "src/backend/worker/polls.rs",
        "literals": [
            {"exact": "Not connected to WhatsApp", "new": "i18n::t(Key::ToastNotConnected)", "all": True},
            {"exact": "No longer available on WhatsApp's servers", "new": "i18n::t(Key::ChatFileGone)", "all": True},
        ],
    },
]

data["sites"].extend(sites)
DATA.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
print(f"rows: {len(data['strings'])}, sites: {len(data['sites'])}")
