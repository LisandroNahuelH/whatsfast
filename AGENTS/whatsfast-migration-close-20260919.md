# WhatsFast migration close (2026-09-19)

| Item | Value |
|------|--------|
| Repo | https://github.com/LisandroNahuelH/whatsfast |
| Local root | `C:\OfiSync\0. Lisandro\0. Programacion\WhatsFast` |
| Upstream | `crmne/zapfast` (read-only remote) |
| Legacy cherry-pick | `local-zapfast` → ZapFast folder |
| Tag | `v0.15.0` |
| crmne PRs | #76 #77 closed |
| Fork zapfast | archived, pointer to whatsfast |
| Portfolio PRs merged | #1 inaugural, #3–#11 G1–G9, #12 recovery (history), #13 rebrand, #14 Windows/docs/0.15.0, #15 clippy/icons |
| allow-forking=false | Not on personal public repo (GitHub 422); documented in DECISIONS.md |

## Notes

- G1–G9 PRs exist on GitHub; `main` was force-aligned to `local/select-messages` plus WhatsFast docs after cherry-pick integration broke compile.
- Icon: `packaging/icons/whatsfast-1024.png` generated; `whatsfast.ico` placeholder from legacy until multi-res ICO regen.
- Tests: `cargo test --locked --all-targets` 331 passed on Windows (2026-09-19).
- jev: skip (quota/time; validation via fmt/clippy/test).
