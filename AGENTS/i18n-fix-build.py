"""Arregla los errores de compilacion que dejo la reconciliacion."""
import pathlib

fixes = [
    # app.rs: falta el import
    (
        "src/app.rs",
        "use crate::backend::{Backend, Command, Event, LinkStatus, Waker};",
        "use crate::backend::{Backend, Command, Event, LinkStatus, Waker};\nuse crate::i18n::{self, Key};",
    ),
    # picker.rs: import duplicado (queda el alias I18nKey)
    (
        "src/ui/picker.rs",
        "use crate::i18n::{self, Key};\nuse crate::i18n::{self, Key as I18nKey};",
        "use crate::i18n::{self, Key as I18nKey};",
    ),
    # notify.rs: import duplicado (queda el de Linux)
    (
        "src/notify.rs",
        "use crate::i18n::{self, Key};\n\n#[cfg(target_os = \"linux\")]\nuse crate::i18n::{self, Key};",
        "#[cfg(target_os = \"linux\")]\nuse crate::i18n::{self, Key};",
    ),
    # worker.rs: argumentos con nombre equivocado
    (
        "src/backend/worker.rs",
        '&[("name", name)],\n                )));\n                self.emit_chat(&id);',
        '&[("name", &name)],\n                )));\n                self.emit_chat(&id);',
    ),
    (
        "src/backend/worker.rs",
        '&[("name", &number.to_string())],',
        '&[("name", &crate::util::phone(&phone))],',
    ),
    (
        "src/backend/worker.rs",
        'self.emit(Event::Info(i18n::f(Key::ToastPackAdded, &[("name", name)])));',
        'self.emit(Event::Info(i18n::f(\n                        Key::ToastPackAdded,\n                        &[("name", &name)],\n                    )));',
    ),
    (
        "src/backend/worker.rs",
        'Key::ToastSavedTo,\n                                    &[("path", &path.to_string())],',
        'Key::ToastSavedTo,\n                                    &[("path", &saved.display().to_string())],',
    ),
    (
        "src/backend/worker.rs",
        'Key::ToastSavedTo,\n                        &[("path", &path.to_string())],',
        'Key::ToastSavedTo,\n                        &[("path", &saved.display().to_string())],',
    ),
]

for name, old, new in fixes:
    path = pathlib.Path(name)
    text = path.read_text(encoding="utf-8")
    count = text.count(old)
    if count == 0:
        print(f"SKIP  {name}: anchor missing {old[:60]!r}")
        continue
    path.write_text(text.replace(old, new, 1), encoding="utf-8", newline="\n")
    print(f"OK    {name}: {old[:50]!r}")
