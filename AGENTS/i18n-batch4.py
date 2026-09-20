"""Lote 4: login, mod (toasts/banner), polls, update, viewer."""
import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
DATA = ROOT / "AGENTS" / "i18n-strings.json"
data = json.loads(DATA.read_text(encoding="utf-8"))

rows = [
    # login
    ("LoginTagline", "A native WhatsApp client.", "Un cliente de WhatsApp nativo."),
    ("LoginConnecting", "Connecting to WhatsApp…", "Conectando con WhatsApp…"),
    ("LoginLinkedWaiting", "Linked. Waiting for your chats…", "Vinculado. Esperando tus chats…"),
    ("LoginUnlinkedRequestingCode", "This computer was unlinked from your phone. Requesting a new code.", "Este equipo se desvinculó de tu teléfono. Se está pidiendo un código nuevo."),
    ("LoginRequestingNewCode", "Requesting a new code…", "Pidiendo un código nuevo…"),
    ("LoginTryAgain", "Try again", "Volver a intentar"),
    ("LoginRequestingCodeFor", "Requesting a code for +{phone}…", "Pidiendo un código para +{phone}…"),
    ("LoginWaitingForCode", "Waiting for a code from WhatsApp…", "Esperando un código de WhatsApp…"),
    ("LoginUnofficialWarning", "Unofficial client. Using it may be against WhatsApp's terms of service.", "Cliente no oficial. Usarlo puede ir contra los términos de servicio de WhatsApp."),
    ("LoginLinkThisComputer", "Link this computer", "Vincular este equipo"),
    ("LoginOpenWhatsApp", "Open WhatsApp on your phone", "Abre WhatsApp en tu teléfono"),
    ("LoginTapMenu", "Tap Menu or Settings, then Linked devices", "Toca Menú o Ajustes y luego Dispositivos vinculados"),
    ("LoginTapLinkDevice", "Tap Link a device and point the phone at this code", "Toca Vincular un dispositivo y apunta el teléfono a este código"),
    ("LoginLinkWithPhone", "Link with phone number instead", "Vincular con un número de teléfono"),
    ("LoginEnterCode", "Enter this code on your phone", "Escribe este código en tu teléfono"),
    ("LoginForPhone", "for +{phone}", "para +{phone}"),
    ("LoginTapLinkPhone", "Tap Link a device, then Link with phone number instead", "Toca Vincular un dispositivo y luego Vincular con un número de teléfono"),
    ("LoginCopyCode", "Copy code", "Copiar código"),
    # barra de estado y toasts
    ("DropToSendTo", "Drop to send to {name}", "Suelta para enviar a {name}"),
    ("HistoryLoadingPercent", "Loading chat history… {percent}%", "Cargando el historial… {percent}%"),
    ("HistoryLoading", "Loading chat history…", "Cargando el historial…"),
    ("OfflineReconnecting", "Offline ({reason}). Reconnecting…", "Sin conexión ({reason}). Reconectando…"),
    ("NotLinkedPhone", "Not linked to a phone", "Sin vincular a un teléfono"),
    ("CommonRetry", "Retry", "Reintentar"),
    ("StatusShowChatList", "Show the chat list (Ctrl+B)", "Mostrar la lista de chats (Ctrl+B)"),
    # encuestas
    ("PollCreateTitle", "Create poll", "Crear encuesta"),
    ("CommonClose", "Close", "Cerrar"),
    ("PollQuestion", "Question", "Pregunta"),
    ("PollAskQuestion", "Ask a question", "Escribe una pregunta"),
    ("PollAnswers", "Answers", "Respuestas"),
    ("PollAnswerHint", "Answer {number}", "Respuesta {number}"),
    ("PollRemoveAnswer", "Remove answer", "Quitar respuesta"),
    ("PollAddAnswer", "Add answer", "Añadir respuesta"),
    ("PollAllowMultiple", "Allow multiple answers", "Permitir varias respuestas"),
    ("PollSending", "Sending…", "Enviando…"),
    ("PollSend", "Send poll", "Enviar encuesta"),
    ("PollSelectOne", "Select one answer", "Elige una respuesta"),
    ("PollSelectMany", "Select answers", "Elige las respuestas"),
    ("PollSendingVote", "Sending vote…", "Enviando voto…"),
    ("PollWaitingPhone", "Waiting for your phone · earlier votes may be missing", "Esperando a tu teléfono · pueden faltar votos anteriores"),
    ("PollLoadingVotes", "Loading earlier votes from your phone…", "Cargando votos anteriores de tu teléfono…"),
    ("PollVotesNotLoaded", "Earlier votes have not been loaded yet", "Todavía no se cargaron los votos anteriores"),
    ("PollVotingKeyMissing", "Voting key unavailable · use your phone", "Llave de votación no disponible · usa tu teléfono"),
    ("PollReconnectToVote", "Reconnect to vote", "Vuelve a conectar para votar"),
    # actualización
    ("UpdateNewVersion", "There's a new version available", "Hay una versión nueva disponible"),
    ("UpdateDownloadFromGitHub", "Download from GitHub", "Descargar desde GitHub"),
    ("UpdateUpdate", "Update", "Actualizar"),
    # visor de fotos
    ("ViewerCouldNotDisplay", "Could not display this picture.", "No se pudo mostrar esta imagen."),
    ("ViewerClose", "Close photo", "Cerrar foto"),
    ("ViewerPrevious", "Previous photo", "Foto anterior"),
    ("ViewerNext", "Next photo", "Foto siguiente"),
]

existing = {r["key"] for r in data["strings"]}
for key, en, es in rows:
    assert key not in existing, f"duplicate key {key}"
    data["strings"].append({"key": key, "en": en, "es": es})

sites = [
    # ---- imports
    {
        "file": "src/ui/login.rs",
        "old": "use crate::app::App;\nuse crate::backend::LinkStatus;",
        "new": "use crate::app::App;\nuse crate::backend::LinkStatus;\nuse crate::i18n::{self, Key};",
    },
    {
        "file": "src/ui/mod.rs",
        "old": "use crate::app::App;\nuse crate::backend::LinkStatus;",
        "new": "use crate::app::App;\nuse crate::backend::LinkStatus;\nuse crate::i18n::{self, Key};",
    },
    {
        "file": "src/ui/polls.rs",
        "old": "use crate::app::App;\nuse crate::model::{Action, Content, Message, PollState};",
        "new": "use crate::app::App;\nuse crate::i18n::{self, Key};\nuse crate::model::{Action, Content, Message, PollState};",
    },
    {
        "file": "src/ui/update.rs",
        "old": "use crate::app::App;\nuse crate::model::Action;",
        "new": "use crate::app::App;\nuse crate::i18n::{self, Key};\nuse crate::model::Action;",
    },
    {
        "file": "src/ui/viewer.rs",
        "old": "use crate::app::App;\nuse crate::model::{Action, ChatId, Content, Message};",
        "new": "use crate::app::App;\nuse crate::i18n::{self, Key};\nuse crate::model::{Action, ChatId, Content, Message};",
    },
    # ---- login: expresiones con format!
    {
        "file": "src/ui/login.rs",
        "old": "                    &format!(\"Requesting a code for +{phone}…\"),",
        "new": "                    &i18n::f(Key::LoginRequestingCodeFor, &[(\"phone\", phone.as_str())]),",
    },
    {
        "file": "src/ui/login.rs",
        "old": "            format!(\"for +{phone}\"),",
        "new": "            i18n::f(Key::LoginForPhone, &[(\"phone\", phone)]),",
    },
    # ---- mod: expresiones con format!
    {
        "file": "src/ui/mod.rs",
        "old": "                            format!(\"Drop to send to {name}\"),",
        "new": "                            i18n::f(Key::DropToSendTo, &[(\"name\", name.as_str())]),",
    },
    {
        "file": "src/ui/mod.rs",
        "old": "                Some(percent) => format!(\"Loading chat history… {percent}%\"),",
        "new": "                Some(percent) => i18n::f(\n                    Key::HistoryLoadingPercent,\n                    &[(\"percent\", &percent.to_string())],\n                ),",
    },
    {
        "file": "src/ui/mod.rs",
        "old": "            format!(\"Offline ({reason}). Reconnecting…\"),",
        "new": "            i18n::f(Key::OfflineReconnecting, &[(\"reason\", reason)]),",
    },
    # ---- polls: hint con format!
    {
        "file": "src/ui/polls.rs",
        "old": "                                .hint_text(format!(\"Answer {}\", index + 1))",
        "new": "                                .hint_text(i18n::f(\n                                    Key::PollAnswerHint,\n                                    &[(\"number\", &(index + 1).to_string())],\n                                ))",
    },
    # ---- literales
    {
        "name": "login-literals",
        "file": "src/ui/login.rs",
        "literals": [
            {"exact": "A native WhatsApp client.", "new": "i18n::t(Key::LoginTagline)"},
            {"exact": "Connecting to WhatsApp…", "new": "i18n::t(Key::LoginConnecting)"},
            {"exact": "Linked. Waiting for your chats…", "new": "i18n::t(Key::LoginLinkedWaiting)"},
            {"starts_with": "This computer was unlinked from your phone.", "new": "i18n::t(Key::LoginUnlinkedRequestingCode)"},
            {"exact": "Requesting a new code…", "new": "i18n::t(Key::LoginRequestingNewCode)"},
            {"exact": "Try again", "new": "i18n::t(Key::LoginTryAgain)"},
            {"exact": "Waiting for a code from WhatsApp…", "new": "i18n::t(Key::LoginWaitingForCode)"},
            {"starts_with": "Unofficial client.", "new": "i18n::t(Key::LoginUnofficialWarning)"},
            {"exact": "Link this computer", "new": "i18n::t(Key::LoginLinkThisComputer)"},
            {"exact": "Open WhatsApp on your phone", "new": "i18n::t(Key::LoginOpenWhatsApp)", "all": True},
            {"exact": "Tap Menu or Settings, then Linked devices", "new": "i18n::t(Key::LoginTapMenu)", "all": True},
            {"exact": "Tap Link a device and point the phone at this code", "new": "i18n::t(Key::LoginTapLinkDevice)"},
            {"exact": "Link with phone number instead", "new": "i18n::t(Key::LoginLinkWithPhone)"},
            {"exact": "Enter this code on your phone", "new": "i18n::t(Key::LoginEnterCode)"},
            {"exact": "Tap Link a device, then Link with phone number instead", "new": "i18n::t(Key::LoginTapLinkPhone)"},
            {"exact": "Copy code", "new": "i18n::t(Key::LoginCopyCode)"},
        ],
    },
    {
        "name": "mod-literals",
        "file": "src/ui/mod.rs",
        "literals": [
            {"exact": "Loading chat history…", "new": "i18n::t(Key::HistoryLoading)"},
            {"exact": "Connecting to WhatsApp…", "new": "i18n::t(Key::LoginConnecting)"},
            {"exact": "Not linked to a phone", "new": "i18n::t(Key::NotLinkedPhone)"},
            {"exact": "Retry", "new": "i18n::t(Key::CommonRetry)"},
            {"exact": "Show the chat list (Ctrl+B)", "new": "i18n::t(Key::StatusShowChatList)"},
        ],
    },
    {
        "name": "polls-literals",
        "file": "src/ui/polls.rs",
        "literals": [
            {"exact": "Create poll", "new": "i18n::t(Key::PollCreateTitle)"},
            {"exact": "Close", "new": "i18n::t(Key::CommonClose)"},
            {"exact": "Question", "new": "i18n::t(Key::PollQuestion)"},
            {"exact": "Ask a question", "new": "i18n::t(Key::PollAskQuestion)"},
            {"exact": "Answers", "new": "i18n::t(Key::PollAnswers)"},
            {"exact": "Remove answer", "new": "i18n::t(Key::PollRemoveAnswer)"},
            {"exact": "Add answer", "new": "i18n::t(Key::PollAddAnswer)"},
            {"exact": "Allow multiple answers", "new": "i18n::t(Key::PollAllowMultiple)"},
            {"exact": "Cancel", "new": "i18n::t(Key::DialogCancel)"},
            {"exact": "Sending…", "new": "i18n::t(Key::PollSending)"},
            {"exact": "Send poll", "new": "i18n::t(Key::PollSend)"},
            {"exact": "Select one answer", "new": "i18n::t(Key::PollSelectOne)"},
            {"exact": "Select answers", "new": "i18n::t(Key::PollSelectMany)"},
            {"exact": "Sending vote…", "new": "i18n::t(Key::PollSendingVote)"},
            {"starts_with": "Waiting for your phone", "new": "i18n::t(Key::PollWaitingPhone)"},
            {"starts_with": "Loading earlier votes", "new": "i18n::t(Key::PollLoadingVotes)"},
            {"starts_with": "Earlier votes have not", "new": "i18n::t(Key::PollVotesNotLoaded)"},
            {"starts_with": "Voting key unavailable", "new": "i18n::t(Key::PollVotingKeyMissing)"},
            {"exact": "Reconnect to vote", "new": "i18n::t(Key::PollReconnectToVote)"},
        ],
    },
    {
        "name": "update-literals",
        "file": "src/ui/update.rs",
        "literals": [
            {"exact": "There's a new version available", "new": "i18n::t(Key::UpdateNewVersion)"},
            {"exact": "Download from GitHub", "new": "i18n::t(Key::UpdateDownloadFromGitHub)"},
            {"exact": "Retry", "new": "i18n::t(Key::CommonRetry)"},
            {"exact": "Update", "new": "i18n::t(Key::UpdateUpdate)"},
        ],
    },
    {
        "name": "viewer-literals",
        "file": "src/ui/viewer.rs",
        "literals": [
            {"starts_with": "Could not display this picture.", "new": "i18n::t(Key::ViewerCouldNotDisplay)"},
            {"exact": "Close photo", "new": "i18n::t(Key::ViewerClose)"},
            {"exact": "Previous photo", "new": "i18n::t(Key::ViewerPrevious)"},
            {"exact": "Next photo", "new": "i18n::t(Key::ViewerNext)"},
        ],
    },
]

data["sites"].extend(sites)
DATA.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
print(f"rows: {len(data['strings'])}, sites: {len(data['sites'])}")
