"""Escaneo residual: literales en ingles de la tabla EN que siguen sueltos en el codigo.

Uso: python AGENTS/i18n-residual.py
Lista, por archivo, cada literal cuyo texto coincide con una fila EN de la tabla
y que no esta ya dentro de una llamada i18n::t/f/count ni en las tablas generadas.
"""
import json
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
DATA = ROOT / "AGENTS" / "i18n-strings.json"
LITERAL = re.compile(r'"((?:[^"\\]|\\.)*)"')

SKIP_FILES = {
    "src/i18n/key.rs",
    "src/i18n/en.rs",
    "src/i18n/es.rs",
    "src/i18n/mod.rs",
    # Sample content and the tour script stay in English: the tour clicks its
    # own labels and the recording runs on an English build.
    "src/demo.rs",
    "src/demo/tour.rs",
    "src/demo/tour/media.rs",
    "src/demo/tour/session.rs",
}


DECL = re.compile(
    r"(?s)^(pub\([^)]*\)\s+|pub\s+)?(unsafe\s+)?(const|static)\s+[A-Za-z_][A-Za-z_0-9]*\s*:"
)


# Text collisions that are not interface: the protocol's own wire value and a
# TLD inside the link parser. Translating either one breaks the wire format or
# the parser, so the scan skips them by (file, text).
SCAN_EXCLUDE = {
    ("src/privacy.rs", "online"),
    ("src/markup.rs", "online"),
    # Legacy stand-ins kept on purpose: `is_fallback_name` recognizes the
    # localized names written by older builds. They are data, not interface.
    ("src/model.rs", "Group"),
    ("src/model.rs", "Grupo"),
    ("src/model.rs", "You"),
    ("src/model.rs", "Tú"),
    ("src/model.rs", "Tu"),
}


def safe_literal(text: str, position: int) -> bool:
    """False for byte strings, raw strings, and const/static initializers.

    A literal belongs to a const/static only when the statement holding it
    starts with that declaration: a `&'static str` in a signature, or an `=>`
    further up the line, must not hide interface text.
    """
    prefix = text[max(0, position - 3) : position]
    if prefix.endswith(("b", "r", "br", "rb")):
        return False
    start = max(text.rfind(";", 0, position), text.rfind("}", 0, position))
    return not DECL.match(text[start + 1 : position])


def unescape(s: str) -> str:
    out = []
    i = 0
    while i < len(s):
        c = s[i]
        if c == "\\" and i + 1 < len(s):
            nxt = s[i + 1]
            out.append({"n": "\n", "r": "\r", "t": "\t", '"': '"', "\\": "\\"}.get(nxt, nxt))
            i += 2
            continue
        out.append(c)
        i += 1
    return "".join(out)


def main() -> int:
    data = json.loads(DATA.read_text(encoding="utf-8"))
    english = {}
    for row in data["strings"]:
        english.setdefault(row["en"], row["key"])

    report = []
    for path in sorted(ROOT.glob("src/**/*.rs")):
        rel = path.relative_to(ROOT).as_posix()
        if rel in SKIP_FILES:
            continue
        text = path.read_text(encoding="utf-8")
        lines = text.split("\n")
        # El modulo de tests suele cerrar el archivo: todo lo que sigue a su
        # `#[cfg(test)] mod` queda fuera del escaneo.
        cutoff = len(lines)
        for i, line in enumerate(lines):
            if line.strip() == "#[cfg(test)]":
                for j in range(i + 1, min(i + 4, len(lines))):
                    if lines[j].strip().startswith("mod "):
                        cutoff = i
                        break
                else:
                    continue
                break
        offset = 0
        in_test = False
        depth = 0
        for line_no, line in enumerate(lines[:cutoff], start=1):
            stripped = line.strip()
            if stripped == "#[test]":
                in_test = True
                depth = 0
            elif in_test:
                depth += line.count("{") - line.count("}")
                if depth <= 0:
                    in_test = False
            line_offset = offset
            offset += len(line) + 1
            if in_test or stripped.startswith("//") or stripped.startswith("///"):
                continue
            for m in re.finditer(r'"((?:[^"\\]|\\.)*)"', line):
                content = unescape(m.group(1))
                if content not in english:
                    continue
                if (rel, content) in SCAN_EXCLUDE:
                    continue
                if not safe_literal(text, line_offset + m.start()):
                    continue
                before = line[: m.start()]
                if "i18n::" in before or "Key::" in before:
                    continue
                if "=>" in before and "Key::" in line:
                    continue
                report.append((rel, line_no, english[content], content))

    if not report:
        print("residual: none")
        return 0
    by_file = {}
    for rel, line_no, key, content in report:
        by_file.setdefault(rel, []).append((line_no, key, content))
    total = len(report)
    for rel, rows in by_file.items():
        print(f"{rel} ({len(rows)})")
        for line_no, key, content in rows:
            print(f"  {line_no}: [{key}] {content[:80]}")
    print(f"total: {total} literals in {len(by_file)} files")
    return 0


if __name__ == "__main__":
    sys.exit(main())
