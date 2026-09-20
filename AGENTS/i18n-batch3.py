"""Lote 3: Ajustes (ui/settings.rs) + atajos (ui/keys.rs) + label a 2 líneas + sección Idioma."""
import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
DATA = ROOT / "AGENTS" / "i18n-strings.json"
data = json.loads(DATA.read_text(encoding="utf-8"))

rows = [
    ("CommonBack", "Back (Esc)", "Volver (Esc)"),
    ("CommonOpen", "Open", "Abrir"),
    ("CommonQuit", "Quit", "Salir"),
    ("CommonShortcuts", "Shortcuts", "Atajos"),
    ("SettingsTitle", "Settings", "Ajustes"),
    ("SettingsSectionAppearance", "Appearance", "Apariencia"),
    ("SettingsThemeLabel", "Theme", "Tema"),
    ("SettingsThemeOmarchyHint", "Follow system uses your Omarchy colours.", "Seguir el sistema usa tus colores de Omarchy."),
    ("SettingsThemeSystemHint", "Follow system uses your desktop's light or dark appearance.", "Seguir el sistema usa el modo claro u oscuro de tu escritorio."),
    ("SettingsOpenThemesFolder", "Open themes folder", "Abrir carpeta de temas"),
    ("SettingsWallpaperLabel", "Chat wallpaper", "Fondo de chat"),
    (
        "SettingsWallpaperHint",
        "Auto picks a doodle from the theme colours. You can force Black 1 to White 3. Right-click the open chat and choose Next wallpaper to step through the three doodles for that family.",
        "Automático elige un fondo según los colores del tema. Puedes forzar de Negro 1 a Blanco 3. Haz clic derecho en el chat abierto y elige Siguiente fondo para recorrer los tres fondos de esa familia.",
    ),
    ("SettingsZoomLabel", "Zoom", "Zoom"),
    ("SettingsZoomHint", "You can also use Ctrl+plus and Ctrl+minus.", "También puedes usar Ctrl+más y Ctrl+menos."),
    ("SettingsZoomLarger", "Larger", "Más grande"),
    ("SettingsZoomSmaller", "Smaller", "Más pequeño"),
    ("SettingsSectionChats", "Chats", "Chats"),
    ("SettingsEnterSends", "Enter sends", "Enter envía"),
    ("SettingsEnterSendsHint", "When off, Enter adds a line and Ctrl+Enter sends.", "Si está desactivado, Enter añade una línea y Ctrl+Enter envía."),
    (
        "SettingsReceiptsOffNote",
        "Read receipts are disabled for your WhatsApp account (Settings Privacy). Direct chats will not send them. When this switch is on, groups still do. Read state syncs between your devices either way.",
        "Las confirmaciones de lectura están desactivadas en tu cuenta de WhatsApp (Ajustes, Privacidad). Los chats directos no las envían. Con este interruptor activado, los grupos sí. El estado de lectura se sincroniza entre tus dispositivos igual.",
    ),
    (
        "SettingsReceiptsOnNote",
        "Let people see when you read messages or play voice messages on this copy. Your WhatsApp account setting in Privacy still applies. Read state syncs between your devices either way.",
        "Deja que los demás vean cuándo lees mensajes o reproduces mensajes de voz en esta copia. El ajuste de tu cuenta de WhatsApp en Privacidad sigue aplicando. El estado de lectura se sincroniza entre tus dispositivos igual.",
    ),
    ("SettingsSendReceipts", "Send read receipts", "Enviar confirmaciones de lectura"),
    ("SettingsSendTyping", "Show when you are typing", "Mostrar cuando escribes"),
    ("SettingsAutoDownload", "Download attachments automatically", "Descargar adjuntos automáticamente"),
    (
        "SettingsAutoDownloadHint",
        "Download pictures, videos, voice messages, and documents up to 64 MB when they enter view. When off, click a file to download it.",
        "Descarga fotos, videos, mensajes de voz y documentos de hasta 64 MB cuando entran en pantalla. Si está desactivado, haz clic en un archivo para descargarlo.",
    ),
    ("SettingsHistoryLabel", "Download older history in the background", "Descargar historial anterior en segundo plano"),
    (
        "SettingsHistoryHint",
        "Slowly fetch older messages and their files (up to 64 MB) so scrolling up does not hit WhatsApp's rate limit. Failed files are asked again with a long wait, for up to 30 days. Recent and pinned covers every pinned chat plus the ten most recently active chats that are not pinned.",
        "Descarga despacio los mensajes anteriores y sus archivos (hasta 64 MB) para que desplazarte hacia arriba no choque con el límite de WhatsApp. Los archivos que fallan se vuelven a pedir tras una espera larga, hasta 30 días. Recientes y fijados abarca cada chat fijado más los diez chats activos más recientes que no estén fijados.",
    ),
    ("SettingsShowSenderPictures", "Show sender pictures in every chat", "Mostrar la foto del remitente en todos los chats"),
    ("SettingsShowSenderPicturesHint", "WhatsApp shows them in groups only.", "WhatsApp solo las muestra en grupos."),
    ("SettingsNamesFromContacts", "Names from your address book", "Nombres de tu agenda"),
    (
        "SettingsNamesFromContactsHint",
        "Prefer saved contact names. When off, prefer public WhatsApp profile names. This applies throughout the app.",
        "Prefiere los nombres guardados en tu agenda. Si está desactivado, prefiere los nombres públicos del perfil de WhatsApp. Se aplica en toda la app.",
    ),
    ("SettingsSaveContacts", "Save contacts to the phone's address book", "Guardar contactos en la agenda del teléfono"),
    (
        "SettingsSaveContactsHint",
        "Also add contacts saved here to your phone's address book. When off, they remain WhatsApp contacts. Names sync to linked devices either way.",
        "También añade a la agenda del teléfono los contactos guardados aquí. Si está desactivado, quedan como contactos de WhatsApp. Los nombres se sincronizan entre dispositivos igual.",
    ),
    ("SettingsForwardInOrder", "Forward messages in order", "Reenviar mensajes en orden"),
    (
        "SettingsForwardInOrderHint",
        "Send a forwarded batch one message at a time, starting each one when the message before it shows its first tick. Mixed text, pictures, and videos then arrive in their original order. When off, they send together and may arrive out of order.",
        "Envía un reenvío múltiple de a un mensaje, y empieza cada uno cuando el anterior muestra su primer tilde. Así el texto, las fotos y los videos mezclados llegan en su orden original. Si está desactivado, se envían juntos y pueden llegar desordenados.",
    ),
    ("SettingsShowPollButton", "Show the create poll button", "Mostrar el botón Crear encuesta"),
    (
        "SettingsShowPollButtonHint",
        "Add the Create poll button beside the composer. When off, the button is hidden and the polls already in a chat keep working.",
        "Añade el botón Crear encuesta junto al cuadro de mensaje. Si está desactivado, el botón se oculta y las encuestas que ya están en un chat siguen funcionando.",
    ),
    ("SettingsShowShortcutHints", "Show shortcut hints", "Mostrar sugerencias de atajos"),
    ("SettingsSectionPrivacy", "Privacy", "Privacidad"),
    ("SettingsPrivacyLoadFailed", "Could not load privacy settings. They load again when WhatsFast reconnects.", "No se pudo cargar la privacidad. Vuelve a cargarse cuando WhatsFast se reconecte."),
    ("SettingsSectionWindow", "Window", "Ventana"),
    ("SettingsKeepRunning", "Keep running when the window closes", "Seguir activo al cerrar la ventana"),
    ("SettingsKeepRunningHint", "Keep WhatsFast linked in the system tray. Quit from the tray menu or with Ctrl+Q.", "Mantiene WhatsFast vinculado en la bandeja del sistema. Sal de la bandeja o con Ctrl+Q."),
    ("SettingsNotify", "Notify about new messages", "Notificar mensajes nuevos"),
    (
        "SettingsNotifyHint",
        "Show desktop notifications when the window is hidden, in the background, or showing another chat. Muted chats do not notify you.",
        "Muestra notificaciones del escritorio cuando la ventana está oculta, en segundo plano o mostrando otro chat. Los chats silenciados no notifican.",
    ),
    ("SettingsHideSidebar", "Hide the sidebar completely", "Ocultar la barra lateral por completo"),
    (
        "SettingsHideSidebarHint",
        "On: hiding the sidebar (Ctrl+B) leaves nothing behind. Off: it narrows to the chat pictures, so search, archived chats, and settings stay one click away.",
        "Activado: ocultar la barra lateral (Ctrl+B) no deja nada. Desactivado: se reduce a las fotos de los chats, así la búsqueda, los chats archivados y los ajustes quedan a un clic.",
    ),
    ("SettingsGiphyKey", "GIPHY API key", "Clave de API de GIPHY"),
    ("SettingsGiphyKeyHintBuiltIn", "Used for GIF search. This build includes a key. Enter a key from developers.giphy.com to replace it.", "Se usa para buscar GIF. Esta compilación incluye una clave. Ingresa una clave de developers.giphy.com para reemplazarla."),
    ("SettingsGiphyKeyHintRequired", "Required for GIF search. Get a free key from developers.giphy.com.", "Necesaria para buscar GIF. Consigue una clave gratis en developers.giphy.com."),
    ("SettingsSectionAccount", "Account", "Cuenta"),
    ("SettingsLinkedDevice", "Linked device", "Dispositivo vinculado"),
    ("SettingsUnlink", "Unlink this computer", "Desvincular este equipo"),
    ("SettingsSectionFiles", "Files", "Archivos"),
    ("SettingsMessageArchive", "Message archive", "Archivo de mensajes"),
    ("SettingsOpenFolder", "Open folder", "Abrir carpeta"),
    ("SettingsDownloadedAttachments", "Downloaded attachments", "Adjuntos descargados"),
    ("SettingsAskWhereToSave", "Ask where to save each file", "Preguntar dónde guardar cada archivo"),
    (
        "SettingsAskWhereToSaveHint",
        "Open the system save dialog for every attachment you save from the selection bar, so you choose the folder and the file name. When off, saved files go to your Downloads folder.",
        "Abre el diálogo de guardado del sistema para cada adjunto que guardas desde la barra de selección, así eliges la carpeta y el nombre del archivo. Si está desactivado, los archivos van a tu carpeta de Descargas.",
    ),
    ("SettingsLogOfRun", "Log of this run", "Registro de esta ejecución"),
    ("SettingsSectionAbout", "About", "Acerca de"),
    ("SettingsAboutLine", "A native WhatsApp client built with Rust, egui, and whatsapp-rust.", "Un cliente de WhatsApp nativo hecho con Rust, egui y whatsapp-rust."),
    ("SettingsCheckUpdates", "Check for updates", "Buscar actualizaciones"),
    ("SettingsCheckUpdatesHint", "Ask GitHub once a day whether a newer WhatsFast release exists. The request identifies only WhatsFast and its version.", "Consulta a GitHub una vez al día si hay una versión más nueva de WhatsFast. La solicitud identifica solo a WhatsFast y su versión."),
    ("SettingsDownloadUpdates", "Download updates automatically", "Descargar actualizaciones automáticamente"),
    (
        "SettingsDownloadUpdatesHint",
        "Download and verify new releases in the background. A toast offers Update; one click installs and restarts. If you skip the toast, the next start of WhatsFast installs the verified file. Native packages and Flatpak update through their package manager.",
        "Descarga y verifica versiones nuevas en segundo plano. Un aviso ofrece Actualizar; con un clic se instala y se reinicia. Si ignoras el aviso, la próxima vez que abras WhatsFast se instala el archivo verificado. Los paquetes nativos y Flatpak se actualizan con su gestor de paquetes.",
    ),
    ("SettingsLanguageLabel", "Interface language", "Idioma de la interfaz"),
    ("ShortcutSearchChats", "Search chats", "Buscar chats"),
    ("ShortcutSearchMessages", "Search messages in the open chat", "Buscar mensajes en el chat abierto"),
    ("ShortcutFocusComposer", "Focus the message input", "Ir al cuadro de mensaje"),
    ("ShortcutPreviousNextChat", "Previous / next chat", "Chat anterior / siguiente"),
    ("ShortcutSend", "Send (Shift+Enter for a new line)", "Enviar (Shift+Enter para una línea nueva)"),
    (
        "ShortcutDismiss",
        "Dismiss the current action, return from search, or close the chat",
        "Cancelar la acción actual, salir de la búsqueda o cerrar el chat",
    ),
    ("ShortcutPhotoViewer", "Photo viewer", "Visor de fotos"),
    (
        "ShortcutPhotoViewerKeys",
        "Wheel zooms, drag moves, ← / → previous or next photo",
        "La rueda acerca, el arrastre mueve, ← / → foto anterior o siguiente",
    ),
    ("ShortcutPaste", "Paste text, or send a picture from the clipboard", "Pegar texto o enviar una imagen del portapapeles"),
    ("ShortcutChatList", "Show or hide the chat list", "Mostrar u ocultar la lista de chats"),
    ("ShortcutNewest", "Jump to the newest message", "Ir al mensaje más reciente"),
    ("ShortcutZoom", "Zoom in / out", "Acercar / alejar"),
    ("ShortcutResetZoom", "Reset zoom", "Restablecer zoom"),
    ("ShortcutThisList", "This list", "Esta lista"),
    ("ShortcutCloseWindow", "Close the window (WhatsFast remains in the tray)", "Cerrar la ventana (WhatsFast queda en la bandeja)"),
]

existing = {r["key"] for r in data["strings"]}
for key, en, es in rows:
    assert key not in existing, f"duplicate key {key}"
    data["strings"].append({"key": key, "en": en, "es": es})

settings_literals = [
    {"exact": "Back (Esc)", "new": "i18n::t(Key::CommonBack)"},
    {"exact": "Settings", "new": "i18n::t(Key::SettingsTitle)"},
    {"exact": "Appearance", "new": "i18n::t(Key::SettingsSectionAppearance)"},
    {"starts_with": "Follow system uses your Omarchy colours.", "new": "i18n::t(Key::SettingsThemeOmarchyHint)"},
    {"starts_with": "Follow system uses your desktop's", "new": "i18n::t(Key::SettingsThemeSystemHint)"},
    {"exact": "Theme", "new": "i18n::t(Key::SettingsThemeLabel)", "all": True},
    {"exact": "Open themes folder", "new": "i18n::t(Key::SettingsOpenThemesFolder)"},
    {"exact": "Chat wallpaper", "new": "i18n::t(Key::SettingsWallpaperLabel)", "all": True},
    {"starts_with": "Auto picks a doodle", "new": "i18n::t(Key::SettingsWallpaperHint)"},
    {"exact": "Auto", "new": "i18n::t(Key::SettingsWallpaperAuto)"},
    {"exact": "Zoom", "new": "i18n::t(Key::SettingsZoomLabel)"},
    {"starts_with": "You can also use Ctrl+plus", "new": "i18n::t(Key::SettingsZoomHint)"},
    {"exact": "Larger", "new": "i18n::t(Key::SettingsZoomLarger)"},
    {"exact": "Smaller", "new": "i18n::t(Key::SettingsZoomSmaller)"},
    {"exact": "Chats", "new": "i18n::t(Key::SettingsSectionChats)"},
    {"exact": "Enter sends", "new": "i18n::t(Key::SettingsEnterSends)"},
    {"starts_with": "When off, Enter adds a line", "new": "i18n::t(Key::SettingsEnterSendsHint)"},
    {"starts_with": "Read receipts are disabled", "new": "i18n::t(Key::SettingsReceiptsOffNote)"},
    {"starts_with": "Let people see when you read messages", "new": "i18n::t(Key::SettingsReceiptsOnNote)"},
    {"exact": "Send read receipts", "new": "i18n::t(Key::SettingsSendReceipts)"},
    {"exact": "Show when you are typing", "new": "i18n::t(Key::SettingsSendTyping)"},
    {"exact": "Download attachments automatically", "new": "i18n::t(Key::SettingsAutoDownload)"},
    {"starts_with": "Download pictures, videos, voice messages", "new": "i18n::t(Key::SettingsAutoDownloadHint)"},
    {"exact": "Download older history in the background", "new": "i18n::t(Key::SettingsHistoryLabel)", "all": True},
    {"starts_with": "Slowly fetch older messages", "new": "i18n::t(Key::SettingsHistoryHint)"},
    {"exact": "Show sender pictures in every chat", "new": "i18n::t(Key::SettingsShowSenderPictures)"},
    {"starts_with": "WhatsApp shows them in groups only.", "new": "i18n::t(Key::SettingsShowSenderPicturesHint)"},
    {"exact": "Names from your address book", "new": "i18n::t(Key::SettingsNamesFromContacts)"},
    {"starts_with": "Prefer saved contact names.", "new": "i18n::t(Key::SettingsNamesFromContactsHint)"},
    {"exact": "Save contacts to the phone's address book", "new": "i18n::t(Key::SettingsSaveContacts)"},
    {"starts_with": "Also add contacts saved here", "new": "i18n::t(Key::SettingsSaveContactsHint)"},
    {"exact": "Forward messages in order", "new": "i18n::t(Key::SettingsForwardInOrder)"},
    {"starts_with": "Send a forwarded batch", "new": "i18n::t(Key::SettingsForwardInOrderHint)"},
    {"exact": "Show the create poll button", "new": "i18n::t(Key::SettingsShowPollButton)"},
    {"starts_with": "Add the Create poll button", "new": "i18n::t(Key::SettingsShowPollButtonHint)"},
    {"exact": "Show shortcut hints", "new": "i18n::t(Key::SettingsShowShortcutHints)"},
    {"exact": "Privacy", "new": "i18n::t(Key::SettingsSectionPrivacy)"},
    {"starts_with": "Could not load privacy settings.", "new": "i18n::t(Key::SettingsPrivacyLoadFailed)"},
    {"exact": "Window", "new": "i18n::t(Key::SettingsSectionWindow)"},
    {"exact": "Keep running when the window closes", "new": "i18n::t(Key::SettingsKeepRunning)"},
    {"starts_with": "Keep WhatsFast linked in the system tray.", "new": "i18n::t(Key::SettingsKeepRunningHint)"},
    {"exact": "Notify about new messages", "new": "i18n::t(Key::SettingsNotify)"},
    {"starts_with": "Show desktop notifications", "new": "i18n::t(Key::SettingsNotifyHint)"},
    {"exact": "Hide the sidebar completely", "new": "i18n::t(Key::SettingsHideSidebar)"},
    {"starts_with": "On: hiding the sidebar", "new": "i18n::t(Key::SettingsHideSidebarHint)"},
    {"exact": "GIPHY API key", "new": "i18n::t(Key::SettingsGiphyKey)"},
    {"starts_with": "Used for GIF search.", "new": "i18n::t(Key::SettingsGiphyKeyHintBuiltIn)"},
    {"starts_with": "Required for GIF search.", "new": "i18n::t(Key::SettingsGiphyKeyHintRequired)"},
    {"exact": "Account", "new": "i18n::t(Key::SettingsSectionAccount)"},
    {"exact": "Linked device", "new": "i18n::t(Key::SettingsLinkedDevice)"},
    {"exact": "Unlink this computer", "new": "i18n::t(Key::SettingsUnlink)"},
    {"exact": "Files", "new": "i18n::t(Key::SettingsSectionFiles)"},
    {"exact": "Message archive", "new": "i18n::t(Key::SettingsMessageArchive)"},
    {"exact": "Open folder", "new": "i18n::t(Key::SettingsOpenFolder)", "all": True},
    {"exact": "Downloaded attachments", "new": "i18n::t(Key::SettingsDownloadedAttachments)"},
    {"exact": "Ask where to save each file", "new": "i18n::t(Key::SettingsAskWhereToSave)"},
    {"starts_with": "Open the system save dialog", "new": "i18n::t(Key::SettingsAskWhereToSaveHint)"},
    {"exact": "Log of this run", "new": "i18n::t(Key::SettingsLogOfRun)"},
    {"exact": "About", "new": "i18n::t(Key::SettingsSectionAbout)", "all": True},
    {"starts_with": "A native WhatsApp client built with", "new": "i18n::t(Key::SettingsAboutLine)"},
    {"exact": "Shortcuts", "new": "i18n::t(Key::CommonShortcuts)"},
    {"exact": "Check for updates", "new": "i18n::t(Key::SettingsCheckUpdates)"},
    {"starts_with": "Ask GitHub once a day", "new": "i18n::t(Key::SettingsCheckUpdatesHint)"},
    {"exact": "Download updates automatically", "new": "i18n::t(Key::SettingsDownloadUpdates)"},
    {"starts_with": "Download and verify new releases", "new": "i18n::t(Key::SettingsDownloadUpdatesHint)"},
    {"exact": "Open", "new": "i18n::t(Key::CommonOpen)"},
]

keys_literals = [
    {"exact": "Search chats", "new": "i18n::t(Key::ShortcutSearchChats)"},
    {"exact": "Search messages in the open chat", "new": "i18n::t(Key::ShortcutSearchMessages)"},
    {"exact": "Focus the message input", "new": "i18n::t(Key::ShortcutFocusComposer)"},
    {"exact": "Previous / next chat", "new": "i18n::t(Key::ShortcutPreviousNextChat)"},
    {"exact": "Send (Shift+Enter for a new line)", "new": "i18n::t(Key::ShortcutSend)"},
    {"exact": "Dismiss the current action, return from search, or close the chat", "new": "i18n::t(Key::ShortcutDismiss)"},
    {"exact": "Photo viewer", "new": "i18n::t(Key::ShortcutPhotoViewer)"},
    {"exact": "Wheel zooms, drag moves, ← / → previous or next photo", "new": "i18n::t(Key::ShortcutPhotoViewerKeys)"},
    {"exact": "Paste text, or send a picture from the clipboard", "new": "i18n::t(Key::ShortcutPaste)"},
    {"exact": "Show or hide the chat list", "new": "i18n::t(Key::ShortcutChatList)"},
    {"exact": "Jump to the newest message", "new": "i18n::t(Key::ShortcutNewest)"},
    {"exact": "Settings", "new": "i18n::t(Key::SettingsTitle)"},
    {"exact": "Zoom in / out", "new": "i18n::t(Key::ShortcutZoom)"},
    {"exact": "Reset zoom", "new": "i18n::t(Key::ShortcutResetZoom)"},
    {"exact": "This list", "new": "i18n::t(Key::ShortcutThisList)"},
    {"exact": "Close the window (WhatsFast remains in the tray)", "new": "i18n::t(Key::ShortcutCloseWindow)"},
    {"exact": "Quit", "new": "i18n::t(Key::CommonQuit)"},
]

sites = [
    {"name": "settings-literals", "file": "src/ui/settings.rs", "literals": settings_literals},
    {"name": "keys-literals", "file": "src/ui/keys.rs", "literals": keys_literals},
    {
        "file": "src/ui/settings.rs",
        "old": "use crate::app::App;\nuse crate::model::{Action, Dialog, Page};",
        "new": "use crate::app::App;\nuse crate::i18n::{self, Key, Language};\nuse crate::model::{Action, Dialog, Page};",
    },
    {
        "file": "src/ui/settings.rs",
        "old": "                    section(ui, app, i18n::t(Key::SettingsSectionChats));",
        "new": """                    section(ui, app, i18n::t(Key::SettingsSectionLanguage));
                    widgets::setting_row(
                        ui,
                        &palette,
                        i18n::t(Key::SettingsLanguageLabel),
                        i18n::t(Key::SettingsLanguageHint),
                        |ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {
                                let selected = app.settings.language.label();
                                let response = egui::ComboBox::from_id_salt("language")
                                    .selected_text(" ")
                                    .width(200.0_f32.min(ui.available_width()))
                                    .show_ui(ui, |ui| {
                                        for language in Language::ALL {
                                            if wallpaper_option(
                                                ui,
                                                &palette,
                                                language.label(),
                                                app.settings.language == language,
                                            )
                                            .clicked()
                                            {
                                                app.settings.language = language;
                                                i18n::set_language(language);
                                                app.actions.push(Action::SettingsChanged);
                                            }
                                        }
                                    });
                                let rect = response.response.rect;
                                let text = widgets::line(
                                    ui,
                                    selected,
                                    theme::regular(14.0),
                                    palette.text,
                                    rect.width() - 36.0,
                                    1,
                                );
                                text.paint(
                                    ui,
                                    egui::pos2(
                                        rect.left() + 8.0,
                                        rect.center().y - text.size().y / 2.0,
                                    ),
                                    palette.text,
                                );
                                response.response.widget_info(|| {
                                    let mut info = egui::WidgetInfo::labeled(
                                        egui::WidgetType::ComboBox,
                                        ui.is_enabled(),
                                        i18n::t(Key::SettingsLanguageLabel),
                                    );
                                    info.current_text_value = Some(selected.to_owned());
                                    info
                                });
                            });
                        },
                    );

                    section(ui, app, i18n::t(Key::SettingsSectionChats));""",
    },
    {
        "file": "src/ui/widgets.rs",
        "old": """            ui.set_width((ui.available_width() - 260.0).max(120.0));
            rich_text(ui, label, theme::medium(14.0), palette.text);""",
        "new": """            ui.set_width((ui.available_width() - 260.0).max(120.0));
            let label = line(ui, label, theme::medium(14.0), palette.text, ui.available_width(), 2);
            let (rect, _) = ui.allocate_exact_size(label.size(), Sense::hover());
            if ui.is_rect_visible(rect) {
                label.paint(ui, rect.min, palette.text);
            }""",
    },
]

data["sites"].extend(sites)
DATA.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
print(f"rows: {len(data['strings'])}, sites: {len(data['sites'])}")
