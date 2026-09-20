# Glosario ES — WhatsFast (i18n)

Fecha: 2026-09-19 · Paquete: I18N-3 (`AGENTS/i18n-package-I18N-3.txt`) · Fuente: consejo I18N (asiento 4) + UI oficial de WhatsApp ES + inventario real del repo.

Reglas de estilo (ETS, español neutro internacional, sin voseo ni usted):

1. Etiquetas, ítems de menú, botones y chips: mayúscula inicial y SIN punto final. Ej.: "Fijar chat", "Salir del grupo".
2. Frases completas (hints, errores, estados vacíos): punto final, una idea por oración.
3. Acciones en menús y botones: infinitivo ("Archivar", "Eliminar chat"). Instrucciones al usuario: imperativo tuteo neutro ("Escribe un mensaje", "Elige una opción"). Prohibido: "Seleccione", "Borrar el chat", voseo ("escribí", "tenés").
4. "chat", "grupo", "canal", "contacto", "sticker", "emoji" en minúscula salvo inicio o etiqueta.
5. Sin posesivos innecesarios; "tu teléfono" solo donde no hay forma neutra (vinculación, cargas).
6. Atajos literales: Ctrl+K, Ctrl+/, Shift+Enter, ⌘Q. Se traduce la descripción, nunca la tecla.
7. Plural real en el call site (claves `...One`/`...Many`): "1 mensaje" / "3 mensajes". Placeholders `{nombre}`.
8. Una palabra, un sentido: Eliminar (nunca Borrar), Salir (nunca Abandonar), Descargar (nunca Bajar), Programar (nunca Agendar), Elegir (nunca Escoger).
9. Sin gerundio como verbo principal; gerundio solo como progreso ("Cargando…", "Enviando…").
10. Signos de apertura ¿ ¡; comas y tildes completas.
11. Fechas: días y meses en minúscula; formato largo "martes, 14 de noviembre de 2023"; hora 24 h.
12. Un mismo literal EN comparte clave aunque aparezca en dos archivos ("Retry" en login y updates).

No se traduce: WhatsApp · WhatsFast · GIPHY · GitHub · Omarchy · nombres propios y datos del usuario (contactos, grupos, canales, archivos, carpetas) · GIF/GIFs · emoji (invariable) · sticker(s) (así lo usa la UI oficial ES) · atajos de teclado · comandos y rutas (snap refresh, whatsfast reload-themes, developers.giphy.com) · logs `log::*!` · panics/`expect`/`unreachable` · tails `{error}` del sistema operativo · ayuda de CLI (clap) · grupos de emoji del picker · metadatos de instalador/paquetes · Id/id_salt de egui · marcadores de protocolo ("Sticker Pack", magic RIFF/WEBP/GIF8/ANIM, OpusHead/OpusTags, SQL, args de ffmpeg).

## Menú de chat

| EN | ES |
|---|---|
| Pin to top | Fijar chat |
| Unpin | Dejar de fijar |
| Archive | Archivar |
| Unarchive | Desarchivar |
| Mute | Silenciar notificaciones |
| Unmute | Reactivar notificaciones |
| Mute for 8 hours / a week / indefinitely | Silenciar durante 8 horas / una semana / siempre |
| Leave group | Salir del grupo |
| Leave channel | Salir del canal |
| Leave group and archive | Salir del grupo y archivar |
| Leave this group? / Leave this channel? | ¿Salir de este grupo? / ¿Salir de este canal? |
| Copy number | Copiar número |
| Close chat | Cerrar chat |
| Select messages / Select all / Cancel selection | Seleccionar mensajes / Seleccionar todo / Cancelar selección |
| {count} selected | {count} seleccionados |
| Next wallpaper | Siguiente fondo |
| Forward | Reenviar |
| Star / Unstar | Destacar / Dejar de destacar |
| Starred / Starred messages | Destacados / Mensajes destacados |
| Download | Descargar |
| Info | Info (ALLOW_SAME, sin punto) |
| Report | Reportar |
| Block | Bloquear |
| Add to favorites / Remove from favorites | Añadir a favoritos / Quitar de favoritos |
| Mark as read / unread | Marcar como leído / como no leído |
| Reply / Copy text / Edit | Responder / Copiar texto / Editar |
| Delete for everyone / for me | Eliminar para todos / para mí |
| Save sticker / Open file / Show in folder | Guardar sticker / Abrir archivo / Mostrar en la carpeta |
| Open in a map | Abrir en el mapa |
| Search messages (Ctrl+G) | Buscar mensajes (Ctrl+G) |

## Composer y mensajes

| EN | ES |
|---|---|
| Type a message | Escribe un mensaje |
| Add a caption | Añade un comentario |
| Send | Enviar |
| Record a voice message | Grabar un mensaje de voz |
| Schedule this message | Programar este mensaje |
| Send files (or drop them on the window) | Enviar archivos (o arrástralos a la ventana) |
| Emoji, GIFs, and stickers | Emoji, GIF y stickers |
| Create poll | Crear encuesta |
| Editing message / Stop editing (Esc) | Editando mensaje / Detener la edición (Esc) |
| Replying to {who} / Cancel reply (Esc) | Respondiendo a {who} / Cancelar respuesta (Esc) |
| Discard | Descartar |
| Play / Pause / Playback speed | Reproducir / Pausar / Velocidad de reproducción |
| No messages here yet | Aún no hay mensajes aquí |
| Loading messages from your phone… | Cargando mensajes de tu teléfono… |
| Loading older messages from your phone… | Cargando mensajes anteriores de tu teléfono… |
| Newest message | Mensaje más reciente |
| Drop to send to {name} | Suelta para enviar a {name} |
| Sending {} file{}… | Enviando {} archivo{}… |
| Forwarded 1 of 3 / Starred 1 of 2 | Reenviado 1 de 3 / Destacado 1 de 2 |

## Estados y presencia

| EN | ES |
|---|---|
| typing… | escribiendo… |
| {one} is typing… / {} and {last} are typing… | {one} está escribiendo… / {} y {last} están escribiendo… |
| online | en línea |
| last seen {} | Última vez {} |
| Sent {} / Delivered / Delivered {} / Read / Played | Enviado {} / Entregado / Entregado {} / Leído / Reproducido |
| edited | editado |
| This message was deleted | Se eliminó este mensaje |
| Forwarded | Reenviado |
| Unsupported: {what} | No compatible: {what} |
| React with any emoji / Remove your reaction | Reaccionar con cualquier emoji / Quitar tu reacción |
| Muted / Muted until {} | Silenciado / Silenciado hasta {} |

## Grupos y canales

| EN | ES |
|---|---|
| Only admins can send messages | Solo los administradores pueden enviar mensajes |
| You left this group / channel | Saliste de este grupo / de este canal |
| {} members / Members ({}) | {} miembros / Miembros ({}) |
| Group / Channel / Contact | Grupo / Canal / Contacto |
| You were added | Te añadieron |
| Add to contacts / Rename | Añadir a contactos / Cambiar nombre |

## Encuestas

| EN | ES |
|---|---|
| Poll / Create poll | Encuesta / Crear encuesta |
| Question / Ask a question | Pregunta / Escribe una pregunta |
| Answers / Answer {} | Respuestas / Respuesta {} |
| Add answer / Remove answer | Añadir respuesta / Quitar respuesta |
| Allow multiple answers | Permitir varias respuestas |
| Select one answer / Select answers | Elige una respuesta / Elige las respuestas |
| Send poll / Sending… | Enviar encuesta / Enviando… |
| vote / votes / See votes | votar / votos / Ver votos |
| Voting key unavailable · use your phone | Llave de votación no disponible · usa tu teléfono |
| Reconnect to vote | Vuelve a conectar para votar |
| Waiting for your phone · earlier votes may be missing | Esperando a tu teléfono · pueden faltar votos anteriores |
| Enter a question of up to 255 characters. | Escribe una pregunta de hasta 255 caracteres. |
| Add 2-12 answers, each with 1-100 characters. | Añade de 2 a 12 respuestas, de 1 a 100 caracteres cada una. |
| Each answer must be different. | Cada respuesta debe ser distinta. |

## Adjuntos y media

| EN | ES |
|---|---|
| attachment / voice message / Photo / Video / Document / Audio / Location | adjunto / mensaje de voz / Foto / Video / Documento / Audio / Ubicación |
| No longer available on WhatsApp's servers | Ya no está disponible en los servidores de WhatsApp |
| We are still trying to get this file automatically. Click to retry manually. | Seguimos intentando conseguir este archivo automáticamente. Haz clic para reintentar. |
| Could not display this picture. Click to open it. | No se pudo mostrar esta imagen. Haz clic para abrirla. |
| Download / Downloaded attachments | Descargar / Adjuntos descargados |

## Ajustes (secciones y toggles)

| EN | ES |
|---|---|
| Settings | Ajustes |
| Appearance / Theme / Follow system / Dark / Light | Apariencia / Tema / Seguir el sistema / Oscuro / Claro |
| Open themes folder | Abrir carpeta de temas |
| Chat wallpaper / Auto | Fondo de chat / Automático |
| Zoom / Larger / Smaller / Reset zoom | Zoom / Más grande / Más pequeño / Restablecer zoom |
| Chats (section) | Chats (ALLOW_SAME) |
| Enter sends | Enter envía |
| When off, Enter adds a line and Ctrl+Enter sends. | Si está desactivado, Enter añade una línea y Ctrl+Enter envía. |
| Send read receipts | Enviar confirmaciones de lectura |
| Show when you are typing | Mostrar cuando escribes |
| Download attachments automatically | Descargar adjuntos automáticamente |
| Download older history in the background | Descargar historial anterior en segundo plano |
| Off / Current Chat / Recent and pinned | Desactivado / Chat actual / Recientes y fijados |
| Show sender pictures in every chat | Mostrar la foto del remitente en todos los chats |
| Names from your address book | Nombres de tu agenda |
| Save contacts to the phone's address book | Guardar contactos en la agenda del teléfono |
| Forward messages in order | Reenviar mensajes en orden |
| Show the create poll button | Mostrar el botón Crear encuesta |
| Show shortcut hints | Mostrar sugerencias de atajos |
| Privacy / Window / Account / Files / About | Privacidad / Ventana / Cuenta / Archivos / Acerca de |
| Keep running when the window closes | Seguir activo al cerrar la ventana |
| Notify about new messages | Notificar mensajes nuevos |
| Hide the sidebar completely | Ocultar la barra lateral por completo |
| Linked device / Unlink this computer | Dispositivo vinculado / Desvincular este equipo |
| Message archive / Open folder / Ask where to save each file | Archivo de mensajes / Abrir carpeta / Preguntar dónde guardar cada archivo |
| Log of this run | Registro de esta ejecución |
| Shortcuts / Check for updates / Download updates automatically | Atajos / Buscar actualizaciones / Descargar actualizaciones automáticamente |
| Version {} | Versión {} |

## Privacidad

| EN | ES |
|---|---|
| Last seen | Última vez |
| Online | En línea |
| Profile photo | Foto del perfil |
| About | Información |
| Who can add me to groups / call me / message me | Quién puede añadirme a grupos / llamarme / enviarme mensajes |
| Read receipts | Confirmaciones de lectura |
| Everybody / My contacts / My contacts except… / Except… / Nobody | Todos / Mis contactos / Mis contactos, excepto… / Excepto… / Nadie |
| Same as last seen | Igual que Última vez |
| My contacts and other people with my number | Mis contactos y otras personas con mi número |
| Hide last seen from / Hide profile photo from / Hide About from | Ocultar Última vez para / Ocultar foto del perfil para / Ocultar Información para |
| Could not load privacy settings. They load again when WhatsFast reconnects. | No se pudo cargar la privacidad. Vuelve a cargarse cuando WhatsFast se reconecte. |

## Updates, notificaciones y bandeja

| EN | ES |
|---|---|
| There's a new version available | Hay una versión nueva disponible |
| Update / Retry / Download from GitHub / Updated to {} | Actualizar / Reintentar / Descargar desde GitHub / Actualizado a {} |
| This installation is in a system directory. Use your package manager or the download page. | Esta instalación está en una carpeta del sistema. Usa tu gestor de paquetes o la página de descarga. |
| The update could not start. The previous version has been restored. | La actualización no pudo empezar. Se restauró la versión anterior. |
| Show or hide WhatsFast / Quit | Mostrar u ocultar WhatsFast / Salir |
| Open (notification action) | Abrir |
| {sender}: {summary} | {sender}: {summary} (solo formato) |
| Be right back / Back online | Vuelvo enseguida / De vuelta en línea |
| This device was unlinked from your phone | Este equipo se desvinculó de tu teléfono |
| History loaded / Copied / Sticker saved / Open a chat first | Historial cargado / Copiado / Sticker guardado / Abre un chat primero |
| Could not open {}: {error} | No se pudo abrir {}: {error} |
| The tray thread stopped responding | La bandeja dejó de responder |

## Fechas y hora (util.rs)

| EN | ES |
|---|---|
| Today / Yesterday | Hoy / Ayer |
| Yesterday at {time} | Ayer a las {time} |
| {} at {time} | {} a las {time} |
| Monday…Sunday | lunes…domingo (minúscula) |
| January…December / abbr. | enero…diciembre / ene…dic (clave propia, nunca `[..3]`) |
| 14 Nov 2023 | 14 nov 2023 |
| Tuesday, 14 November 2023 | martes, 14 de noviembre de 2023 |
| 22:41, 8/18/2026 (copy_stamp) | 22:41, 18/8/2026 (orden local d/m/a) |
| Once / Every day / Every {} / Day {day} of every month / {} {} of every month | Una vez / Todos los días / Todos los {} / El día {day} de cada mes / El {} {} de cada mes |
| Earlier / Later / Earlier minutes / Later minutes (tooltips) | Una hora antes / Una hora después / Cinco minutos antes / Cinco minutos después |
| Schedule / Time / Repeat / No scheduled messages | Programar / Hora / Repetir / No hay mensajes programados |

## Login y vinculación

| EN | ES |
|---|---|
| Link this computer / Unlink this computer? | Vincular este equipo / ¿Desvincular este equipo? |
| Open WhatsApp on your phone | Abre WhatsApp en tu teléfono |
| Tap Menu or Settings, then Linked devices | Toca Menú o Ajustes y luego Dispositivos vinculados |
| Tap Link a device and point the phone at this code | Toca Vincular un dispositivo y apunta el teléfono a este código |
| Link with phone number instead | Vincular con un número de teléfono |
| Enter this code on your phone / for +{phone} | Escribe este código en tu teléfono / para +{phone} |
| Copy code / Get a code | Copiar código / Obtener un código |
| Connecting to WhatsApp… | Conectando con WhatsApp… |
| Try again / Retry | Reintentar (un solo término en los dos sitios) |

## Buscador, lista y picker

| EN | ES |
|---|---|
| Search / Search chats / Search messages | Buscar / Buscar chats / Buscar mensajes |
| No results / No messages found | Sin resultados / No se encontraron mensajes |
| Try another name, number, or message text. | Prueba con otro nombre, número o texto. |
| All / Unread / Favorites / Groups | Todos / No leídos / Favoritos / Grupos |
| Archived chats / Nothing archived / Archived chats appear here. | Chats archivados / No hay nada archivado / Los chats archivados aparecen aquí. |
| No chats yet / No unread chats / No favorites yet / No groups | Aún no hay chats / No hay chats sin leer / Aún no hay favoritos / No hay grupos |
| Last message / Unread / Pinned (fila) | Último mensaje / Sin leer / Fijado |
| Search emoji / Frequently Used / Recent / Nothing matches | Buscar emoji / Usados frecuentemente / Recientes / Sin coincidencias |
| Emoji / GIF / Stickers (pestañas) | Emoji / GIF / Stickers |
| Search GIFs via GIPHY / Searching… | Buscar GIF en GIPHY / Buscando… |
| Loading your stickers… / Saved / Find packs / Open file | Cargando tus stickers… / Guardados / Buscar paquetes / Abrir archivo |
| Remove this pack / Remove from saved | Quitar este paquete / Quitar de guardados |

## macOS y ventana

| EN | ES |
|---|---|
| About WhatsFast / Settings… / Hide WhatsFast / Quit WhatsFast | Acerca de WhatsFast / Ajustes… / Ocultar WhatsFast / Salir de WhatsFast |
| File / New Contact… / Close Window | Archivo / Contacto nuevo… / Cerrar ventana |
| Edit / Undo / Redo / Cut / Copy / Paste / Select All / Find… | Edición / Deshacer / Rehacer / Cortar / Copiar / Pegar / Seleccionar todo / Buscar… |
| View / Toggle Sidebar / Zoom In / Zoom Out / Actual Size | Ver / Mostrar u ocultar barra lateral / Acercar / Alejar / Tamaño real |
| Window / Zoom / Show WhatsFast | Ventana / Zoom / Mostrar WhatsFast (el "Zoom" del menú Ventana es del SO: no traducir) |
| Help / Keyboard Shortcuts / WhatsFast Help | Ayuda / Atajos de teclado / Ayuda de WhatsFast |
| Close the window (WhatsFast remains in the tray) | Cerrar la ventana (WhatsFast queda en la bandeja) |

## Copia de selección (marcadores y cabecera)

| EN | ES |
|---|---|
| [photo] / [video] / [GIF] / [audio] / [sticker] | [foto] / [video] / [GIF] / [audio] / [sticker] |
| [voice message] / [voice message, 0:12] | [nota de voz] / [nota de voz, 0:12] |
| [document: notes.pdf] | [documento: notes.pdf] |
| [location] / [contact: Nombre] / [poll: Pregunta] | [ubicación] / [contacto: Nombre] / [encuesta: Pregunta] |
| (replying to Ada: "…") | (en respuesta a Ada: "…") |
| header [hh:mm, d/m/a] Nombre: | sale de `copy_stamp` (B8), sin término nuevo |

## Errores de subsistemas (audio, voice, stickers, archivo, temas)

| EN | ES |
|---|---|
| The clip is empty | El clip está vacío |
| Could not decode audio: {error} | No se pudo decodificar el audio: {error} |
| No sound output: {error} | Sin salida de sonido: {error} |
| No microphone available: {error} | No hay micrófono disponible: {error} |
| The microphone has no supported format: {error} | El micrófono no tiene un formato compatible: {error} |
| bad OGG stream / not an Opus stream / truncated Opus header | secuencia OGG no válida / no es una secuencia Opus / cabecera Opus truncada |
| sticker pack / Missing pack_id. Copy the full signal.art link | paquete de stickers / Falta pack_id. Copia el enlace completo de signal.art |
| Could not decrypt the sticker pack | No se pudo descifrar el paquete de stickers |
| This file is not a sticker archive: {error} | Este archivo no es un paquete de stickers: {error} |
| archive (store de mensajes) | archivo de mensajes (desambigua de "archivo" = file) |
| OS keyring | llavero del sistema |
| encrypted archive / unlock | archivo cifrado / desbloquear (cifrado, no "encriptado") |
| The archive is encrypted but its OS keyring key is missing. | El archivo está cifrado, pero falta su clave en el llavero del sistema. |
| Loading local themes… | Cargando temas locales… |
| The selected theme is unavailable. Keeping the last usable appearance. See the log for details. | El tema seleccionado no está disponible. Se mantiene la última apariencia válida. Consulta el registro para más detalles. |
| The themes folder could not be read. See the log for details. | No se pudo leer la carpeta de temas. Consulta el registro para más detalles. |
| Custom themes could not be loaded. Run whatsfast reload-themes to try again. | No se pudieron cargar los temas personalizados. Ejecuta whatsfast reload-themes para volver a intentarlo. |

## Casos difíciles (decisión + motivo)

- Leave group → "Salir del grupo" (oficial; "Abandonar" suena administrativo).
- Unarchive → "Desarchivar" ("Restaurar" colisiona con versiones).
- Unpin → "Dejar de fijar" (patrón oficial "Dejar de destacar").
- Mute / Unmute → "Silenciar notificaciones" / "Reactivar notificaciones" (la etiqueta sola deja ambiguo qué se silencia).
- "No longer available on WhatsApp's servers" → "Ya no está disponible en los servidores de WhatsApp" (el sujeto lo da la burbuja).
- "You left this group" → "Saliste de este grupo" (aviso, no instrucción).
- "Keep running when the window closes" → "Seguir activo al cerrar la ventana".
- "Show when you are typing" → "Mostrar cuando escribes".
- "History prefetch" → "Descargar historial anterior en segundo plano" (etiqueta de acción; el sustantivo se reserva a estados).
- Wallpaper → "Fondo" (menú) / "Fondo de chat" (Ajustes).
- Sticker → "Sticker" (la UI oficial ES lo conserva; ALLOW_SAME).
- Poll → "Encuesta"; votar → "votar"; votos → "votos".
- Channel → "Canal"; Broadcast → "Difusión"; Status → "Estado" (nunca para presencia: eso es "en línea"/"Última vez").
- Backup → "copia de seguridad"; Shortcut → "Atajo"; Schedule → "Programar".
- Zoom → "Zoom" con acciones "Acercar"/"Alejar".
- Delivered / Read / Sent / Played → "Entregado / Leído / Enviado / Reproducido" (masculino por "mensaje" elidido).
- "This message was deleted" → "Se eliminó este mensaje".
- "Only admins can send messages" → "Solo los administradores pueden enviar mensajes" (los 3 literales con color pasan a 1 clave).
- "Type a message" → "Escribe un mensaje"; "Add a caption" → "Añade un comentario" (oficial).
- Chats (sección) → "Chats" ALLOW_SAME (alineación con la UI oficial; "Conversaciones" descartado).
- emoji → "emoji" invariable ("emoticono" es otra cosa).
- Delete → "Eliminar" ("Borrar" se reserva a contenido multimedia).
- Vincular / Desvincular (nunca "Iniciar/Cerrar sesión" para el dispositivo).

## NEEDS_REVIEW resueltos

| Clave | Conflicto | Decisión |
|---|---|---|
| last seen | Oficial abrevia "últ. vez" | "Última vez" completo (ETS evita abreviaturas) |
| Info | Oficial muestra "Info." | "Info" sin punto |
| Chats | Anglicismo | "Chats" ALLOW_SAME |
| sticker | Anglicismo | "sticker" minúscula |
| Chat wallpaper | Oficial "Fondo de pantalla" | "Fondo de chat" / "Siguiente fondo" |
| Played | Sin cadena oficial | "Reproducido" |
| Schedule | Función nueva | "Programar" |
| Report | es_ES diría "Denunciar" | "Reportar" |

## Método (gates)

- G0: antes de traducir un archivo, cada cadena visible tiene fila en `AGENTS/i18n-progress.md` (fuente: `AGENTS/i18n-triage-20260919.txt`).
- G1: match exhaustivo de `Key` en cada locale (el compilador es el primer test).
- G2: test de no-vacío + ES != EN salvo `ALLOW_SAME` (derivada de este glosario).
- G3: un commit por superficie coherente; el diff es la prueba.
- G4: `--demo-shot` en ES y EN de las superficies que crecen; PNG revisado.
- G5: cuadre de conteos (claves del módulo == filas traducidas == cadenas del inventario de esa superficie).
- G6: anti-regresión: una superficie traducida no vuelve a tener literales visibles en inglés (rg).
