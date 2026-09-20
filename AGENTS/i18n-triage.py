"""Triage del inventario i18n: candidatos reales por archivo, excluyendo tests.

Read-only sobre src/. Salida: AGENTS/i18n-triage-20260919.txt
"""
import re
import pathlib

LIT = re.compile(r'"((?:[^"\\]|\\.)*)"')
TEST_START = re.compile(r"^\s*#\[cfg\(test\)\]")
SINK = re.compile(
    r"(\blabel\(|\btext\(|rich_text|\bline\(|toggle\(|section\(|menu_item\(|bar_button\(|"
    r"icon_button\(|circle_button\(|setting_row\(|widgets::|theme::|toast|dialog|title|"
    r"placeholder|hint|format!|push_str\(|writeln!|write!\(|\.map\(|=>)"
)
ID_SALT = re.compile(r"(Id::new|id_salt|from_id_salt|make_persistent_id|push_id)")
PROTOCOL = re.compile(
    r"(@s\.whatsapp\.net|@g\.us|@lid|@newsletter|@broadcast|OggS|application/|image/|"
    r"audio/|video/|text/|BLACKLIST|CARGO_PKG_VERSION|[A-Za-z0-9+/]{40,}=*)"
)


def interesting(s: str) -> bool:
    if len(s) < 2:
        return False
    if "\\u" in s or "\\n" in s or "\\t" in s or "\\r" in s:
        return False
    if s.startswith("http") or "://" in s:
        return False
    if re.match(r"^[a-z0-9_.:/@#-]+$", s):
        return False
    if not re.search(r"[A-Za-z]", s):
        return False
    return (" " in s) or (s[0].isupper()) or ("{" in s)


files = sorted(p for p in pathlib.Path("src").rglob("*.rs"))
out = []
summary = []
for p in files:
    rel = p.as_posix()
    lines = p.read_text(encoding="utf-8").splitlines()
    test_from = None
    for i, ln in enumerate(lines):
        if TEST_START.match(ln):
            test_from = i
            break
    rows = []
    for i, ln in enumerate(lines, 1):
        if ln.lstrip().startswith("//"):
            continue
        if re.search(r"(log::|debug!|info!|warn!|error!|trace!|eprintln!|println!|dbg!)", ln):
            continue
        for m in LIT.finditer(ln):
            s = m.group(1)
            if not interesting(s):
                continue
            tags = []
            if test_from is not None and i - 1 >= test_from:
                tags.append("test")
            if ID_SALT.search(ln):
                tags.append("idsalt")
            if PROTOCOL.search(s):
                tags.append("proto")
            if SINK.search(ln):
                tags.append("sink")
            if not tags:
                tags.append("plain")
            rows.append((i, s, ln.strip()[:150], ",".join(tags)))
    prod = [r for r in rows if "test" not in r[3]]
    if rows:
        out.append(f"### {rel}  total={len(rows)} prod={len(prod)}")
        for i, s, ctx, tags in rows:
            out.append(f"{i}\t[{tags}]\t{s!r}\t|| {ctx}")
        out.append("")
        summary.append((rel, len(rows), len(prod), len(prod) and sum(1 for r in prod if "idsalt" in r[3] or "proto" in r[3])))

pathlib.Path("AGENTS/i18n-triage-20260919.txt").write_text("\n".join(out), encoding="utf-8")
summary.sort(key=lambda x: -x[2])
tot = sum(s[1] for s in summary)
totp = sum(s[2] for s in summary)
print(f"files={len(summary)} total={tot} prod={totp}")
print(f"{'file':60s} total prod excl")
for rel, t, pr, ex in summary:
    print(f"{rel:60s} {t:5d} {pr:5d} {str(ex):5s}")
