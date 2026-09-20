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
    }
}
