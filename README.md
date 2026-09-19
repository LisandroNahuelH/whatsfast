<div align="center">

<img src="docs/assets/images/app-mark.png" alt="WhatsFast" width="96" height="96" />

# WhatsFast

**The most powerful, robust, fast, and complete native WhatsApp linked-device client for Windows.**

Rust + [egui](https://github.com/emilk/egui). Protocol: [whatsapp-rust](https://github.com/oxidezap/whatsapp-rust). **No browser engine.**

<br />

[![Windows 10 and 11](https://img.shields.io/badge/Windows-10%20%26%2011-0078D4?style=for-the-badge&logo=windows&logoColor=white)](https://github.com/LisandroNahuelH/whatsfast/releases)
[![Latest release](https://img.shields.io/github/v/release/LisandroNahuelH/whatsfast?style=for-the-badge&color=E85D04)](https://github.com/LisandroNahuelH/whatsfast/releases)
[![Rust](https://img.shields.io/badge/Rust-egui-DEA584?style=for-the-badge&logo=rust&logoColor=000)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-1b4332?style=for-the-badge)](LICENSE)
[![Native](https://img.shields.io/badge/Engine-native%20%7C%20no%20Chromium-0b1c17?style=for-the-badge)](#install-windows)

<br />

<a href="https://github.com/LisandroNahuelH/whatsfast/releases">
  <img src="https://img.shields.io/badge/Download_installer-GitHub_Releases-E85D04?style=for-the-badge&logo=github&logoColor=white" alt="Download the Windows installer from GitHub Releases" />
</a>

<br />

<img src="docs/assets/images/hero-banner.png" alt="WhatsFast: native WhatsApp client for Windows" width="920" />

<img src="docs/assets/images/demo-chat.png" alt="WhatsFast demo window: chat list, photo, voice messages, document, and link preview. Synthetic sample data only." width="920" />

<sub>Screenshot from the offline demo. No real chats.</sub>

</div>

<p align="center">
  <a href="#whatsfast-improvements">Features</a> ·
  <a href="#install-windows">Install</a> ·
  <a href="#upgrading-from-zapfast">Upgrade from ZapFast</a> ·
  <a href="#build-windows">Build</a> ·
  <a href="#license">License</a> ·
  <a href="#attributions">Credits</a>
</p>

---

WhatsFast is **Windows 10 and 11 desktop only** for downloads and support.

**Maintainer:** [LisandroNahuelH](https://github.com/LisandroNahuelH) only. This repository is not the upstream ZapFast project; see [ATTRIBUTIONS.md](ATTRIBUTIONS.md) for credits.

WhatsApp is a trademark of Meta Platforms, Inc. WhatsFast is not affiliated with WhatsApp or Meta.

## WhatsFast improvements

Each item below is a WhatsFast change on top of the upstream ZapFast baseline. Under every title: what it does in the app, then why it helps day to day.

<table>
<tr>
<td width="50%" valign="top">

### Mark as unread from the chat list

- **Technical:** Right-click a chat in the left list (or the compact rail) and choose **Mark as unread**. The row uses the unread style with an empty round badge and no number. Opening the chat, **Mark as read**, or a real new message clears it. Counted unread badges stay numbered.
- **Daily use:** Flag a thread you still need to answer without pretending there is a pending count. Same empty dot as WhatsApp Desktop.

</td>
<td width="50%" valign="top">

### One-click in-app updates

- **Technical:** With check and download enabled (both on by default), a sticky toast says a new version is available. One click on **Update** installs the GitHub release, restarts WhatsFast, and reopens the last chat. Installer and portable builds only; package-manager installs keep their own update path.
- **Daily use:** You do not hunt for a setup file. Click Update when the toast appears, wait a few seconds, and keep working in the same chat.

</td>
</tr>
<tr>
<td width="50%" valign="top">

### In-app photo viewer with zoom and pan

- **Technical:** Clicking a downloaded photo in a chat opens a full-window viewer in the app. The mouse wheel zooms toward the pointer, click-and-hold moves the photo, and the arrow keys (or on-screen chevrons) step to the previous or next downloaded photo in that chat. Escape, the close control, or a click on the dark background closes it. Stickers, videos, and the bubble menu **Open file** still use the system handler.
- **Daily use:** You can read a screenshot or a document photo without leaving WhatsFast or waiting for Photos to open. Zoom in on the part you need, move around, then go back to the thread.

</td>
<td width="50%" valign="top">

### Composer caret stays visible while the window has focus

- **Technical:** The message composer keeps a visible text caret (insertion point) whenever the WhatsApp window has keyboard focus, including after repaints and layout updates in egui.
- **Daily use:** You always see where the next character will land. Long replies, edits, and paste-at-cursor work feel like a normal desktop editor, not a web view that hides the caret.

</td>
</tr>
<tr>
<td width="50%" valign="top">

### Hide the create-poll control in Settings

- **Technical:** Settings exposes a toggle that removes the create-poll entry from the attachment menu next to the composer. The rest of poll viewing and voting is unchanged.
- **Daily use:** If you never create polls, the menu stays shorter and you stop mis-tapping poll when you wanted a file or photo. Cleaner composer for work chats that rarely use polls.

</td>
<td width="50%" valign="top">

### Background download of older chat history

- **Technical:** Settings **Download older history in the background** (Recent and pinned by default) slowly asks the phone for older messages and then downloads their files up to 64 MB. Failed files are asked again with a long wait, for up to 30 days. It fills the local archive, not the visible thread. Off turns it off. This chat covers only the open conversation. Recent and pinned covers every pinned chat plus the ten most recently active chats that are not pinned. It keeps running from the tray.
- **Daily use:** Scroll up later and the older text, photos, videos, and documents are already there, without hitting WhatsApp's rate limit from a fast flick.

### Sequential forwarding for batched messages

- **Technical:** When you forward several messages at once, Settings **Forward messages in order** (on by default) sends them one after another. Each one waits until the message before it shows its first tick, so mixed text, pictures, and videos keep the original chat order. When off, they send together and may arrive out of order.
- **Daily use:** Instructions, numbered steps, and screenshots stay in the order you selected. Better for handoffs, checklists, and “read top to bottom” threads at work.

</td>
</tr>
<tr>
<td width="50%" valign="top">

### Close Settings by clicking the active section again

- **Technical:** The settings panel treats a second click on the already selected section as “close panel,” without adding a separate dismiss control.
- **Daily use:** One click to open a section, one click on the same row to get back to the chat. Fewer stray clicks when you only wanted to tweak one option.

</td>
<td width="50%" valign="top">

### Starred messages with sidebar list and bubble previews

- **Technical:** Messages can be starred from the chat; stars are stored in the archive and shown in a dedicated sidebar list with bubble-style previews (same rendering path as the main transcript).
- **Daily use:** Bookmark decisions, links, and client notes and find them later without scrolling the whole history. The preview shows context so you know which star is which.

</td>
</tr>
<tr>
<td width="50%" valign="top">

### Scheduled sends (once or on a repeat rule)

- **Technical:** The composer clock control schedules outbound messages for a future time, once or on a repeat rule defined in the app; the backend queues them and sends when due.
- **Daily use:** Remind a team Monday morning, ping a contact in their timezone, or send a follow-up without staying online. Handy for support windows and async work.

</td>
<td width="50%" valign="top">

### Narrow sidebar rail for navigation

- **Technical:** A slim rail stays visible at the edge of the UI so chat list, starred, scheduled, and related navigation remain one click away in every view.
- **Daily use:** You do not lose your place when switching tasks. Jump between inbox, stars, and scheduled sends without hunting for hidden menus.

</td>
</tr>
<tr>
<td width="50%" valign="top">

### Reorder pinned chats by drag and hold

- **Technical:** Pinned chats in the list support click-and-drag reorder; order is persisted so the archive reflects your chosen pin stack.
- **Daily use:** Put your boss, active project, and family groups where you want them every day. Less scrolling past pins you rarely open.

</td>
<td width="50%" valign="top">

### Multi-select messages for forward, download, and star

- **Technical:** Bubble menu and row actions enter a selection mode across multiple messages; batch forward, save attachments, or star applies to the whole selection with one confirmation path.
- **Daily use:** Forward a week of updates, save every PDF from a thread, or star a run of messages in one go. Less repetitive right-click work in heavy group chats.

</td>
</tr>
</table>

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

<p align="center">
  <a href="https://github.com/LisandroNahuelH/whatsfast/releases">
    <img src="https://img.shields.io/badge/Open_Releases-whatsfast--v*-*-setup.exe-0078D4?style=for-the-badge&logo=windows&logoColor=white" alt="Open GitHub Releases for the Windows installer" />
  </a>
</p>

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
