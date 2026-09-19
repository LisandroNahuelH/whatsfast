WhatsFast **v0.15.1** is the first Windows download on this repository: native linked-device client, Rust and egui, no browser engine.

## Install (Windows)

Requires **Windows 10 or 11**, **64-bit (x86_64)**.

Pick one of the assets below (all are in **`checksums.txt`**, SHA-256):

- **`whatsfast-v0.15.1-x86_64-pc-windows-msvc-setup.exe`**: wizard installer (recommended). Installs under `%LOCALAPPDATA%\Programs\WhatsFast` without admin rights.
- **`whatsfast-v0.15.1-x86_64-pc-windows-msvc.zip`**: portable folder (exe, README, LICENSE, `whatsfast-portable.txt`). Extract anywhere and run **`whatsfast.exe`**.
- **`whatsfast-v0.15.1-x86_64-pc-windows-msvc.exe`**: standalone binary only. Put it in a folder of your choice or replace an existing copy.

Pair from **WhatsApp on your phone → Linked devices** after the first launch.

## New

- **WhatsFast** branding and Windows-focused release under [LisandroNahuelH/whatsfast](https://github.com/LisandroNahuelH/whatsfast).
- Features carried from the maintainer branch: multi-select messages, scheduled sends, starred list, pinned reorder, composer and settings polish (see README).

## Fixed

- **Upgrading from ZapFast:** message archive unlock on Windows when data lives under `%LOCALAPPDATA%\paolino\*\data\`, including automatic migration of the ZapFast credential manager key.

## Thanks

Lisandro Nahuel Hillebrand, maintainer.

Full changelog: https://github.com/LisandroNahuelH/whatsfast/compare/v0.15.0...v0.15.1
