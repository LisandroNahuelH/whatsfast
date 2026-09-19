# Architecture

WhatsFast is a native WhatsApp linked-device client. Structural layout matches
the upstream ZapFast lineage; product name and remotes differ.

## Remotes

| Remote | URL | Use |
|--------|-----|-----|
| `origin` | `LisandroNahuelH/whatsfast` | Push, releases, internal PRs |
| `upstream` | `crmne/zapfast` | Read-only fixes and features |
| `local-zapfast` | Local legacy clone (optional) | Cherry-pick source for portfolio PRs |

After every merge or cherry-pick from `upstream`, run the rebrand pass documented
in `AGENTS/whatsfast-upstream-sync.md` (once that file exists).

## Layers

- **`src/ui/`** — egui views; emit `model::Action`; no direct archive access.
  Clicking a downloaded chat photo opens `ui/viewer.rs`, a full-window overlay
  with wheel zoom, click-and-drag pan, and previous/next among downloaded
  photos in that chat. Stickers, videos, and **Open file** still use the
  system handler.
- **`src/app.rs`** — Applies actions after each frame.
- **`src/backend.rs` / `src/backend/worker.rs`** — Tokio worker thread; owns
  whatsapp-rust `Bot`, archive, downloads, profile pictures. Talks to UI via
  `Command` and `Event`. Serial `Command::Forward` keeps a queue and starts
  the next send only after `Command::Sent` writes `Delivery::Sent` (first
  tick) or `Failed`; parallel batches still use `forward_batch`.
- **`src/archive.rs`** — SQLite (SQLCipher) message store; single copy after
  link-time history sync. `chats.marked_unread` is a local empty-dot reminder;
  real `unread` counts still come from the phone. Opening the chat or a new
  incoming message clears the flag.
- **`src/model.rs`** — App types; worker translates protobuf in `classify()`.

See root `AGENTS.md` for invariants (privacy, polls, receipts, selection, updates).

## Platform scope

Supported product: **Windows desktop** releases. Verification runs **locally**
only (see `AGENTS.md` *Definition of done*). There are no GitHub Actions
workflows. Non-Windows code may remain temporarily but is not supported or released.
