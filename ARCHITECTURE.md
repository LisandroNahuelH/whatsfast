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
  On Windows and Linux the viewport has no OS decorations. `titlebar_strip`
  paints a 27 px themed caption (`theme::client_chrome`) with Lucide min/max/close
  and `ViewportCommand` drag, minimize, maximize, and close. `F11` sends
  `ToggleFullscreen` and hides that bar. macOS keeps native traffic lights.
  The open chat paints a bundled doodle wallpaper behind the bubbles, scaled
  to cover the panel. Each family has three PNGs; Settings names them
  `Black 1` and so on, and the chat menu **Next wallpaper** steps the slot.
  Settings keeps a plain `palette.chat` fill.
  Clicking a chat photo or video opens `ui/viewer.rs`, a full-window overlay
  with a header, the file in the centre, and a chronological filmstrip of
  that chat's images and videos from the archive. The overlay sizes those
  panes from the window `content_rect` (`set_min_size` plus
  `expand_to_include_rect`); a default egui Modal Area would shrink to the
  widgets and leave an empty overlay. Wheel zoom, Plus/Minus, and pan still
  apply to photos. Stickers and GIFs keep the click on the message row.
  Overlay order is viewer, then picker, then dialogs.
  While messages are picked, the composer is replaced by a selection bar
  (star, download, forward, delete-for-me).
  `ui/pane.rs` is the right inspector (`RightPane`). Search is the first use:
  one pane at a time, width persisted as `settings.inspector_width`. Header
  Search and Ctrl+G open it; Ctrl+F stays on the left list. The day filter is
  an `Area` popup under the calendar icon, centered on it. Closing the chat
  closes the pane.
  Pinned messages sit under the chat header as one plain line each (`Pin` icon
  plus the message text, one line, ellipsis), so the banner grows a row per
  pin instead of wrapping a chip; a click opens the pin of the band it hits.
- **`src/app.rs`** — Applies actions after each frame. Opening a chat always
  sends `LoadChat` for the newest archive page. A verified GitHub update
  in `.whatsfast-pending` is adopted here on process start (not in demos).
  Quit while the payload is Ready starts the helper; closing to the tray does
  not. Ctrl+V image paste reads `Event::Paste` or V-release (egui-winit drops
  the press). A short Ctrl chord covers V-up after Ctrl-up. A picture already
  staged in the composer is not attached again.
- **`src/updates/`** — GitHub latest, SHA-256, helper `--apply-update`. Staging
  is `.whatsfast-pending` beside the install, with `prepared.json`. Click
  **Update** runs the helper now. The next process start does the same when
  auto-download is on.
- **`src/backend.rs` / `src/backend/worker.rs`** — Tokio worker thread; owns
  whatsapp-rust `Bot`, archive, downloads, profile pictures. Talks to UI via
  `Command` and `Event`. Ingest and history peel nested `device_sent` /
  ephemeral / view-once wrappers before `classify()`, so copies sent from
  the phone are stored. Serial `Command::Forward` keeps a queue and starts
  the next send only after `Command::Sent` writes `Delivery::Sent` (first
  tick) or `Failed`; parallel batches still use `forward_batch`. Background
  history prefetch asks the phone one page at a time and downloads files
  one at a time into the archive; it does not prepend those pages into the
  open conversation. A failed file is retried with backoff for 30 days.
  `Client::pin_message` / `unpin_message` (`PinDuration::Days7`) write the
  archive only after the server accepts. Incoming `pin_in_chat_message` is
  filed the same way.
  `Command::StorageStats` runs one `json_extract` aggregate on the archive;
  `Event::StorageStats` caches counts and `media.size` sums for Settings.
  Weight is the persisted WhatsApp size of rows with a local path, not a
  disk scan. GIFs (`Content::Video { gif: true }`) share the sticker bucket.
  Leaving a group uses `Client::groups().leave`. The chat row stays; `read_only`
  is set and `me` is dropped from `participants`. Archive is optional.
  Leaving a channel (`@newsletter`) uses `Client::newsletter().leave`. Lists
  (`@broadcast`) have no leave path.
  Unlinking (from the phone or Settings) announces `LinkStatus::LoggedOut`
  before the teardown, deletes the session store with retries (Windows keeps
  that file open until the outgoing client's connection pool closes), and
  starts a fresh client. A start whose store carries no identity reports
  pairing (`LinkStatus::Unlinked`, then the QR) rather than `Connecting`, so
  the window leaves the chat shell for the login screen and never paints a
  linked-looking shell with nothing in it. `App::reset_session` drops the
  account-scoped state at the same moment.
  Connect loads account privacy with `fetch_privacy_settings` and MEX
  `get_privacy_lists`. Settings Privacy writes `set_privacy_setting` and
  `set_privacy_disallowed_list` (a 409 refetches the list hash once). Values
  stay on the phone, not in `settings.json`. The same section also paints
  this-copy **Send read receipts** and **Show when you are typing**, which
  stay in `settings.json` and do not need the link.
- **`src/privacy.rs`** — Account privacy kinds, values, Except lists, and the
  in-memory snapshot the Settings page shows.
- **`src/archive.rs`** — SQLite (SQLCipher) message store; single copy after
  link-time history sync. `put_lid` copies mute/pin and refiles message
  rows from `{lid}@lid` onto `{pn}@s.whatsapp.net`. `chats.marked_unread` is a local empty-dot reminder;
  real `unread` counts still come from the phone. Opening the chat or a new
  incoming message clears the flag. `chats.favorite` and the `chat_lists` /
  `chat_list_members` / `chat_list_pins` tables are local list filters. Pins
  in All stay on `chats.pinned` and still sync with the phone. Pins on any
  other chip live only in `chat_list_pins`. `storage_stats()` sums downloaded
  attachment sizes by JSON `kind` without loading message bodies.
  `message_pins` stores in-chat pins (`PIN_FOR_ALL` / `UNPIN_FOR_ALL`) with
  `expires_at`; at most three active pins per chat. `App::pins` feeds the
  bubble menu, the media viewer, and the same footer mark as a star. A live
  or history `REVOKE` writes `messages.revoked_at` and leaves `content`.
  Local Delete for everyone still stores `Content::Revoked`. `chat_media()` lists
  Image and non-GIF Video rows, and Document rows that are those files, for
  the viewer strip.
  Composer drafts live in `drafts` (chat, text, mentions, reply_to). The UI
  writes `Command::SetDraft` at most every 300 ms and on leave; `Event::Drafts`
  restores them at start. `put_lid` moves the row onto the phone-number chat.
  The chat list shows `Draft:` only for a chat that is not open.
- **`src/model.rs`** — App types; worker translates protobuf in `classify()`.
- **`src/i18n/`** — Every interface string is a `Key` in an enum with an
  English and a Spanish table (`key.rs`, `en.rs`, `es.rs`); `t`, `f`, and
  `count` read the locale from a process-wide atomic. `Language` is the
  persisted preference (`System`, `English`, `Spanish`); `System` asks the
  operating system once and never reaches the tables. `main` resolves the
  language from settings before the first frame, and Settings changes it live.
  The tray item and the macOS menus take their language when they are built,
  so a change shows there on restart. Tables and call-site replacements come
  from `AGENTS/i18n-strings.json` through `AGENTS/i18n-gen.py`;
  `AGENTS/i18n-residual.py` lists English text still painted, and
  `tests/i18n_es.rs` renders every demo page in Spanish and fails on
  leftovers. Protocol bytes, SQL, log lines, ids, and demo sample data stay
  in English.

See root `AGENTS.md` for invariants (privacy, polls, receipts, selection, updates).

## Platform scope

Supported product: **Windows desktop** releases. Verification runs **locally**
only (see `AGENTS.md` *Definition of done*). There are no GitHub Actions
workflows. Non-Windows code may remain temporarily but is not supported or released.
