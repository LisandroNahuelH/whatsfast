# Windows-only packaging audit

WhatsFast targets **Windows x64 MSVC** only.

## GitHub

- **No GitHub Actions** (`allow_actions=disabled`). No `.github/workflows/`.
- Verification: run *Definition of done* in `AGENTS.md` on your PC before push.
- Releases: build the Inno Setup installer locally; publish **only** `*-setup.exe` on GitHub Releases (see `AGENTS.md` Releasing).

## Removed upstream automation (historical)

crmne triage, Pages, Flatpak, packaging matrix, and Windows CI jobs on GitHub were removed.

## Deferred in tree (not shipped)

- `packaging/flatpak/`, `packaging/macos/`, Linux desktop files, `native-packages.yaml` (upstream templates).
