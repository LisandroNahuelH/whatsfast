# Windows-only CI and packaging audit

WhatsFast targets **Windows x64 MSVC** only. This documents what was removed from CI/release and what stays deferred.

## Removed CI targets (2026-09-19)

| Former target | Workflow | Notes |
|---------------|----------|-------|
| `ubuntu-latest` (quality + test) | `ci.yml` | Quality and tests run on `windows-latest`. |
| `macos-latest` (test, demo shot, preview artifact) | `ci.yml` | macOS window smoke test removed. |
| `windows-latest` / `aarch64-pc-windows-msvc` compile check | `ci.yml` | `windows-arm64` job removed. |

## Removed release builds

| Former target / job | Workflow | Notes |
|---------------------|----------|-------|
| `x86_64-unknown-linux-gnu` | `release.yml` | No `.tar.gz` Linux portable. |
| `aarch64-unknown-linux-gnu` | `release.yml` | No arm64 Linux build. |
| `aarch64-pc-windows-msvc` | `release.yml` | No Windows arm64 release. |
| `macos` (universal DMG, notarize) | `release.yml` | Entire job removed. |
| `flatpak` (Flathub bundle) | `release.yml` | Entire job removed; needs Linux artifact. |

## Kept

- `windows-latest` + `x86_64-pc-windows-msvc` release build.
- Artifacts: `whatsfast-<tag>-x86_64-pc-windows-msvc.zip`, Inno Setup `*-setup.exe`, `checksums.txt` on GitHub Releases.
- `packaging.yml` workflow hook after publish (stable tags only).

## Deferred (not deleted from repo)

- `packaging/flatpak/` — unused until a Linux product exists again.
- `packaging/macos/` — unused until macOS returns to scope.
- Linux desktop files under `packaging/applications/` — legacy ZapFast paths; not shipped by WhatsFast releases.
