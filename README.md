# WhatsFast

**WhatsApp for Windows, native and fast.** WhatsFast is a linked-device client
written in Rust with [egui](https://github.com/emilk/egui). It uses
[whatsapp-rust](https://github.com/oxidezap/whatsapp-rust) for the protocol.
There is no browser engine. Link your phone, keep history on this PC, and chat
from the desktop.

WhatsFast is **Windows desktop only** for downloads and support. Builds and
releases target portable ZIP and installer `.exe` on Windows.

WhatsApp is a trademark of Meta Platforms, Inc. WhatsFast is not affiliated with
WhatsApp or Meta. Use it only as a linked device paired with your own phone.

## Why WhatsFast

- Native **Rust + egui** UI (no Chromium shell).
- **Local encrypted archive** on your machine (SQLCipher + OS keyring).
- Desktop UX: tray, notifications, themes, voice messages, polls, and more.
- Features below ship in this repository and grow with each release.

## WhatsFast improvements

- Settings can hide the create poll button beside the attachment menu.
- The composer caret stays visible while the WhatsApp window has focus.
Features land through reviewed pull requests on this repo. This section updates
as each one merges. (Inaugural baseline: upstream ZapFast 0.14.x capabilities
on Windows; local enhancements follow in order.)

## Install (Windows)

Download the latest **Windows** release from
[GitHub Releases](https://github.com/LisandroNahuelH/whatsfast/releases):
portable ZIP or installer `.exe`, plus `checksums.txt`.

## Build (Windows)

Install Rust (stable), CMake, and the Windows SDK toolchain OpenSSL expects.
Then:

```sh
cargo build --release
```

Run `target\release\zapfast.exe` until the rebrand PR lands (`whatsfast.exe`).

## Requirements

- Windows 10 or later (64-bit).
- A phone with WhatsApp to scan the QR code or link by number.

## Upgrading from ZapFast

WhatsFast adopts data from earlier app names on first run (`fastsapp`,
`fastwhatsapp`, `zapfast`). Your archive and settings move under a `whatsfast`
folder after rebrand. Relink the phone if you want the linked-device name to
show **WhatsFast** instead of ZapFast.

## License

MIT. See [LICENSE](LICENSE).

## Attributions

See [ATTRIBUTIONS.md](ATTRIBUTIONS.md).
