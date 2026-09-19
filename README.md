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

Each item below is a WhatsFast change on top of the upstream ZapFast baseline. Under every title: what it does in the app, then why it helps day to day.

### Mark as unread from the chat list

- **Technical:** Right-click a chat in the left list (or the compact rail) and choose **Mark as unread**. The row uses the unread style with an empty round badge and no number. Opening the chat, **Mark as read**, or a real new message clears it. Counted unread badges stay numbered.
- **Daily use:** Flag a thread you still need to answer without pretending there is a pending count. Same empty dot as WhatsApp Desktop.

### One-click in-app updates

- **Technical:** With check and download enabled (both on by default), a sticky toast says a new version is available. One click on **Update** installs the GitHub release, restarts WhatsFast, and reopens the last chat. Installer and portable builds only; package-manager installs keep their own update path.
- **Daily use:** You do not hunt for a setup file. Click Update when the toast appears, wait a few seconds, and keep working in the same chat.

### In-app photo viewer with zoom and pan

- **Technical:** Clicking a downloaded photo in a chat opens a full-window viewer in the app. The mouse wheel zooms toward the pointer, click-and-hold moves the photo, and the arrow keys (or on-screen chevrons) step to the previous or next downloaded photo in that chat. Escape, the close control, or a click on the dark background closes it. Stickers, videos, and the bubble menu **Open file** still use the system handler.
- **Daily use:** You can read a screenshot or a document photo without leaving WhatsFast or waiting for Photos to open. Zoom in on the part you need, move around, then go back to the thread.

### Composer caret stays visible while the window has focus

- **Technical:** The message composer keeps a visible text caret (insertion point) whenever the WhatsApp window has keyboard focus, including after repaints and layout updates in egui.
- **Daily use:** You always see where the next character will land. Long replies, edits, and paste-at-cursor work feel like a normal desktop editor, not a web view that hides the caret.

### Hide the create-poll control in Settings

- **Technical:** Settings exposes a toggle that removes the create-poll entry from the attachment menu next to the composer. The rest of poll viewing and voting is unchanged.
- **Daily use:** If you never create polls, the menu stays shorter and you stop mis-tapping poll when you wanted a file or photo. Cleaner composer for work chats that rarely use polls.

### Sequential forwarding for batched messages

- **Technical:** When you forward several messages at once, Settings **Forward messages in order** (on by default) sends them one after another. Each one waits until the message before it shows its first tick, so mixed text, pictures, and videos keep the original chat order. When off, they send together and may arrive out of order.
- **Daily use:** Instructions, numbered steps, and screenshots stay in the order you selected. Better for handoffs, checklists, and “read top to bottom” threads at work.

### Close Settings by clicking the active section again

- **Technical:** The settings panel treats a second click on the already selected section as “close panel,” without adding a separate dismiss control.
- **Daily use:** One click to open a section, one click on the same row to get back to the chat. Fewer stray clicks when you only wanted to tweak one option.

### Starred messages with sidebar list and bubble previews

- **Technical:** Messages can be starred from the chat; stars are stored in the archive and shown in a dedicated sidebar list with bubble-style previews (same rendering path as the main transcript).
- **Daily use:** Bookmark decisions, links, and client notes and find them later without scrolling the whole history. The preview shows context so you know which star is which.

### Scheduled sends (once or on a repeat rule)

- **Technical:** The composer clock control schedules outbound messages for a future time, once or on a repeat rule defined in the app; the backend queues them and sends when due.
- **Daily use:** Remind a team Monday morning, ping a contact in their timezone, or send a follow-up without staying online. Handy for support windows and async work.

### Narrow sidebar rail for navigation

- **Technical:** A slim rail stays visible at the edge of the UI so chat list, starred, scheduled, and related navigation remain one click away in every view.
- **Daily use:** You do not lose your place when switching tasks. Jump between inbox, stars, and scheduled sends without hunting for hidden menus.

### Reorder pinned chats by drag and hold

- **Technical:** Pinned chats in the list support click-and-drag reorder; order is persisted so the archive reflects your chosen pin stack.
- **Daily use:** Put your boss, active project, and family groups where you want them every day. Less scrolling past pins you rarely open.

### Multi-select messages for forward, download, and star

- **Technical:** Bubble menu and row actions enter a selection mode across multiple messages; batch forward, save attachments, or star applies to the whole selection with one confirmation path.
- **Daily use:** Forward a week of updates, save every PDF from a thread, or star a run of messages in one go. Less repetitive right-click work in heavy group chats.

## Upgrading from ZapFast

If you used ZapFast on this PC, link again after installing WhatsFast. Your
linked device entry may still show the old name until you unlink and pair.
WhatsFast reads the same archive path after the one-time folder rename from
`zapfast` to `whatsfast` under your app data directory. The Windows credential
manager key from ZapFast is picked up automatically on first launch.

## Install (Windows)

Download the latest release from
[GitHub Releases](https://github.com/LisandroNahuelH/whatsfast/releases).
The Windows installer then updates itself from GitHub: a toast, one click on
**Update**, and a restart back into your last chat.

## Build (Windows)

```sh
cargo build --release
```

Run fmt, clippy, and tests locally before you push (see `AGENTS.md`).

## License

WhatsFast is distributed under the **[MIT License](LICENSE)**.

The `LICENSE` file names **Lisandro Nahuel** as copyright holder for this
repository. Portions derive from ZapFast (see [ATTRIBUTIONS.md](ATTRIBUTIONS.md)).
You may use, modify, and redistribute the software under the conditions in
that file. The software is provided **as is**, without warranty.

For a full list of credits and third-party components, see
[Attributions](#attributions) below and [ATTRIBUTIONS.md](ATTRIBUTIONS.md).

## Attributions

WhatsFast stands on work by many open-source authors. **Thank you** to:

- **[Carmine Paolino](https://github.com/crmne)** and contributors of
  **[ZapFast](https://github.com/crmne/zapfast)** — foundation of this client
  (MIT).
- **[oxidezap/whatsapp-rust](https://github.com/oxidezap/whatsapp-rust)** —
  WhatsApp linked-device protocol.
- **[egui](https://github.com/emilk/egui)** — native UI toolkit.
- **Lucide** icon authors ([license](assets/icons/LICENSE.txt)) and **Inter**
  font authors ([license](assets/fonts/Inter-LICENSE.txt)) — interface assets.

WhatsApp is a trademark of **Meta Platforms, Inc.** WhatsFast is independent
and is not affiliated with WhatsApp or Meta.

Details, maintainer information, and dependency notes:
**[ATTRIBUTIONS.md](ATTRIBUTIONS.md)**.