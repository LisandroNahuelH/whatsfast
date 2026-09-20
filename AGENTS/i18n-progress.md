# i18n ES: estado y pendientes

Seguimiento del trabajo de traduccion (ingles canonico -> espanol) de WhatsFast.
Generado y actualizado por el agente; no es documentacion de producto.

## Piezas

| Pieza | Ruta |
|-------|------|
| Tabla de claves | `src/i18n/key.rs` (754 claves) |
| Textos EN / ES | `src/i18n/en.rs`, `src/i18n/es.rs` |
| Modulo (global, t/f/count, deteccion del sistema) | `src/i18n/mod.rs` |
| Fuente de verdad de filas y reemplazos | `AGENTS/i18n-strings.json` |
| Generador (idempotente) | `AGENTS/i18n-gen.py` |
| Escaneo residual | `AGENTS/i18n-residual.py` |
| Glosario ETS/WhatsApp | `AGENTS/i18n-glosario-es.md` |
| Test de etiquetas ES (binario propio) | `tests/i18n_es.rs` |

Regenerar todo: `python AGENTS/i18n-gen.py` (crea las tablas y aplica los
reemplazos pendientes). Revisar restos: `python AGENTS/i18n-residual.py`.

## Estado por superficie

Traducido: ajustes (incluido el selector de idioma), fechas y horas
(`util.rs`, `schedule.rs`, `ui/schedule.rs`, `ui/pane.rs`), lista de chats,
conversacion, panel de busqueda, picker (emoji/GIF/stickers), encuestas,
vinculacion, dialogo de atajos, dialogos, visor, aviso de actualizacion,
bandeja y notificaciones, menus de macOS, temas personalizados y Omarchy,
importacion de stickers, audio y microfono, actualizaciones (install, macos,
transfer), llavero del archivo, `app.rs`/`model.rs`/`privacy.rs` (avisos y
etiquetas).

Pendiente de reaplicar (el agente paralelo reescribio estos cuatro archivos
despues del commit `af141d1` y borro parte de la traduccion): `src/app.rs`,
`src/ui/settings.rs`, `src/backend/worker.rs`, `src/model.rs`. Al cerrar su
trabajo: borrar `AGENTS/i18n-sites-state.json` y correr el generador; despues
`AGENTS/i18n-residual.py` para confirmar que no queden literales EN.

No se traduce (por politica): registros de protocolo y SQL, panics, ids y
marcadores de wire, contenido de muestra de `demo.rs`, README, empaquetado
(`packaging/*`), nombres de paleta, constantes de marca, el aviso del segundo
arranque (`single_instance.rs`) y el volcado de `--verbose`.
