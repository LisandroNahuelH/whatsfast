//! es strings.

use super::Key;

pub(super) fn text(key: Key) -> &'static str {
    match key {
        Key::SettingsThemeDark => "Oscuro",
        Key::SettingsThemeLight => "Claro",
        Key::SettingsThemeSystem => "Seguir el sistema",
        Key::SettingsHistoryOff => "Desactivado",
        Key::SettingsHistoryCurrent => "Chat actual",
        Key::SettingsHistoryRecent => "Recientes y fijados",
        Key::SettingsHistoryOffHint => "No descarga mensajes anteriores en segundo plano.",
        Key::SettingsHistoryCurrentHint => {
            "Descarga mensajes y archivos anteriores solo del chat abierto."
        }
        Key::SettingsHistoryRecentHint => {
            "Descarga historial anterior de los chats fijados y de los diez más recientes."
        }
        Key::SettingsWallpaperBlack => "Negro",
        Key::SettingsWallpaperGray => "Gris",
        Key::SettingsWallpaperGreen => "Verde",
        Key::SettingsWallpaperRed => "Rojo",
        Key::SettingsWallpaperWhite => "Blanco",
        Key::SettingsWallpaperAuto => "Automático",
        Key::SettingsSectionLanguage => "Idioma",
        Key::SettingsLanguageSystem => "Idioma del sistema",
        Key::SettingsLanguageEnglish => "English",
        Key::SettingsLanguageSpanish => "Español",
        Key::SettingsLanguageHint => {
            "Se aplica a toda la interfaz. Los menús de la bandeja del sistema cambian al reiniciar."
        }
        Key::DateWeekdayMonday => "lunes",
        Key::DateWeekdayTuesday => "martes",
        Key::DateWeekdayWednesday => "miércoles",
        Key::DateWeekdayThursday => "jueves",
        Key::DateWeekdayFriday => "viernes",
        Key::DateWeekdaySaturday => "sábado",
        Key::DateWeekdaySunday => "domingo",
        Key::DateWeekdayAbbrMonday => "lun",
        Key::DateWeekdayAbbrTuesday => "mar",
        Key::DateWeekdayAbbrWednesday => "mié",
        Key::DateWeekdayAbbrThursday => "jue",
        Key::DateWeekdayAbbrFriday => "vie",
        Key::DateWeekdayAbbrSaturday => "sáb",
        Key::DateWeekdayAbbrSunday => "dom",
        Key::DateMonthJanuary => "enero",
        Key::DateMonthFebruary => "febrero",
        Key::DateMonthMarch => "marzo",
        Key::DateMonthApril => "abril",
        Key::DateMonthMay => "mayo",
        Key::DateMonthJune => "junio",
        Key::DateMonthJuly => "julio",
        Key::DateMonthAugust => "agosto",
        Key::DateMonthSeptember => "septiembre",
        Key::DateMonthOctober => "octubre",
        Key::DateMonthNovember => "noviembre",
        Key::DateMonthDecember => "diciembre",
        Key::DateMonthAbbrJanuary => "ene",
        Key::DateMonthAbbrFebruary => "feb",
        Key::DateMonthAbbrMarch => "mar",
        Key::DateMonthAbbrApril => "abr",
        Key::DateMonthAbbrMay => "may",
        Key::DateMonthAbbrJune => "jun",
        Key::DateMonthAbbrJuly => "jul",
        Key::DateMonthAbbrAugust => "ago",
        Key::DateMonthAbbrSeptember => "sep",
        Key::DateMonthAbbrOctober => "oct",
        Key::DateMonthAbbrNovember => "nov",
        Key::DateMonthAbbrDecember => "dic",
        Key::DateToday => "Hoy",
        Key::DateYesterday => "Ayer",
        Key::DateYesterdayAt => "Ayer a las {time}",
        Key::DateAt => "{moment} a las {time}",
        Key::DateShort => "{day} {month} {year}",
        Key::DateLong => "{weekday}, {day} de {month} de {year}",
        Key::DateStampShort => "{weekday} {day} {month}, {time}",
        Key::DateShortWeekday => "{weekday} {day} {month}",
        Key::MonthYear => "{month} de {year}",
        Key::CopyStamp => "{time}, {day}/{month}/{year}",
        Key::ScheduleOnce => "Una vez",
        Key::ScheduleEveryDay => "Todos los días",
        Key::ScheduleEveryWeekday => "Todos los {weekday}",
        Key::ScheduleDayOfMonth => "El día {day} de cada mes",
        Key::ScheduleNthWeekdayOfMonth => "El {ordinal} {weekday} de cada mes",
        Key::ScheduleOrdinal1 => "primer",
        Key::ScheduleOrdinal2 => "segundo",
        Key::ScheduleOrdinal3 => "tercero",
        Key::ScheduleOrdinal4 => "cuarto",
        Key::ScheduleTitle => "Programar mensaje",
        Key::ScheduleTimeLabel => "Hora",
        Key::ScheduleRepeatLabel => "Repetir",
        Key::ScheduleEarlierHour => "Una hora antes",
        Key::ScheduleLaterHour => "Una hora después",
        Key::ScheduleEarlierMinutes => "Cinco minutos antes",
        Key::ScheduleLaterMinutes => "Cinco minutos después",
        Key::ScheduleSend => "Programar",
        Key::SchedulePreviousMonth => "Mes anterior",
        Key::ScheduleNextMonth => "Mes siguiente",
        Key::ScheduleNeedMessage => "Escribe el mensaje primero; se envía a la hora que elijas.",
        Key::ScheduleFrom => "{repeat}, desde el {when}",
        Key::DialogCancel => "Cancelar",
        Key::CommonBack => "Volver (Esc)",
        Key::CommonOpen => "Abrir",
        Key::CommonQuit => "Salir",
        Key::CommonShortcuts => "Atajos",
        Key::SettingsTitle => "Ajustes",
        Key::SettingsSectionAppearance => "Apariencia",
        Key::SettingsThemeLabel => "Tema",
        Key::SettingsThemeOmarchyHint => "Seguir el sistema usa tus colores de Omarchy.",
        Key::SettingsThemeSystemHint => {
            "Seguir el sistema usa el modo claro u oscuro de tu escritorio."
        }
        Key::SettingsOpenThemesFolder => "Abrir carpeta de temas",
        Key::SettingsWallpaperLabel => "Fondo de chat",
        Key::SettingsWallpaperHint => {
            "Automático elige un fondo según los colores del tema. Puedes forzar de Negro 1 a Blanco 3. Haz clic derecho en el chat abierto y elige Siguiente fondo para recorrer los tres fondos de esa familia."
        }
        Key::SettingsZoomLabel => "Zoom",
        Key::SettingsZoomHint => "También puedes usar Ctrl+más y Ctrl+menos.",
        Key::SettingsZoomLarger => "Más grande",
        Key::SettingsZoomSmaller => "Más pequeño",
        Key::SettingsSectionChats => "Chats",
        Key::SettingsEnterSends => "Enter envía",
        Key::SettingsEnterSendsHint => {
            "Si está desactivado, Enter añade una línea y Ctrl+Enter envía."
        }
        Key::SettingsReceiptsOffNote => {
            "Las confirmaciones de lectura están desactivadas en tu cuenta de WhatsApp (Ajustes, Privacidad). Los chats directos no las envían. Con este interruptor activado, los grupos sí. El estado de lectura se sincroniza entre tus dispositivos igual."
        }
        Key::SettingsReceiptsOnNote => {
            "Deja que los demás vean cuándo lees mensajes o reproduces mensajes de voz en esta copia. El ajuste de tu cuenta de WhatsApp en Privacidad sigue aplicando. El estado de lectura se sincroniza entre tus dispositivos igual."
        }
        Key::SettingsSendReceipts => "Enviar confirmaciones de lectura",
        Key::SettingsSendTyping => "Mostrar cuando escribes",
        Key::SettingsAutoDownload => "Descargar adjuntos automáticamente",
        Key::SettingsAutoDownloadHint => {
            "Descarga fotos, videos, mensajes de voz y documentos de hasta 64 MB cuando entran en pantalla. Si está desactivado, haz clic en un archivo para descargarlo."
        }
        Key::SettingsHistoryLabel => "Descargar historial anterior en segundo plano",
        Key::SettingsHistoryHint => {
            "Descarga despacio los mensajes anteriores y sus archivos (hasta 64 MB) para que desplazarte hacia arriba no choque con el límite de WhatsApp. Los archivos que fallan se vuelven a pedir tras una espera larga, hasta 30 días. Recientes y fijados abarca cada chat fijado más los diez chats activos más recientes que no estén fijados."
        }
        Key::SettingsShowSenderPictures => "Mostrar la foto del remitente en todos los chats",
        Key::SettingsShowSenderPicturesHint => "WhatsApp solo las muestra en grupos.",
        Key::SettingsNamesFromContacts => "Nombres de tu agenda",
        Key::SettingsNamesFromContactsHint => {
            "Prefiere los nombres guardados en tu agenda. Si está desactivado, prefiere los nombres públicos del perfil de WhatsApp. Se aplica en toda la app."
        }
        Key::SettingsSaveContacts => "Guardar contactos en la agenda del teléfono",
        Key::SettingsSaveContactsHint => {
            "También añade a la agenda del teléfono los contactos guardados aquí. Si está desactivado, quedan como contactos de WhatsApp. Los nombres se sincronizan entre dispositivos igual."
        }
        Key::SettingsForwardInOrder => "Reenviar mensajes en orden",
        Key::SettingsForwardInOrderHint => {
            "Envía un reenvío múltiple de a un mensaje, y empieza cada uno cuando el anterior muestra su primer tilde. Así el texto, las fotos y los videos mezclados llegan en su orden original. Si está desactivado, se envían juntos y pueden llegar desordenados."
        }
        Key::SettingsShowPollButton => "Mostrar el botón Crear encuesta",
        Key::SettingsShowPollButtonHint => {
            "Añade el botón Crear encuesta junto al cuadro de mensaje. Si está desactivado, el botón se oculta y las encuestas que ya están en un chat siguen funcionando."
        }
        Key::SettingsShowShortcutHints => "Mostrar sugerencias de atajos",
        Key::SettingsSectionDownloads => "Descargas",
        Key::SettingsStorageHint => {
            "Peso de las fotos, videos, stickers y GIF que ya están en este equipo. El recuento de mensajes incluye el texto."
        }
        Key::SettingsStorageImages => "Imágenes",
        Key::SettingsStorageVideos => "Vídeos",
        Key::SettingsStorageStickersGifs => "Stickers y GIF",
        Key::SettingsStorageOther => "Otros",
        Key::SettingsStorageMessagesOne => "1 mensaje",
        Key::SettingsStorageMessagesMany => "{count} mensajes",
        Key::SettingsSectionPrivacy => "Privacidad",
        Key::SettingsPrivacyLoadFailed => {
            "No se pudo cargar la privacidad. Vuelve a cargarse cuando WhatsFast se reconecte."
        }
        Key::SettingsSectionWindow => "Ventana",
        Key::SettingsKeepRunning => "Seguir activo al cerrar la ventana",
        Key::SettingsKeepRunningHint => {
            "Mantiene WhatsFast vinculado en la bandeja del sistema. Sal de la bandeja o con Ctrl+Q."
        }
        Key::SettingsNotify => "Notificar mensajes nuevos",
        Key::SettingsNotifyHint => {
            "Muestra notificaciones del escritorio cuando la ventana está oculta, en segundo plano o mostrando otro chat. Los chats silenciados no notifican."
        }
        Key::SettingsHideSidebar => "Ocultar la barra lateral por completo",
        Key::SettingsHideSidebarHint => {
            "Activado: ocultar la barra lateral (Ctrl+B) no deja nada. Desactivado: se reduce a las fotos de los chats, así la búsqueda, los chats archivados y los ajustes quedan a un clic."
        }
        Key::SettingsGiphyKey => "Clave de API de GIPHY",
        Key::SettingsGiphyKeyHintBuiltIn => {
            "Se usa para buscar GIF. Esta compilación incluye una clave. Ingresa una clave de developers.giphy.com para reemplazarla."
        }
        Key::SettingsGiphyKeyHintRequired => {
            "Necesaria para buscar GIF. Consigue una clave gratis en developers.giphy.com."
        }
        Key::SettingsSectionAccount => "Cuenta",
        Key::SettingsLinkedDevice => "Dispositivo vinculado",
        Key::SettingsUnlink => "Desvincular este equipo",
        Key::SettingsSectionFiles => "Archivos",
        Key::SettingsMessageArchive => "Archivo de mensajes",
        Key::SettingsOpenFolder => "Abrir carpeta",
        Key::SettingsDownloadedAttachments => "Adjuntos descargados",
        Key::SettingsAskWhereToSave => "Preguntar dónde guardar cada archivo",
        Key::SettingsAskWhereToSaveHint => {
            "Abre el diálogo de guardado del sistema para cada adjunto que guardas desde la barra de selección, así eliges la carpeta y el nombre del archivo. Si está desactivado, los archivos van a tu carpeta de Descargas."
        }
        Key::SettingsLogOfRun => "Registro de esta ejecución",
        Key::SettingsSectionAbout => "Acerca de",
        Key::SettingsAboutLine => {
            "Un cliente de WhatsApp nativo hecho con Rust, egui y whatsapp-rust."
        }
        Key::SettingsCheckUpdates => "Buscar actualizaciones",
        Key::SettingsCheckUpdatesHint => {
            "Consulta a GitHub una vez al día si hay una versión más nueva de WhatsFast. La solicitud identifica solo a WhatsFast y su versión."
        }
        Key::SettingsDownloadUpdates => "Descargar actualizaciones automáticamente",
        Key::SettingsDownloadUpdatesHint => {
            "Descarga y verifica versiones nuevas en segundo plano. Un aviso ofrece Actualizar; con un clic se instala y se reinicia. Si ignoras el aviso, la próxima vez que abras WhatsFast se instala el archivo verificado. Los paquetes nativos y Flatpak se actualizan con su gestor de paquetes."
        }
        Key::SettingsLanguageLabel => "Idioma de la interfaz",
        Key::ShortcutSearchChats => "Buscar chats",
        Key::ShortcutSearchMessages => "Buscar mensajes en el chat abierto",
        Key::ShortcutFocusComposer => "Ir al cuadro de mensaje",
        Key::ShortcutPreviousNextChat => "Chat anterior / siguiente",
        Key::ShortcutSend => "Enviar (Shift+Enter para una línea nueva)",
        Key::ShortcutDismiss => "Cancelar la acción actual, salir de la búsqueda o cerrar el chat",
        Key::ShortcutPhotoViewer => "Visor de fotos",
        Key::ShortcutPhotoViewerKeys => {
            "La rueda acerca, el arrastre mueve, ← / → foto anterior o siguiente"
        }
        Key::ShortcutPaste => "Pegar texto o enviar una imagen del portapapeles",
        Key::ShortcutChatList => "Mostrar u ocultar la lista de chats",
        Key::ShortcutNewest => "Ir al mensaje más reciente",
        Key::ShortcutZoom => "Acercar / alejar",
        Key::ShortcutResetZoom => "Restablecer zoom",
        Key::ShortcutThisList => "Esta lista",
        Key::ShortcutCloseWindow => "Cerrar la ventana (WhatsFast queda en la bandeja)",
        Key::LoginTagline => "Un cliente de WhatsApp nativo.",
        Key::LoginConnecting => "Conectando con WhatsApp…",
        Key::LoginLinkedWaiting => "Vinculado. Esperando tus chats…",
        Key::LoginUnlinkedRequestingCode => {
            "Este equipo se desvinculó de tu teléfono. Se está pidiendo un código nuevo."
        }
        Key::LoginRequestingNewCode => "Pidiendo un código nuevo…",
        Key::LoginTryAgain => "Volver a intentar",
        Key::LoginRequestingCodeFor => "Pidiendo un código para +{phone}…",
        Key::LoginWaitingForCode => "Esperando un código de WhatsApp…",
        Key::LoginUnofficialWarning => {
            "Cliente no oficial. Usarlo puede ir contra los términos de servicio de WhatsApp."
        }
        Key::LoginLinkThisComputer => "Vincular este equipo",
        Key::LoginOpenWhatsApp => "Abre WhatsApp en tu teléfono",
        Key::LoginTapMenu => "Toca Menú o Ajustes y luego Dispositivos vinculados",
        Key::LoginTapLinkDevice => {
            "Toca Vincular un dispositivo y apunta el teléfono a este código"
        }
        Key::LoginLinkWithPhone => "Vincular con un número de teléfono",
        Key::LoginEnterCode => "Escribe este código en tu teléfono",
        Key::LoginForPhone => "para +{phone}",
        Key::LoginTapLinkPhone => {
            "Toca Vincular un dispositivo y luego Vincular con un número de teléfono"
        }
        Key::LoginCopyCode => "Copiar código",
        Key::DropToSendTo => "Suelta para enviar a {name}",
        Key::HistoryLoadingPercent => "Cargando el historial… {percent}%",
        Key::HistoryLoading => "Cargando el historial…",
        Key::OfflineReconnecting => "Sin conexión ({reason}). Reconectando…",
        Key::NotLinkedPhone => "Sin vincular a un teléfono",
        Key::CommonRetry => "Reintentar",
        Key::StatusShowChatList => "Mostrar la lista de chats (Ctrl+B)",
        Key::PollCreateTitle => "Crear encuesta",
        Key::CommonClose => "Cerrar",
        Key::PollQuestion => "Pregunta",
        Key::PollAskQuestion => "Escribe una pregunta",
        Key::PollAnswers => "Respuestas",
        Key::PollAnswerHint => "Respuesta {number}",
        Key::PollRemoveAnswer => "Quitar respuesta",
        Key::PollAddAnswer => "Añadir respuesta",
        Key::PollAllowMultiple => "Permitir varias respuestas",
        Key::PollSending => "Enviando…",
        Key::PollSend => "Enviar encuesta",
        Key::PollSelectOne => "Elige una respuesta",
        Key::PollSelectMany => "Elige las respuestas",
        Key::PollSendingVote => "Enviando voto…",
        Key::PollWaitingPhone => "Esperando a tu teléfono · pueden faltar votos anteriores",
        Key::PollLoadingVotes => "Cargando votos anteriores de tu teléfono…",
        Key::PollVotesNotLoaded => "Todavía no se cargaron los votos anteriores",
        Key::PollVotingKeyMissing => "Llave de votación no disponible · usa tu teléfono",
        Key::PollReconnectToVote => "Vuelve a conectar para votar",
        Key::UpdateNewVersion => "Hay una versión nueva disponible",
        Key::UpdateDownloadFromGitHub => "Descargar desde GitHub",
        Key::UpdateUpdate => "Actualizar",
        Key::ViewerCouldNotDisplay => "No se pudo mostrar esta imagen.",
        Key::ViewerClose => "Cerrar foto",
        Key::ViewerPrevious => "Foto anterior",
        Key::ViewerNext => "Foto siguiente",
        Key::PaneSearchTitle => "Buscar mensajes",
        Key::PaneFilterByDate => "Filtrar por fecha",
        Key::PaneSearchHint => "Buscar",
        Key::PaneSearchWith => "Buscar mensajes con {title}",
        Key::PaneNoMessages => "No se encontraron mensajes",
        Key::PaneTryAnother => "Prueba con otra palabra o elige otro día.",
        Key::PickerTabEmoji => "Emoji",
        Key::PickerTabGif => "GIF",
        Key::PickerTabStickers => "Stickers",
        Key::PickerGroupSmileys => "Caritas y emociones",
        Key::PickerGroupPeople => "Personas y cuerpos",
        Key::PickerGroupAnimals => "Animales y naturaleza",
        Key::PickerGroupFood => "Comida y bebida",
        Key::PickerGroupTravel => "Viajes y lugares",
        Key::PickerGroupActivities => "Actividades",
        Key::PickerGroupObjects => "Objetos",
        Key::PickerGroupSymbols => "Símbolos",
        Key::PickerGroupFlags => "Banderas",
        Key::PickerRecent => "Recientes",
        Key::PickerFrequentlyUsed => "Usados frecuentemente",
        Key::PickerNothingMatches => "Sin coincidencias",
        Key::PickerSearchEmoji => "Buscar emoji",
        Key::PickerSearchGifs => "Buscar GIF en GIPHY",
        Key::PickerSearching => "Buscando…",
        Key::PickerGifEmpty => "Busca un GIF o mira los resultados del momento.",
        Key::PickerGiphyKeyRejected => {
            "GIPHY rechazó esta clave de API. Crea una clave gratis en developers.giphy.com y pégala aquí. Se guarda en tus ajustes."
        }
        Key::PickerGiphyKeyNeeded => {
            "La búsqueda de GIF necesita una clave de API de GIPHY. Crea una clave gratis en developers.giphy.com y pégala aquí. Se guarda en tus ajustes."
        }
        Key::PickerStickersLoading => "Cargando tus stickers…",
        Key::PickerStickersEmpty => {
            "Los stickers recientes aparecen aquí. Haz clic derecho en uno para guardarlo. Para importar un paquete, pega un enlace de signal.art o abre un archivo .wastickers."
        }
        Key::PickerStickersSaved => "Guardados",
        Key::PickerStickerRemovePack => "Quitar este paquete",
        Key::PickerFindPacks => "Buscar paquetes",
        Key::CommonOpenFile => "Abrir archivo",
        Key::PickerPasteLink => "Pega un enlace de signal.art",
        Key::PickerBrowseStickers => "Explorar signalstickers.org",
        Key::PickerImportingPack => "Importando el paquete…",
        Key::PickerRemoveFromSaved => "Quitar de guardados",
        Key::PickerSaveSticker => "Guardar sticker",
        Key::DialogForwardOne => "Reenviar mensaje",
        Key::DialogForwardMany => "Reenviar {count} mensajes",
        Key::DialogNoWritableChats => "No se encontraron chats donde escribir",
        Key::DialogEditList => "Editar lista",
        Key::DialogNewList => "Nueva lista",
        Key::DialogListName => "Nombre de la lista",
        Key::DialogSave => "Guardar",
        Key::DialogCreate => "Crear",
        Key::DialogShortcutsTitle => "Atajos de teclado",
        Key::DialogAboutLine => {
            "Un cliente de WhatsApp nativo escrito en Rust con egui. Se conecta a través de whatsapp-rust. Los mensajes están cifrados de extremo a extremo en este dispositivo."
        }
        Key::DialogUnofficialWarning => {
            "Este es un cliente no oficial. Usarlo puede ir contra los términos de servicio de WhatsApp y podría suspender una cuenta."
        }
        Key::DialogSourceCode => "Código fuente",
        Key::DialogVersion => "Versión {version}",
        Key::DialogUnlinkTitle => "¿Desvincular este equipo?",
        Key::DialogUnlinkBody => {
            "Esto quita el dispositivo de WhatsApp y elimina los chats guardados aquí. Puedes vincular de nuevo con un código nuevo."
        }
        Key::DialogUnlinkButton => "Desvincular",
        Key::DialogLeaveChannelTitle => "¿Salir de este canal?",
        Key::DialogLeaveGroupTitle => "¿Salir de este grupo?",
        Key::DialogLeaveBody => {
            "No recibirás mensajes nuevos. El historial local queda en este equipo."
        }
        Key::DialogLeaveChannelAction => "Salir del canal",
        Key::DialogLeaveGroupAction => "Salir del grupo",
        Key::DialogLeaveChannelArchive => "Salir del canal y archivar",
        Key::DialogLeaveGroupArchive => "Salir del grupo y archivar",
        Key::DialogLinkPhoneTitle => "Vincular con un número de teléfono",
        Key::DialogPhoneInstructions => {
            "Escribe el número de teléfono de WhatsApp con su código de país. No incluyas el signo más ni un cero inicial. Recibirás un código para escribir en el teléfono."
        }
        Key::DialogGetCode => "Obtener un código",
        Key::DialogNewContactTitle => "Contacto nuevo",
        Key::DialogNewContactBody => {
            "Escribe un número de teléfono con su código de país, sin el signo más ni un cero inicial. Añade un nombre para guardar el contacto, o déjalo en blanco para abrir el chat. WhatsApp usa el nombre de pila como nombre visible."
        }
        Key::DialogFirstName => "Nombre",
        Key::DialogSurname => "Apellido",
        Key::DialogCheckingNumber => "Verificando el número…",
        Key::DialogSaveContact => "Guardar contacto",
        Key::DialogMessage => "Mensaje",
        Key::KindGroup => "Grupo",
        Key::KindChannel => "Canal",
        Key::KindContact => "Contacto",
        Key::DialogSaveName => "Guardar nombre (Enter)",
        Key::DialogMembersOne => "1 miembro",
        Key::DialogMembersMany => "{count} miembros",
        Key::DialogMembersTitle => "Miembros ({count})",
        Key::DialogLastSeen => "Última vez: {when}",
        Key::DialogMuted => "Silenciado",
        Key::DialogMutedUntil => "Silenciado hasta {when}",
        Key::DialogRename => "Cambiar nombre",
        Key::DialogAddToContacts => "Añadir a contactos",
        Key::DialogCopyNumber => "Copiar número",
        Key::DialogUnmute => "Reactivar notificaciones",
        Key::DialogMute => "Silenciar notificaciones",
        Key::DialogUnfavorite => "Quitar de favoritos",
        Key::DialogFavorite => "Añadir a favoritos",
        Key::DialogUnpin => "Dejar de fijar",
        Key::DialogPin => "Fijar",
        Key::ChatArchive => "Archivar",
        Key::ChatUnarchive => "Desarchivar",
        Key::TrayShowHide => "Mostrar u ocultar WhatsFast",
        Key::TrayThreadStopped => "La bandeja dejó de responder",
        Key::DisplayYou => "Tú",
        Key::DisplayYouPrefix => "Tú: ",
        Key::TypingOne => "{who} está escribiendo…",
        Key::TypingMany => "{others} y {last} están escribiendo…",
        Key::ChatListHideCtrl => "Ocultar la lista de chats (Ctrl+B)",
        Key::ChatListHideCmd => "Ocultar la lista de chats (⌘B)",
        Key::ChatListNewContactCmd => "Contacto nuevo (⌘N)",
        Key::ChatListSearchCtrl => "Buscar (Ctrl+F)",
        Key::ChatListArchived => "Chats archivados",
        Key::ChatListBack => "Volver a los chats",
        Key::ChatListScheduled => "Programados",
        Key::ChatListArchivedShort => "Archivados",
        Key::ChatListStarred => "Destacados",
        Key::ChatListScheduledMessages => "Mensajes programados",
        Key::ChatListStarredMessages => "Mensajes destacados",
        Key::ChatListSettingsCtrl => "Ajustes (Ctrl+,)",
        Key::ChatListFilterAll => "Todos",
        Key::ChatListFilterUnread => "No leídos",
        Key::ChatListFilterFavorites => "Favoritos",
        Key::ChatListFilterGroups => "Grupos",
        Key::CommonEdit => "Editar",
        Key::CommonDelete => "Eliminar",
        Key::ChatListNoScheduled => "No hay mensajes programados",
        Key::ChatListNoScheduledHint => {
            "Escribe un mensaje y elige una hora con el reloj junto al botón Enviar."
        }
        Key::ChatListNoStarred => "No hay mensajes destacados",
        Key::ChatListNoStarredHint => {
            "Elige mensajes en un chat y pulsa Destacar para guardarlos aquí."
        }
        Key::ChatListNothingArchived => "No hay nada archivado",
        Key::ChatListArchivedHint => "Los chats archivados aparecen aquí.",
        Key::ChatListLoading => "Cargando tus chats",
        Key::ChatListLoadingHint => "Recibiendo el historial de tu teléfono.",
        Key::ChatListNoUnread => "No hay chats sin leer",
        Key::ChatListNoUnreadHint => "Los chats con mensajes sin leer aparecen aquí.",
        Key::ChatListNoFavorites => "Aún no hay favoritos",
        Key::ChatListNoFavoritesHint => "Haz clic derecho en un chat y añádelo a favoritos.",
        Key::ChatListNoGroups => "No hay grupos",
        Key::ChatListNoGroupsHint => "Los chats de grupo aparecen aquí.",
        Key::ChatListNothingInList => "No hay nada en esta lista",
        Key::ChatListNothingInListHint => {
            "Haz clic derecho en este filtro y elige Editar para añadir chats."
        }
        Key::ChatListNoChats => "Aún no hay chats",
        Key::ChatListNoChatsHint => {
            "Los chats nuevos aparecen aquí. Puedes iniciar uno desde tu teléfono."
        }
        Key::ChatListNoResults => "Sin resultados",
        Key::ChatListNoResultsHint => "Prueba con otro nombre, número o texto.",
        Key::ChatSearchSectionChats => "Chats",
        Key::ChatSearchSectionMessages => "Mensajes",
        Key::ChatSearchSectionContacts => "Contactos",
        Key::ChatMenuMarkRead => "Marcar como leído",
        Key::ChatMenuMarkUnread => "Marcar como no leído",
        Key::ChatMenuRemoveFavorite => "Quitar de favoritos",
        Key::ChatMenuAddFavorite => "Añadir a favoritos",
        Key::ChatPinToTop => "Fijar chat",
        Key::ChatMuteFor8Hours => "Silenciar durante 8 horas",
        Key::ChatMuteForWeek => "Silenciar durante una semana",
        Key::ChatMuteForever => "Silenciar siempre",
        Key::ChatInfo => "Info",
        Key::TypingShort => "escribiendo…",
        Key::ChatEmptyLoading => "Tus chats aparecen a la izquierda mientras cargan.",
        Key::ChatEmptySelect => "Elige un chat a la izquierda.",
        Key::ChatEmptyHints => "Ctrl+K para buscar · Ctrl+/ para los atajos",
        Key::ChatMore => "Más",
        Key::ChatClose => "Cerrar chat",
        Key::ChatSearchMessages => "Buscar mensajes (Ctrl+G)",
        Key::ChatSelectMessages => "Seleccionar mensajes",
        Key::ChatSelectAll => "Seleccionar todo",
        Key::ChatNextWallpaper => "Siguiente fondo",
        Key::ChatCancelSelection => "Cancelar selección",
        Key::ChatSelectedOne => "1 seleccionado",
        Key::ChatSelectedMany => "{count} seleccionados",
        Key::ChatUnstar => "Dejar de destacar",
        Key::ChatStar => "Destacar",
        Key::CommonDownload => "Descargar",
        Key::CommonForward => "Reenviar",
        Key::CommonSend => "Enviar",
        Key::CommonDiscard => "Descartar",
        Key::ChatLeftChannel => "Saliste de este canal",
        Key::ChatLeftGroup => "Saliste de este grupo",
        Key::ChatOnlyAdmins => "Solo los administradores pueden enviar mensajes",
        Key::ChatSendFiles => "Enviar archivos (o arrástralos a la ventana)",
        Key::ChatAttachHint => "Emoji, GIF y stickers",
        Key::ChatTypeMessage => "Escribe un mensaje",
        Key::ChatAddCaption => "Añade un comentario",
        Key::ChatRecordVoice => "Grabar un mensaje de voz",
        Key::ChatScheduleMessage => "Programar este mensaje",
        Key::ChatHintsEnter => {
            "Enter envía · Shift+Enter para una línea nueva · *negrita* _cursiva_ ~tachado~ · Ctrl+V pega una imagen"
        }
        Key::ChatHintsCtrlEnter => {
            "Ctrl+Enter envía · *negrita* _cursiva_ ~tachado~ · Ctrl+V pega una imagen"
        }
        Key::ChatHideHints => "Ocultar sugerencias de atajos (se restauran en Ajustes)",
        Key::ChatAllShortcuts => "Todos los atajos ({keys})",
        Key::ChatEditingMessage => "Editando mensaje",
        Key::ChatStopEditing => "Detener la edición (Esc)",
        Key::ChatReplyingTo => "Respondiendo a {who}",
        Key::ChatCancelReply => "Cancelar respuesta (Esc)",
        Key::ChatNewest => "Mensaje más reciente",
        Key::ChatLoadingOlder => "Cargando mensajes anteriores de tu teléfono…",
        Key::ChatLoadingMessages => "Cargando mensajes de tu teléfono…",
        Key::ChatNoMessages => "Aún no hay mensajes aquí",
        Key::MarkerPhoto => "[foto]",
        Key::MarkerGif => "[GIF]",
        Key::MarkerVideo => "[video]",
        Key::MarkerVoiceWith => "[nota de voz, {duration}]",
        Key::MarkerVoice => "[nota de voz]",
        Key::MarkerAudio => "[audio]",
        Key::MarkerDocument => "[documento: {name}]",
        Key::MarkerSticker => "[sticker]",
        Key::MarkerLocation => "[ubicación]",
        Key::MarkerContact => "[contacto: {name}]",
        Key::MarkerPoll => "[encuesta: {question}]",
        Key::MarkerReplying => "(en respuesta a {name}: \"{text}\")",
        Key::ChatForwarded => "Reenviado",
        Key::ChatDeleteEveryone => "Eliminar para todos",
        Key::ChatDeleteMe => "Eliminar para mí",
        Key::ChatShowInFolder => "Mostrar en la carpeta",
        Key::ChatRemoveReaction => "Quitar tu reacción",
        Key::ChatReactWithEmoji => "Reaccionar con cualquier emoji",
        Key::ChatReply => "Responder",
        Key::ChatCopyText => "Copiar texto",
        Key::ChatSent => "Enviado {when}",
        Key::ChatDeliveredAt => "Entregado {when}",
        Key::ChatDelivered => "Entregado",
        Key::ChatPlayed => "Reproducido",
        Key::ChatRead => "Leído",
        Key::ChatWhatAt => "{what} {when}",
        Key::ChatPagesOne => "1 página",
        Key::ChatPagesMany => "{count} páginas",
        Key::KindLocation => "Ubicación",
        Key::KindGif => "GIF",
        Key::KindVideo => "Video",
        Key::ChatOpenInMap => "Abrir en el mapa",
        Key::ChatMessageDeleted => "Se eliminó este mensaje",
        Key::ChatUnsupported => "No compatible: {what}",
        Key::ChatPictureFailed => "No se pudo mostrar esta imagen. Haz clic para abrirla.",
        Key::ChatPause => "Pausar",
        Key::ChatPlay => "Reproducir",
        Key::ChatPreparingSpeed => "Preparando la velocidad de reproducción",
        Key::ChatPlaybackSpeed => "Velocidad de reproducción",
        Key::DisplayUnknown => "Desconocido",
        Key::ToastHistoryLoaded => "Historial cargado",
        Key::ToastBackOnline => "De vuelta en línea",
        Key::ToastUnlinked => "Este equipo se desvinculó de tu teléfono",
        Key::ToastOpenChatFirst => "Abre un chat primero",
        Key::ToastSendingFilesOne => "Enviando 1 archivo…",
        Key::ToastSendingFilesMany => "Enviando {count} archivos…",
        Key::ToastCouldNotOpen => "No se pudo abrir {what}: {error}",
        Key::ToastCopied => "Copiado",
        Key::ToastStickerSaved => "Sticker guardado",
        Key::ToastSendingGif => "Enviando GIF…",
        Key::ToastCouldNotRecord => "No se pudo grabar: {error}",
        Key::DialogDigitsOnly => {
            "Escribe el número de teléfono con su código de país, solo con dígitos"
        }
        Key::PollErrorQuestion => "Escribe una pregunta de hasta 255 caracteres.",
        Key::PollErrorAnswers => "Añade de 2 a 12 respuestas, de 1 a 100 caracteres cada una.",
        Key::PollErrorDuplicates => "Cada respuesta debe ser distinta.",
        Key::KindPhoto => "Foto",
        Key::KindVoice => "Mensaje de voz",
        Key::KindAudio => "Audio",
        Key::KindSticker => "Sticker",
        Key::KindDocumentWith => "Documento: {name}",
        Key::KindLocationWith => "Ubicación: {name}",
        Key::KindContactWith => "Contacto: {name}",
        Key::KindPollWith => "Encuesta: {question}",
        Key::ChatUnsupportedMessage => "Mensaje no compatible ({what})",
        Key::ChatRetryFile => {
            "Seguimos intentando conseguir este archivo automáticamente. Haz clic para reintentar."
        }
        Key::ChatFileGone => "Ya no está disponible en los servidores de WhatsApp",
        Key::PrivacyLastSeen => "Última vez",
        Key::PrivacyOnline => "En línea",
        Key::PrivacyProfilePhoto => "Foto del perfil",
        Key::PrivacyAbout => "Información",
        Key::PrivacyGroupsAdd => "Quién puede añadirme a grupos",
        Key::PrivacyReadReceipts => "Confirmaciones de lectura",
        Key::PrivacyCalls => "Quién puede llamarme",
        Key::PrivacyMessages => "Quién puede enviarme mensajes",
        Key::PrivacyLastSeenHint => "Cuándo pueden ver que usaste WhatsApp por última vez.",
        Key::PrivacyOnlineHint => "Cuándo pueden ver que estás en línea.",
        Key::PrivacyProfilePhotoHint => "Quién puede ver tu foto del perfil.",
        Key::PrivacyAboutHint => "Quién puede ver tu información. No es la pestaña Estado.",
        Key::PrivacyGroupsHint => "Quién puede añadirte a un grupo.",
        Key::PrivacyReceiptsHint => {
            "Todos o nadie en tu cuenta de WhatsApp. El interruptor de Chats sigue aplicando a esta copia."
        }
        Key::PrivacyCallsHint => "Quién puede llamarte por WhatsApp.",
        Key::PrivacyMessagesHint => "Quién puede iniciar un chat contigo.",
        Key::PrivacyHideLastSeen => "Ocultar Última vez para",
        Key::PrivacyHidePhoto => "Ocultar foto del perfil para",
        Key::PrivacyHideAbout => "Ocultar Información para",
        Key::PrivacyWhoCannotAdd => "Quién no puede añadirte a grupos",
        Key::PrivacyExcept => "Excepto",
        Key::PrivacyEveryone => "Todos",
        Key::PrivacyMyContacts => "Mis contactos",
        Key::PrivacyExceptEllipsis => "Excepto…",
        Key::PrivacyNobody => "Nadie",
        Key::PrivacySameAsLastSeen => "Igual que Última vez",
        Key::PrivacyContactsWithNumber => "Mis contactos y otras personas con mi número",
        Key::ToastNotConnected => "Sin conexión a WhatsApp",
        Key::ToastNotConnectedYet => "Todavía no hay conexión a WhatsApp",
        Key::PollErrCantSendHere => "No se pueden enviar encuestas a este chat.",
        Key::PollErrDisappearing => {
            "Aún no se pueden crear encuestas en chats con mensajes que desaparecen."
        }
        Key::PollErrRecipients => "No se pudo cargar la lista de destinatarios del grupo",
        Key::PollErrSend => "No se pudo enviar la encuesta. Inténtalo de nuevo.",
        Key::PollErrAccountChanged => "La cuenta cambió mientras se enviaba la encuesta.",
        Key::PollErrKeyNotSaved => {
            "La encuesta se envió, pero no se pudo guardar su clave de votación."
        }
        Key::PollErrNotReady => {
            "Esta encuesta todavía no está lista para votar. Vuelve a conectar e inténtalo de nuevo."
        }
        Key::PollErrVoteSend => "No se pudo enviar tu voto. Inténtalo de nuevo.",
        Key::PollErrVoteNotSaved => "Tu voto se envió, pero no se pudo guardar en este equipo.",
        Key::ChatWaitingMessage => "Esperando este mensaje. Abre WhatsApp en tu teléfono",
        Key::ToastHistoryPartFailed => "No se pudo leer parte del historial: {error}",
        Key::ToastNoOlder => {
            "Tu teléfono no envió mensajes anteriores. Comprueba que esté en línea"
        }
        Key::ToastOlderFailed => "No se pudieron pedir mensajes anteriores a tu teléfono: {error}",
        Key::ToastStarred => "Destacado",
        Key::ToastStarRemoved => "Se quitó el destacado",
        Key::ToastStarredProgress => "Destacados {done} de {total}",
        Key::ChatSendToWhatsApp => "Enviar a WhatsApp",
        Key::ToastStickerSaveFailed => "No se pudo guardar el sticker: {error}",
        Key::DialogAddPack => "Añadir un pack de stickers",
        Key::DialogStickerPacks => "Packs de stickers",
        Key::ToastContactSaveFailed => "No se pudo guardar el contacto: {error}",
        Key::ToastContactAdded => "Se añadió {name} a los contactos",
        Key::ToastNotOnWhatsApp => "{name} no está en WhatsApp",
        Key::ToastPackAdded => "Se añadió el pack de stickers \"{name}\"",
        Key::ToastPackFailed => "No se pudo añadir el pack de stickers: {error}",
        Key::ToastPrivacyFailed => "No se pudieron actualizar los ajustes de privacidad.",
        Key::ToastLinkPhoneFailed => "No se pudo vincular con el número de teléfono: {error}",
        Key::ToastMessageNotSent => "No se envió el mensaje: {error}",
        Key::ToastSavedTo => "Guardado en {path}",
        Key::ToastDownloadNotSaved => "La descarga falló, así que no se guardó",
        Key::ToastLeaveChannelFailed => "No se pudo salir del canal.",
        Key::ToastLeaveGroupFailed => "No se pudo salir del grupo.",
        Key::ToastScheduled => "Mensaje programado",
        Key::ToastForwardBusy => "Espera a que termine el reenvío actual",
        Key::ToastForwardedProgress => "Reenviados {done} de {total}",
        Key::ToastNotStored => "Este mensaje no está en este equipo",
        Key::ToastNotOnComputer => "Este mensaje no está en este equipo",
        Key::ToastCannotForward => "Este mensaje no se puede reenviar",
        Key::ToastForwardNoData => {
            "Los datos del mensaje original no están disponibles para reenviar"
        }
        Key::ToastForwardUnreadable => "No se pudieron leer los datos del mensaje original",
        Key::ToastReadChatFailed => "No se pudo leer el chat: {error}",
        Key::ToastKeysMissing => "Faltan las claves de descarga del adjunto",
        Key::ToastNoFile => "Este mensaje no tiene un archivo para descargar",
        Key::ToastNoDownloads => "No hay una carpeta de Descargas en este equipo",
        Key::ToastSaveDialogOpen => "Ya hay un diálogo de guardado abierto",
        Key::DialogSaveAttachment => "Guardar adjunto",
        Key::DialogSingleFile => "un solo archivo",
        Key::ToastSaveCancelled => "Guardado cancelado",
        Key::ToastSavingProgress => "Guardando {done} de {total}",
        Key::ToastSearchFailed => "No se pudo buscar: {error}",
        Key::ToastEditFailed => "No se pudo enviar la edición: {error}",
        Key::ToastDeleteFailed => "No se pudo eliminar el mensaje para todos: {error}",
        Key::ToastFileFailed => "No se pudo enviar el archivo: {error}",
        Key::ToastClipboardInvalid => "Los datos de imagen del portapapeles no son válidos",
        Key::ToastPictureFailed => "No se pudo enviar la imagen: {error}",
        Key::ToastVoiceFailed => "No se pudo enviar el mensaje de voz: {error}",
        Key::ToastStickerSendFailed => "No se pudo enviar el sticker: {error}",
        Key::ToastGifFailed => "No se pudo enviar el GIF: {error}",
        Key::ToastEncodeFailed => "No se pudo codificar el adjunto",
        Key::ToastShuttingDown => "La aplicación se está cerrando",
        Key::ToastRecipientsSaveFailed => {
            "No se pudieron guardar los destinatarios del mensaje de grupo"
        }
        Key::ToastStartFailed => "No se pudo iniciar WhatsApp: {error}",
        Key::ToastDeviceStoreFailed => "No se pudo abrir el almacén del dispositivo: {error}",
        Key::ErrDetail => ": {message}",
        Key::ErrConnectFailed => "Falló la conexión con WhatsApp ({reason}){detail}",
        Key::ErrStreamReplaced => "Otra sesión de WhatsApp Web reemplazó a esta",
        Key::ErrTemporaryBan => "WhatsApp bloqueó esta cuenta temporalmente ({code})",
        Key::ErrClientOutdated => {
            "WhatsApp rechazó esta versión de WhatsFast. Actualiza la aplicación"
        }
        Key::KindDocument => "Documento",
        Key::KindLiveLocation => "Ubicación en vivo",
        Key::KindContactsOne => "1 contacto",
        Key::KindContactsMany => "{count} contactos",
        Key::KindPoll => "Encuesta",
        Key::KindGroupInvite => "invitación a un grupo",
        Key::KindStickerPack => "pack de stickers",
        Key::KindInteractive => "mensaje interactivo",
        Key::KindAnimatedSticker => "sticker animado",
        Key::GifErrNoKey => "La búsqueda de GIF necesita una clave de API de GIPHY.",
        Key::GifErrRequest => "Falló la solicitud a GIPHY: {error}",
        Key::GifErrKeyRejected => "GIPHY rechazó la clave de API (error {code}).",
        Key::GifErrResponse => "Respuesta inválida de GIPHY: {error}",
        Key::GifErrMessage => "GIPHY: {message}",
        Key::GifErrNoResults => "La respuesta de GIPHY no tuvo resultados",
        Key::ToastClipEmpty => "El clip está vacío",
        Key::ToastAudioDecodeFailed => "No se pudo decodificar el audio: {error}",
        Key::ToastNoSoundOutput => "Sin salida de sonido: {error}",
        Key::ToastAudioReadFailed => "No se pudo leer el audio: {error}",
        Key::ToastAudioDecodeFailed2 => "No se pudo decodificar el audio: {error}",
        Key::ToastNoAudioRecorded => "No se grabó audio",
        Key::ToastMicMissing => "No hay micrófono disponible: {error}",
        Key::ToastMicFormat => "El micrófono no tiene un formato compatible: {error}",
        Key::ToastMicOpenFailed => "No se pudo abrir el micrófono: {error}",
        Key::ToastMicEmpty => "El micrófono no grabó audio",
        Key::StickerErrMissingPackId => "Falta pack_id. Copia el enlace completo de signal.art",
        Key::StickerErrMissingPackKey => {
            "Falta pack_key o no es válido. Copia el enlace completo de signal.art"
        }
        Key::StickerErrIncompleteData => "Los datos del sticker están incompletos",
        Key::StickerErrKeyDerive => "No se pudo derivar la clave del sticker",
        Key::StickerErrKeyMismatch => "La clave no coincide con este pack",
        Key::StickerErrDecrypt => "No se pudo descifrar el pack de stickers",
        Key::StickerManifestIncomplete => "El manifiesto del sticker está incompleto",
        Key::StickerManifestNumber => "El manifiesto del sticker tiene un número inválido",
        Key::StickerManifestField => "El manifiesto del sticker tiene un campo desconocido",
        Key::StickerErrCertificate => "No se pudo leer el certificado de Signal: {error}",
        Key::StickerErrRequest => "Falló la solicitud a signal.art: {error}",
        Key::StickerErrResponse => "No se pudo leer la respuesta de signal.art: {error}",
        Key::StickerErrNoStickers => "Este pack no tiene stickers",
        Key::StickerErrOpenFile => "No se pudo abrir el archivo: {error}",
        Key::StickerErrNotArchive => "Este archivo no es un archivo de stickers: {error}",
        Key::StickerErrNoneRead => "No se pudo leer ningún sticker de este pack",
        Key::StickerErrWrite => "No se pudo escribir el pack de stickers: {error}",
        Key::StickerErrCreateFolder => "No se pudo crear la carpeta del pack: {error}",
        Key::StickerErrTooMany => "Hay demasiados packs de stickers con este nombre",
        Key::KindStickerPackTitle => "Pack de stickers",
        Key::KindStickers => "Stickers",
        Key::MenuAbout => "Acerca de WhatsFast",
        Key::MenuSettings => "Ajustes…",
        Key::MenuHide => "Ocultar WhatsFast",
        Key::MenuQuit => "Salir de WhatsFast",
        Key::MenuFile => "Archivo",
        Key::MenuNewContact => "Contacto nuevo…",
        Key::MenuCloseWindow => "Cerrar ventana",
        Key::MenuEdit => "Edición",
        Key::MenuUndo => "Deshacer",
        Key::MenuRedo => "Rehacer",
        Key::MenuCut => "Cortar",
        Key::MenuCopy => "Copiar",
        Key::MenuPaste => "Pegar",
        Key::MenuSelectAll => "Seleccionar todo",
        Key::MenuFind => "Buscar…",
        Key::MenuView => "Visualización",
        Key::MenuToggleSidebar => "Mostrar u ocultar la barra lateral",
        Key::MenuZoomIn => "Acercar",
        Key::MenuZoomOut => "Alejar",
        Key::MenuActualSize => "Tamaño real",
        Key::MenuShowWhatsFast => "Mostrar WhatsFast",
        Key::MenuHelp => "Ayuda",
        Key::MenuKeyboardShortcuts => "Atajos de teclado",
        Key::MenuWhatsFastHelp => "Ayuda de WhatsFast",
        Key::UpdFlatpak => {
            "Actualiza esta instalación desde tu centro de software o con flatpak update."
        }
        Key::UpdSnap => "Actualiza esta instalación con snap refresh.",
        Key::UpdCargo => "Actualiza esta instalación con cargo install.",
        Key::UpdNix => "Actualiza esta instalación con Nix o Homebrew.",
        Key::UpdThrough => {
            "Actualiza esta instalación con {instruction} o desde tu centro de software."
        }
        Key::UpdSystemDir => {
            "Esta instalación está en una carpeta del sistema. Usa tu gestor de paquetes o la página de descarga."
        }
        Key::UpdNoInstallDir => "La aplicación no tiene carpeta de instalación",
        Key::UpdNotPortable => {
            "Esta instalación no se identifica como descarga portátil. Usa la página de descarga para instalar una versión que se pueda actualizar."
        }
        Key::UpdMissingInstallDir => "Falta la carpeta de instalación",
        Key::UpdCannotReplaceStaged => "No se puede reemplazar la actualización preparada",
        Key::UpdCannotWriteInstallDir => "No se puede escribir en la carpeta de instalación",
        Key::UpdCannotRunTar => "No se pudo ejecutar tar para descomprimir la actualización",
        Key::UpdMissingArchiveStream => "Falta el flujo del archivo",
        Key::UpdExeInvalidSize => "El ejecutable de la actualización tiene un tamaño inválido",
        Key::UpdCannotUnpackExe => "No se pudo descomprimir el ejecutable de la actualización",
        Key::UpdDownloadCannotRun => "La aplicación descargada no puede ejecutarse en este equipo",
        Key::UpdStartupCheckFailed => "La aplicación descargada no pasó la prueba de arranque",
        Key::UpdMissingVersionOutput => "Falta la salida de versión",
        Key::UpdWrongVersion => "La aplicación descargada tiene una versión distinta",
        Key::UpdStartupCheckNoAnswer => {
            "La aplicación descargada no respondió a la prueba de arranque"
        }
        Key::UpdStagedChanged => "La actualización preparada cambió. Descárgala de nuevo.",
        Key::UpdCannotStartHelper => "No se pudo iniciar el asistente de actualización",
        Key::UpdHelperExited => "El asistente de actualización se cerró antes de estar listo",
        Key::UpdHelperNoStart => "El asistente de actualización no arrancó. Inténtalo de nuevo.",
        Key::UpdCannotWatchApp => "No se puede vigilar la aplicación en ejecución",
        Key::UpdAppDidNotClose => "La aplicación no se cerró en un minuto",
        Key::UpdCannotIdentifyApp => "No se pudo identificar la aplicación en ejecución",
        Key::UpdStagedChecksumChanged => {
            "Cambió la suma de verificación de la actualización preparada"
        }
        Key::UpdAlreadyApplied => "Esta actualización ya se aplicó",
        Key::UpdCannotBackUp => "No se pudo respaldar la aplicación actual",
        Key::UpdAppStillRunning => "La aplicación sigue en ejecución o no se puede reemplazar",
        Key::UpdCannotRestore => "No se pudo restaurar la aplicación anterior",
        Key::UpdCannotReplaceApp => "No se pudo reemplazar la aplicación",
        Key::UpdInstallerFailed => {
            "Falló el instalador. Revisa el registro del instalador de la actualización."
        }
        Key::UpdHasBackup => "Esta actualización ya tiene un respaldo",
        Key::UpdInvalidJobDir => "Carpeta de trabajo de actualización inválida",
        Key::UpdInvalidPayload => "Paquete preparado inválido",
        Key::UpdInvalidInstallDir => "Carpeta de instalación inválida",
        Key::UpdFailed => "Falló la actualización: {error}",
        Key::UpdCannotLaunchUpdated => "No se pudo iniciar la aplicación actualizada",
        Key::UpdUpdatedAppExited => "La aplicación actualizada se cerró antes de abrir su ventana",
        Key::UpdUpdatedAppNoWindow => "La aplicación actualizada no abrió su ventana en un minuto",
        Key::UpdFailedRestored => {
            "Falló la actualización; se restauró la aplicación anterior: {error}"
        }
        Key::UpdUpdatedTo => "Actualizado a {version}",
        Key::UpdCouldNotStartRestored => {
            "La actualización no pudo empezar. Se restauró la versión anterior."
        }
        Key::UpdCannotRestartPrevious => "No se pudo reiniciar la aplicación anterior",
        Key::UpdInvalidReceiptDir => "Carpeta de recibo de actualización inválida",
        Key::UpdReceiptOtherInstall => "El recibo pertenece a otra instalación",
        Key::UpdUpdatedWrongVersion => "La aplicación actualizada informa una versión distinta",
        Key::UpdNoUniqueDownload => "La publicación no tiene una descarga única para {name}",
        Key::UpdInvalidDownloadSize => "Tamaño de descarga de la actualización inválido",
        Key::UpdDuplicateChecksum => "Suma de verificación duplicada para la actualización",
        Key::UpdInvalidChecksum => "Suma de verificación de la actualización inválida",
        Key::UpdMissingChecksum => {
            "A la publicación le falta la suma de verificación de la actualización"
        }
        Key::UpdInvalidReleaseVersion => "Versión de la publicación inválida",
        Key::UpdRedirectNotAllowed => {
            "No se permite una redirección en la descarga de la actualización"
        }
        Key::UpdReleaseChanged => "La publicación cambió. Busca actualizaciones de nuevo.",
        Key::UpdUseDownloadPage => {
            "Usa la página de descarga para este sistema operativo o arquitectura"
        }
        Key::UpdNotOnReleaseHost => {
            "La descarga de la actualización no está en el servidor de la publicación"
        }
        Key::UpdAssetOtherRelease => {
            "El archivo de la actualización no pertenece a esta publicación"
        }
        Key::UpdExceedsSize => "La descarga de la actualización supera el tamaño publicado",
        Key::UpdInterrupted => "La descarga de la actualización se interrumpió",
        Key::UpdCouldNotVerifyDownload => {
            "No se pudo verificar la descarga. Intenta descargarla de nuevo."
        }
        Key::UpdMoveToApplications => {
            "Mueve la aplicación a Aplicaciones y ábrela para actualizar."
        }
        Key::UpdBundleMissingKey => "Al paquete de la aplicación le falta {key}",
        Key::UpdNotAnAppBundle => "La descarga no es un paquete de aplicación de WhatsFast",
        Key::UpdHomebrew => "Actualiza esta instalación con Homebrew.",
        Key::UpdSignatureUnverified => "No se pudo verificar la firma de la aplicación",
        Key::UpdCannotReadSigning => "No se pudo leer la identidad de firma de la aplicación",
        Key::UpdBundleWrongVersion => "El paquete de la aplicación tiene una versión distinta",
        Key::UpdSignedByOther => "La actualización está firmada por otro editor",
        Key::UpdMacosNotApproved => "macOS no aprobó esta actualización para abrirla",
        Key::UpdMissingAppBundle => "Falta el paquete de la aplicación",
        Key::UpdMissingUpdateDir => "Falta la carpeta de la actualización",
        Key::UpdCannotOpenDmg => "macOS no pudo abrir la imagen de disco descargada",
        Key::UpdDmgInvalidBundle => "La imagen de disco tiene un paquete de aplicación inválido",
        Key::UpdDmgNoBundle => "La imagen de disco no tiene el paquete de aplicación de WhatsFast",
        Key::UpdCannotCopyBundle => "No se pudo copiar el paquete de la aplicación descargada",
        Key::UpdCannotBackUpBundle => "No se pudo respaldar el paquete de la aplicación actual",
        Key::UpdCannotRestoreBundle => "No se pudo restaurar el paquete de la aplicación anterior",
        Key::UpdCannotReplaceBundle => "No se pudo reemplazar el paquete de la aplicación",
        Key::UpdCannotMoveFailed => "No se pudo apartar la actualización fallida",
        Key::ThemeErrHexColor => "{name}: se esperaba #RRGGBB o #RRGGBBAA",
        Key::ThemeErrUnknownColor => "color desconocido: {name}",
        Key::ThemeErrJsonFilename => "se esperaba un nombre de archivo JSON en la carpeta de temas",
        Key::ThemeErrRegularFile => {
            "se esperaba un archivo normal, no una carpeta ni un enlace simbólico"
        }
        Key::ThemeErrTooBig => "el tema supera el límite de 64 KiB",
        Key::ThemeErrUtf8 => "se esperaba JSON en UTF-8",
        Key::ThemeErrFolderUnreadable => {
            "No se pudo leer la carpeta de temas. Mira el registro para más detalles."
        }
        Key::ThemeErrTooManyEntries => {
            "La carpeta de temas tiene más de 512 entradas. Deja menos archivos ahí para listar los temas personalizados."
        }
        Key::ThemeErrTooManyThemes => {
            "Solo se pueden listar 128 temas personalizados. Deja menos archivos JSON en la carpeta de temas para ver el resto."
        }
        Key::ThemeErrOmarchyLoad => {
            "No se pudo cargar la paleta de Omarchy. Se mantiene el último aspecto que funcionó. Mira el registro para más detalles."
        }
        Key::ThemeErrReload => {
            "No se pudieron cargar los temas personalizados. Ejecuta whatsfast reload-themes para reintentar."
        }
        Key::ThemeLoadingLocal => "Cargando temas locales…",
        Key::ThemeErrSelectedUnavailable => {
            "El tema elegido no está disponible. Se mantiene el último aspecto que funcionó. Mira el registro para más detalles."
        }
        Key::ThemeErrOmarchyColors => "No se pudieron leer los colores actuales de Omarchy",
        Key::ThemeErrOmarchyTooBig => "El archivo de tema de Omarchy supera los 64 KiB",
        Key::ThemeErrOmarchyIncomplete => "marcador de paleta incompleto",
        Key::ThemeErrOmarchyMissingColor => "falta un color de Omarchy",
        Key::ThemeErrOmarchyMix => "mezcla de paleta no compatible",
        Key::ThemeErrRgb => "se esperaba un color RGB",
        Key::ThemeErrOmarchyPlaceholder => "marcador de paleta no compatible",
        Key::ArchiveErrBadKeyringKey => {
            "La clave del archivo en el llavero del sistema no es válida"
        }
        Key::ArchiveErrNoParentDir => "El archivo no tiene carpeta contenedora",
        Key::ArchiveErrUnlockKeyring => "Desbloquea el llavero del sistema y reinicia WhatsFast",
        Key::ArchiveErrKeyringOpen => {
            "El llavero del sistema no pudo abrir la clave del archivo de WhatsFast"
        }
        Key::ArchiveErrKeySaveMigrated => {
            "No se pudo guardar la clave migrada del archivo en el llavero del sistema"
        }
        Key::ArchiveErrKeyMissing => {
            "El archivo está cifrado pero falta su clave en el llavero del sistema. Restaura el llavero original; el archivo no cambió"
        }
        Key::ArchiveErrKeyGenerate => "No se pudo generar una clave para el archivo",
        Key::ArchiveErrKeySave => {
            "No se pudo guardar la clave del archivo en el llavero del sistema"
        }
        Key::ArchiveErrKeyVerify => "No se pudo verificar la clave guardada del archivo",
        Key::ArchiveErrKeyNotRetained => "El llavero del sistema no conservó la clave del archivo",
        Key::ArchiveErrNoCipher => "Esta compilación no admite archivos cifrados",
        Key::ArchiveErrUnlockFailed => {
            "No se pudo desbloquear el archivo con su clave del llavero del sistema"
        }
        Key::ArchiveErrClosePrograms => {
            "Cierra los otros programas que usan el archivo antes de migrarlo"
        }
        Key::ArchiveErrPathNotUtf8 => "La ruta del archivo no es UTF-8",
        Key::ArchiveErrIntegrity => "El archivo cifrado no pasó la verificación de integridad",
        Key::ArchiveErrReplaceFailed => "No se pudo reemplazar el archivo por su copia cifrada",
        Key::NotifyErrIdentity => "identidad de notificación no disponible: {error}",
    }
}
