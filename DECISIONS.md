# Decision log

Append-only, newest first.

| Date | Decision | Why | Replaces |
|------|----------|-----|----------|
| 2026-09-19 | End each coding turn with `cargo build --locked` (debug) | Incremental rebuild so `target/debug/whatsfast.exe` is ready to try at once | Waiting for `--release` or a later `cargo run` that may only surface the installer copy |
| 2026-09-19 | One-click update toast; download auto ON by default | Wizard users update with one click; last chat already restores | Modal Update WhatsFast + explicit Restart |
| 2026-09-19 | Chat photos open in an in-app viewer | Click must show the photo here, with zoom and pan. The system viewer stays on **Open file**, stickers, and video | `Action::OpenFile` on photo click |
| 2026-09-19 | No GitHub Actions; Actions disabled in repo settings | Solo dev on PC; GH CI only slowed pushes | CI + Release workflows |
| 2026-09-19 | Branch protection removed | No required checks without Actions | protection on `main` |
| 2026-09-19 | Removed crmne Actions (triage, docs Pages, flatpak, packaging) | WhatsFast is not the upstream project; CI = one Windows job + Release on tags | — |
| 2026-09-19 | Post-verify fixes: default branch main, no Release→packaging job | GitHub showed inaugural branch; Actions queue blocked Release | — |
| 2026-09-19 | Jev gate skipped on G1–G9 integration PRs | quota/time; full checks run on Windows main before tag | — |
| 2026-09-19 | whatsfast.ico still ZapFast pixels until PNG→ICO tool run | SHA match legacy; PNG 1024 ships in repo | — |
| 2026-09-19 | WhatsFast fork from crmne/zapfast | portfolio + Windows scope | — |
| 2026-09-19 | Windows-only product | no multi-OS releases | — |
| 2026-09-19 | fork allow-forking N/A on personal repo | — | — |
| 2026-09-19 | main recovery force-push | unrelated histories | — |
