# WhatsFast upstream sync runbook

Rebrand pass after pulling changes from upstream (crmne/zapfast or whatsapp-rust).

## Before merge

1. Fetch upstream tags and `main`. Read the release notes for breaking UI or protocol changes.
2. List files that still carry ZapFast naming: `grep -ri zapfast -- .` (exclude `DECISIONS.md`, git history, and this runbook).
3. Confirm Windows-only CI still matches `.github/workflows/ci.yml` and `release.yml` (single `x86_64-pc-windows-msvc` target).

## Rebrand checklist

| Area | Action |
|------|--------|
| `Cargo.toml` / binary name | Keep `whatsfast`; do not revert to `zapfast`. |
| User-facing strings | WhatsFast, not ZapFast. |
| Icons | `packaging/icons/whatsfast-1024.png`, `packaging/windows/whatsfast.ico`. |
| Portable marker | `packaging/whatsfast-portable.txt`. |
| Installer | `packaging/windows/whatsfast.iss`. |
| Release artifacts | `whatsfast-v*-<target>.zip`, `whatsfast-*-setup.exe`. |
| Homepage / repo URLs | `LisandroNahuelH/whatsfast` only in this fork. |

## After merge

1. Run full local checks from `AGENTS.md` (fmt, clippy, tests, doc).
2. Bump version if shipping; tag `vX.Y.Z` only on `main` after merge.
3. Update `DECISIONS.md` if scope or packaging policy changes.

## Do not restore

- Linux/macOS release matrix entries or Flatpak/macOS packaging jobs removed for Windows-only scope (see `AGENTS/windows-only-audit.md`).
