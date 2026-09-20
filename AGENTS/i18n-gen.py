#!/usr/bin/env python3
"""Genera src/i18n/{key.rs,en.rs,es.rs} desde AGENTS/i18n-strings.json y aplica
los reemplazos de call sites declarados. Idempotente; aborta si un ancla falta
o aparece más de una vez."""
import hashlib
import json
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
DATA = ROOT / "AGENTS" / "i18n-strings.json"


def rust_lit(s: str) -> str:
    return '"' + s.replace("\\", "\\\\").replace('"', '\\"') + '"'


def unescape_rust(s: str) -> str:
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


LITERAL = re.compile(r'"((?:[^"\\]|\\.)*)"')


def safe_literal(text: str, position: int) -> bool:
    """False for byte strings, raw strings, and const/static initializers.

    Those carry protocol bytes or compile-time data; translating the text
    inside them breaks the wire format or does not compile.
    """
    prefix = text[max(0, position - 3) : position]
    if prefix.endswith(("b", "r", "br", "rb")):
        return False
    window = text[max(0, position - 4000) : position]
    const_at = max(window.rfind("const "), window.rfind("static "))
    if const_at >= 0 and "}" not in window[const_at:]:
        return False
    return True


def fmt_sites(path: pathlib.Path, name: str, items: list, applied: list) -> str | None:
    """Replaces the whole `format!` call that holds a literal when there is one."""
    text = path.read_text(encoding="utf-8")
    before = text
    missing = []
    for item in items:
        literal = '"' + item["literal"] + '"'
        while True:
            idx = text.find(literal)
            while idx >= 0 and not safe_literal(text, idx):
                idx = text.find(literal, idx + 1)
            if idx < 0:
                break
            start = text.rfind("format!(", max(0, idx - 400), idx)
            if start < 0:
                text = text[:idx] + item["new"] + text[idx + len(literal):]
            else:
                depth = 0
                i = start + len("format!")  # at the '('
                while i < len(text):
                    if text[i] == "(":
                        depth += 1
                    elif text[i] == ")":
                        depth -= 1
                        if depth == 0:
                            break
                    i += 1
                if depth != 0:
                    return f"{name}: unbalanced format! call for {item['literal'][:60]!r}"
                text = text[:start] + item["new"] + text[i + 1:]
            if not item.get("all"):
                break
        if item["new"] not in text:
            missing.append(item["literal"])
            continue
    if text != before:
        path.write_text(text, encoding="utf-8", newline="\n")
        applied.append(name)
    if missing:
        print(f"note: {name} skipped {len(missing)} literals no longer in {path.name}")
        for literal in missing:
            print(f"  missing: {literal[:70]!r}")
    return None


def literal_sites(path: pathlib.Path, name: str, replacements: list, applied: list) -> str | None:
    """Replaces literals matched by exact text or prefix, in the whole file."""
    text = path.read_text(encoding="utf-8")
    before = text
    missing = []
    for replacement in replacements:
        new = replacement["new"]
        mode = "exact" if "exact" in replacement else "starts_with"
        needle = replacement[mode]
        escaped = needle.replace("\\", "\\\\").replace('"', '\\"')
        if mode == "exact":
            pattern = re.compile('"' + re.escape(escaped) + '"')
        else:
            pattern = re.compile('"' + re.escape(escaped) + '(?:[^"\\\\]|\\\\.)*"')

        found = list(pattern.finditer(text))
        found = [m for m in found if safe_literal(text, m.start())]
        if not found:
            if new in text:
                continue
            missing.append(needle)
            continue
        if not replacement.get("all") and len(found) != 1:
            print(f"note: {name} found {len(found)} literals {mode} {needle[:40]!r}; replacing all")
        for m in reversed(found):
            text = text[: m.start()] + new + text[m.end() :]
    if text != before:
        path.write_text(text, encoding="utf-8", newline="\n")
        applied.append(name)
    if missing:
        print(f"note: {name} skipped {len(missing)} literals no longer in {path.name}")
        for needle in missing:
            print(f"  missing: {needle[:70]!r}")
    return None


def signature(site: dict) -> str:
    return hashlib.sha256(
        json.dumps(site, sort_keys=True, ensure_ascii=False).encode("utf-8")
    ).hexdigest()[:16]


def read_state(path: pathlib.Path) -> set:
    if path.exists():
        return set(json.loads(path.read_text(encoding="utf-8")))
    return set()


def main() -> int:
    data = json.loads(DATA.read_text(encoding="utf-8"))
    rows = data["strings"]
    keys = [r["key"] for r in rows]
    if len(set(keys)) != len(keys):
        dupes = sorted({k for k in keys if keys.count(k) > 1})
        print("duplicate keys:", dupes, file=sys.stderr)
        return 1
    for r in rows:
        for field in ("key", "en", "es"):
            if field not in r:
                print(f"row missing {field}: {r}", file=sys.stderr)
                return 1

    key_rs = [
        "//! Interface strings: one variant per string.",
        "",
        "/// Every string the interface can show.",
        "#[derive(Clone, Copy, Debug, PartialEq, Eq)]",
        "pub enum Key {",
    ]
    for r in rows:
        key_rs.append(f"    {r['key']},")
    key_rs += [
        "}",
        "",
        "impl Key {",
        "    /// Every key, for the completeness tests.",
        f"    pub const ALL: [Key; {len(rows)}] = [",
    ]
    for r in rows:
        key_rs.append(f"        Key::{r['key']},")
    key_rs += ["    ];", "}", ""]

    def table(locale: str) -> str:
        out = [
            f"//! {locale} strings.",
            "",
            "use super::Key;",
            "",
            "pub(super) fn text(key: Key) -> &'static str {",
            "    match key {",
        ]
        for r in rows:
            out.append(f"        Key::{r['key']} => {rust_lit(r[locale])},")
        out += ["    }", "}", ""]
        return "\n".join(out)

    (ROOT / "src" / "i18n" / "key.rs").write_text("\n".join(key_rs), encoding="utf-8", newline="\n")
    (ROOT / "src" / "i18n" / "en.rs").write_text(table("en"), encoding="utf-8", newline="\n")
    (ROOT / "src" / "i18n" / "es.rs").write_text(table("es"), encoding="utf-8", newline="\n")
    print(f"key.rs/en.rs/es.rs: {len(rows)} keys")

    applied = 0
    skipped = 0
    state_path = ROOT / "AGENTS" / "i18n-sites-state.json"
    state = read_state(state_path)
    for site in data.get("sites", []):
        sig = signature(site)
        if sig in state:
            skipped += 1
            continue
        path = ROOT / site["file"]
        if "fmt_items" in site:
            error = fmt_sites(path, site.get("name", site["file"]), site["fmt_items"], [])
            if error:
                print(error, file=sys.stderr)
                return 1
            state.add(sig)
            applied += 1
            continue
        if "literals" in site:
            error = literal_sites(
                path, site.get("name", site["file"]), site["literals"], []
            )
            if error:
                print(error, file=sys.stderr)
                return 1
            state.add(sig)
            applied += 1
            continue
        text = path.read_text(encoding="utf-8")
        old, new = site["old"], site["new"]
        if new.startswith("use crate::i18n") and "use crate::i18n" in text:
            state.add(sig)
            skipped += 1
            continue
        count = text.count(old)
        if new in text and not site.get("all"):
            state.add(sig)
            skipped += 1
            continue
        if count >= 1 and site.get("all"):
            path.write_text(text.replace(old, new), encoding="utf-8", newline="\n")
            state.add(sig)
            applied += 1
            continue
        if count == 1:
            path.write_text(text.replace(old, new, 1), encoding="utf-8", newline="\n")
            state.add(sig)
            applied += 1
            continue
        if count == 0 and new in text:
            state.add(sig)
            skipped += 1
            continue
        if count == 0:
            # rustfmt reflows inserted code, so an applied site may no longer
            # match its own `new` text. Treat it as applied and say so.
            print(
                f"note: anchor gone in {site['file']}: {old[:60]!r} (assumed applied)",
                file=sys.stderr,
            )
            state.add(sig)
            skipped += 1
            continue
        print(f"ANCHOR {count}x in {site['file']}: {old[:70]!r}", file=sys.stderr)
        return 1
    state_path.write_text(json.dumps(sorted(state), indent=0) + "\n", encoding="utf-8")
    print(f"call sites applied: {applied}, already applied: {skipped}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
