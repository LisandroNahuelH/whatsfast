# i18n ES: estado y cierre verificado

Seguimiento del trabajo de traduccion (ingles canonico -> espanol) de WhatsFast.
Generado y actualizado por el agente; no es documentacion de producto.

## Piezas

| Pieza | Ruta |
|-------|------|
| Tabla de claves | `src/i18n/key.rs` (788 claves) |
| Textos EN / ES | `src/i18n/en.rs`, `src/i18n/es.rs` |
| Modulo (global, t/f/count, deteccion del sistema) | `src/i18n/mod.rs` |
| Fuente de verdad de filas y reemplazos | `AGENTS/i18n-strings.json` |
| Generador (idempotente) | `AGENTS/i18n-gen.py` |
| Escaneo residual | `AGENTS/i18n-residual.py` |
| Glosario ETS/WhatsApp | `AGENTS/i18n-glosario-es.md` |
| Test de etiquetas ES (binario propio) | `tests/i18n_es.rs` |
| Test del idioma (binario propio) | `tests/i18n_language.rs` |
| Paquetes del consejo de verificadores | `AGENTS/i18n-verify-package-1.txt`, `-2.txt` |

Regenerar todo: `python AGENTS/i18n-gen.py` (crea las tablas y aplica los
reemplazos pendientes). Revisar restos: `python AGENTS/i18n-residual.py`.

## Cierre verificado

- Estado del arbol de la corrida: ver `AGENTS/i18n-close-sha.txt` (SHA corto y
  `git status --porcelain` del momento).
- Corrida de cierre: `AGENTS/i18n-close-run.txt` (`cargo test --locked
  --all-targets --no-fail-fast`) y `AGENTS/i18n-close-run-features.txt`
  (`--all-features`). Ambas terminan en 0 fallos: 417 y 418 en la lib, 1 en
  `i18n_language`, 2 en `i18n_es`.
- Escaner antes del fix: `residual: none` era falso; `safe_literal` cortaba en
  el primer `&'static str` y ocultaba 26 literales de `src/privacy.rs` mas los
  2 de `src/model.rs`. El generador marcaba el sitio `privacy-literals` como
  aplicado sin haber reemplazado nada. Ambos scripts usan ahora la regla por
  declaracion (el tramo debe **empezar** con `const`/`static`), el generador no
  marca un sitio sin reemplazos, y el escaner excluye por `(archivo, texto)` las
  dos colisiones que no son interfaz: el valor de protocolo `online`
  (`privacy.rs`) y el TLD `online` (`markup.rs`), mas los nombres localizados
  que `is_fallback_name` reconoce como marcadores viejos.
- Escaner despues del fix: `residual: none` real (con el heuristico corregido).
- Piso del test ES: mapa medido por pagina (ver `FLOOR` en `tests/i18n_es.rs`);
  reemplaza el `>= 3` anterior y ahora cubre 14 paginas, con Ajustes pintando
  111 etiquetas (antes la mitad baja quedaba fuera del viewport de 1180x780).
- Atribucion corregida: el estado previo al i18n es `fe8b94b` (padre de
  `af141d1`). `0d60b06` **no** es pre-i18n: contiene `af141d1..b037cb1`.
- Flake resuelto: `a_startup_language_is_read_back` cambia el idioma global del
  proceso; vivia en el binario de la lib junto a tests que pintan etiquetas en
  ingles. Se movio a `tests/i18n_language.rs` (patron de `tests/i18n_es.rs`).
- Que cubre cada control: la tabla (788 claves, placeholders y
  `ALLOWED_IDENTICAL`) la prueban los tests de `src/i18n/mod.rs`; el test ES
  prueba el chrome pintado de las 14 paginas cosechadas; el escaner prueba los
  call sites que todavia tienen literales. Limites: el escaner solo ve textos
  que ya son fila EN, y saltea `src/demo*.rs` (contenido de muestra y el guion
  del tour, que corre en ingles a proposito).
- "i18n verificado" significa: los comandos de la corrida de cierre en verde
  sobre un arbol limpio al SHA registrado, mas la comprobacion a ojo de la
  bandeja y los menus de macOS despues de reiniciar (se construyen una vez).

## Estado por superficie

Traducido: ajustes (incluido el selector de idioma y la tarjeta de privacidad),
fechas y horas (`util.rs`, `schedule.rs`, `ui/schedule.rs`, `ui/pane.rs`), lista
de chats, conversacion (incluido el sello `editado` y la presencia `en linea`),
panel de busqueda, picker (emoji/GIF/stickers), encuestas (plural de votos por
`i18n::count`), vinculacion, dialogo de atajos, dialogos, visor, aviso de
actualizacion, bandeja y notificaciones, menus de macOS, temas personalizados y
Omarchy, importacion de stickers, audio y microfono, actualizaciones (install,
macos, transfer), llavero del archivo, `app.rs`/`model.rs`/`privacy.rs`.

Nombres de respaldo: un chat sin nombre real guarda el marcador
`model::FALLBACK_NAME` y las vistas pintan `KindGroup`/`DisplayYou`; los
archivos viejos que guardaron `Group`/`Grupo`/`You`/`Tu` se reconocen con
`model::is_fallback_name` y se vuelven a pedir.

No se traduce (por politica): registros de protocolo y SQL, panics, ids y
marcadores de wire, contenido de muestra de `demo.rs`, README, empaquetado
(`packaging/*`), nombres de paleta, constantes de marca, el aviso del segundo
arranque (`single_instance.rs`) y el volcado de `--verbose`. La lista exacta de
filas identicas EN/ES vive en `src/i18n/mod.rs` (`ALLOWED_IDENTICAL`).

Deuda declarada (fuera de este trabajo): el mismo modo de falla que tenia
`language` (un valor invalido en un enum de `settings.json` tira todo el
archivo a default y el guardado lo hace permanente) sigue vivo en `theme`,
`history_prefetch` y `chat_wallpaper`. `language` ya es tolerante
(`read_language`); los otros tres quedan para un cambio aparte.
