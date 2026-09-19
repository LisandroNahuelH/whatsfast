# GitHub release upload (Windows, no Actions)

Stable method used after `gh release upload` and multi-asset `gh release create` failed or timed out on large binaries (~20–45 MB). **Ship only** `whatsfast-v*-*-pc-windows-msvc-setup.exe` (see `AGENTS.md` Releasing).

## What failed (do not rely on)

| Approach | Typical failure |
|----------|-----------------|
| `gh release create` with several assets at once | Draft stuck; only small files (e.g. checksums) attach |
| `gh release upload` on large `.exe` | Very slow; `HTTP 404` on `uploads.github.com` after long runs |
| Parallel uploads (multiple `gh`/`curl`) | Contention; 404s and aborted transfers |
| Wrong `curl` URL (encoding / hostname) | Immediate 404 or `Bad hostname` |

## What works

1. **One asset, one upload, sequential.** Use **`curl.exe`** against GitHub’s **upload** host, not `api.github.com` for the file body.
2. **Draft first, publish after** the installer asset shows `state: uploaded` on the release.
3. **No parallel** upload processes for the same release.

## Procedure

Replace `v0.15.1` and paths with the version you are shipping.

### 1. Build installer

From repo root (Inno Setup 6 required; see `packaging/windows/whatsfast.iss`):

```powershell
cd "C:\path\to\WhatsFast"
$version = "0.15.1"
cargo build --locked --release
$iscc = "${env:ProgramFiles(x86)}\Inno Setup 6\ISCC.exe"
if (-not (Test-Path $iscc)) { throw "Install Inno Setup 6" }
New-Item -ItemType Directory -Force dist | Out-Null
& $iscc "/DVersion=$version" "/DArch=x86_64" `
  "/DBinary=$PWD\target\release\whatsfast.exe" `
  "/DOutputDir=$PWD\dist"
```

Artifact: `dist\whatsfast-v$version-x86_64-pc-windows-msvc-setup.exe`

### 2. Tag and draft release

Commit version bump, push `main`, then tag and push the tag. Create a **draft** release with notes only (no assets yet):

```powershell
gh release create "v$version" -R LisandroNahuelH/whatsfast `
  --title "WhatsFast v$version" `
  --notes-file "AGENTS\release-notes-v$version.md" `
  --verify-tag --draft
```

If a broken draft already exists for that tag, delete it once (`gh release delete "v$version" -y`) and recreate, or reuse the draft and skip create.

### 3. Upload setup.exe (curl)

```powershell
cd dist
$releaseId = gh api repos/LisandroNahuelH/whatsfast/releases/tags/v$version --jq '.id'
$token = gh auth token
$file = "whatsfast-v${version}-x86_64-pc-windows-msvc-setup.exe"
$url = "https://uploads.github.com/repos/LisandroNahuelH/whatsfast/releases/${releaseId}/assets?name=${file}"

curl.exe --retry 5 --retry-delay 15 --retry-all-errors -sS -f -X POST `
  -H "Authorization: Bearer $token" `
  -H "Accept: application/vnd.github+json" `
  -H "Content-Type: application/octet-stream" `
  --data-binary "@${file}" `
  "$url"
```

Wait until this command finishes (can take several minutes on a slow uplink). Verify:

```powershell
gh api repos/LisandroNahuelH/whatsfast/releases/tags/v$version `
  --jq '.assets[] | select(.name | endswith("-setup.exe")) | {name,size,state}'
```

### 4. Publish

```powershell
gh release edit "v$version" -R LisandroNahuelH/whatsfast --draft=false
```

Open `https://github.com/LisandroNahuelH/whatsfast/releases/tag/v$version` and confirm one installer asset downloads.

## Notes

- **Optional checksums:** not required for the current policy. If you add `checksums.txt`, upload it in a **second** curl call after the setup.exe succeeds, then publish.
- **TLS / API errors:** retry `gh api` queries; do not start a second curl upload for the same filename until the first exits.
- **v0.15.1 history:** that release was published with setup + zip before the setup-only policy; new releases follow this runbook only.
