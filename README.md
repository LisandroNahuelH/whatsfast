# WhatsFast

**The most powerful, robust, fast, and complete native WhatsApp linked-device
client for Windows.** WhatsFast is written in Rust with
[egui](https://github.com/emilk/egui). It uses
[whatsapp-rust](https://github.com/oxidezap/whatsapp-rust) for the protocol.
There is no browser engine.

WhatsFast is **Windows 10 and 11 desktop only** for downloads and support.

**Maintainer:** [LisandroNahuelH](https://github.com/LisandroNahuelH) only. This repository is not the upstream ZapFast project; see [ATTRIBUTIONS.md](ATTRIBUTIONS.md) for credits.

WhatsApp is a trademark of Meta Platforms, Inc. WhatsFast is not affiliated with
WhatsApp or Meta.

## WhatsFast improvements

- The composer caret stays visible while the WhatsApp window has focus.
- Settings can hide the create poll button beside the attachment menu.
- Forwarded batches can send one message at a time to preserve order, with a Settings toggle.
- Click the active settings section again to close the panel.
- Star messages from the chat and browse them in a sidebar list with bubble previews.
- Schedule messages once or on a repeat rule from the composer clock control.
- A narrow sidebar rail keeps navigation visible in every view.
- Hold and drag pinned chats to reorder them on the chat list.
- Select multiple messages from the bubble menu or row, then forward, download, or star the batch.

## Upgrading from ZapFast

If you used ZapFast on this PC, link again after installing WhatsFast. Your
linked device entry may still show the old name until you unlink and pair.
WhatsFast reads the same archive path after the one-time folder rename from
`zapfast` to `whatsfast` under your app data directory.

## Install (Windows)

Download the latest release from
[GitHub Releases](https://github.com/LisandroNahuelH/whatsfast/releases).

## Build (Windows)

```sh
cargo build --release
```

Run fmt, clippy, and tests locally before you push (see `AGENTS.md`).

## License

MIT. See [LICENSE](LICENSE).

## Attributions

See [ATTRIBUTIONS.md](ATTRIBUTIONS.md).
