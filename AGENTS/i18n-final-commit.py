"""Commit final del trabajo i18n: bump de version, paths explicitos y push."""
import pathlib
import re
import subprocess

ROOT = pathlib.Path(__file__).resolve().parents[1]


def git(*args: str, check: bool = True) -> str:
    result = subprocess.run(
        ["git", *args], cwd=ROOT, capture_output=True, text=True, check=check
    )
    return result.stdout + result.stderr


count = int(git("rev-list", "--count", "HEAD").strip())
version = f"0.15.{100 + count + 1}"
cargo = ROOT / "Cargo.toml"
text = cargo.read_text(encoding="utf-8")
text = re.sub(r'version = "0\.15\.\d+"', f'version = "{version}"', text, count=1)
cargo.write_text(text, encoding="utf-8", newline="\n")
lock = ROOT / "Cargo.lock"
lines = lock.read_text(encoding="utf-8").split("\n")
for i, line in enumerate(lines):
    if line == 'name = "whatsfast"':
        lines[i + 1] = f'version = "{version}"'
        break
lock.write_text("\n".join(lines), encoding="utf-8", newline="\n")
print("version:", version)

staged = [
    "Cargo.toml",
    "Cargo.lock",
    "src/app.rs",
    "src/backend/worker.rs",
    "src/i18n/en.rs",
    "src/i18n/es.rs",
    "src/i18n/mod.rs",
    "src/notify.rs",
    "src/privacy.rs",
    "src/updates/macos.rs",
    "AGENTS/i18n-gen.py",
    "AGENTS/i18n-residual.py",
    "AGENTS/i18n-strings.json",
    "AGENTS/i18n-batch2.py",
    "AGENTS/i18n-batch3.py",
    "AGENTS/i18n-batch4.py",
    "AGENTS/i18n-batch5.py",
    "AGENTS/i18n-batch6.py",
    "AGENTS/i18n-batch7.py",
    "AGENTS/i18n-batch8.py",
    "AGENTS/i18n-batch9.py",
    "AGENTS/i18n-batch10.py",
    "AGENTS/i18n-batch11.py",
    "AGENTS/i18n-batch12.py",
    "AGENTS/i18n-batch13.py",
    "AGENTS/i18n-batch14.py",
    "AGENTS/i18n-fix-build.py",
]
git("add", "--", *staged)
print(git("diff", "--cached", "--name-only"))
message = """feat(i18n): finish the Spanish sweep and keep the tables in step

Re-applies the translation to app.rs, backend/worker.rs, privacy.rs and
updates/macos.rs after the storage-stats and privacy rewrites reverted it.
Keeps protocol bytes, const tables and egui::Key names untouched: the
generator now skips byte and raw strings, const initializers and repeated
imports. Container messages in worker.rs take the value that is in scope.
"""
git("commit", "-m", message)
print(git("log", "--oneline", "-1"))
print(git("push", "origin", "main"))
