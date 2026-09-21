# Missing chat messages (phone copies)

Date: 2026-09-20.

Repair archive ingest and UI reload so messages WhatsApp already delivered appear in the open chat. Do not add features. Do not change send, revoke, or paging size.

## Findings

| Id | Sev. | Evidence | Why | Risk if left |
|----|------|----------|-----|--------------|
| M1 | critica | `proto_helpers.rs` 224–261; `worker.rs` `classify` 6274; `ingest` 1848 | `get_base_message` peels `device_sent` only when it is the outer wrapper. Phone copies often nest `ephemeral_message { device_sent_message { body } }`. One peel leaves `device_sent` set. `classify` returns `None`. Live and history drop the row. Matches "own messages sent from the phone". | Those bubbles never enter `archive.db`. |
| M2 | critica | `put_lid` `archive.rs` 700–723; `emit_chats` `worker.rs` 617 | Mapping copies mute/pin only. Messages stay on `{lid}@lid`. The list keeps a mapped lid row only when `last` is set, so the named phone-id chat is missing those rows. | Split thread: user opens the contact and does not see the lid-filed copies. |
| M3 | alta | `ensure_loaded` `app.rs` 1636–1644; `apply_history` 2586–2660 | First `LoadChat` sets `requested`. History files rows and `emit_chat` / `emit_chats` without a first-page `Event::Messages`. Re-opening the same chat does not reload. | Archive has the messages; the open thread does not. |
| M4 | alta | `Event::Messages` `app.rs` 1211–1216; `store_message` 2230–2234 | `complete` is set on the first page (`was_empty`) or on `older`. A later `LoadChat` with more than one page keeps `complete == true`. Scroll-up asks the phone, not the extra archive rows. | Gaps older than the first 60 stay hidden. |
| M5 | media | `parse_conversation` `worker.rs` 6820 vs `seconds` 5855 | History timestamps skip `seconds()`. Values above `100_000_000_000` sort into the newest page and push real seconds off it. Live ingest uses `DateTime::timestamp()` (seconds). | Newest page is the wrong slice. Identity for normal Unix seconds. |

## Out of scope

- `album_message` → `None` (child image/video rows still ingest).
- Event schema, `PAGE` (60), send path, revoke, prefetch-into-UI.
- Reading `archive.db` bodies (privacy).
- Other agents' dirty files.

## Steps

### 1. Backup (P0)

1.1 Copy `src/backend/worker.rs`, `src/archive.rs`, `src/app.rs`, `ARCHITECTURE.md`, `README.md`, `DECISIONS.md` to `AGENTS/Backups/missing-messages-20260920/`.

### 2. Peel nested wrappers (P0, M1)

2.1 In `worker.rs` next to `seconds`, add `visible_base(message: &wa::Message) -> &wa::Message`. Loop `get_base_message` until the pointer is stable, max 8. Why: same wrappers the library already peels, repeated for nest order. Observable: bodies that were dropped now classify as they would if the wrapper order matched the library. Protocol shells still `None`.

2.2 Call `visible_base` in `ingest`, `parse_conversation`, `backfill`, `quoted_of`, `download`, `media_name`. Leave `prepare_for_forward` and `message_secret_from_raw` on a single peel.

2.3 Test: `ephemeral_message { device_sent_message { text } }`. `classify(get_base_message(_))` is `None`. `classify(visible_base(_))` is `Content::text`. `parse_conversation` keeps the id.

### 3. History timestamps (P0, M5)

3.1 `parse_conversation`: `timestamp = seconds(info.message_timestamp.unwrap_or(0) as i64)`. `last_activity` uses `seconds` on conversation timestamps before `max(newest)`. Unix seconds stay unchanged.

3.2 Test: `message_timestamp: 1_700_000_000_000` → stored `1_700_000_000`.

### 4. Refile lid messages (P0, M2)

4.1 `Archive::refile_lid(lid, pn)`: `{lid}@lid` → `{pn}@s.whatsapp.net` for `messages`, `stars`, `message_pins`, `polls`, `poll_history`, `poll_votes`, `group_receipts`, `scheduled`, `chat_list_members`, `chat_list_pins`. Skip ids that already exist on the canonical chat. Delete leftover lid rows and the lid chat. Refresh `chats.last_activity` on the phone id.

4.2 `put_lid` calls `refile_lid` after mute/pin copy. Return true if prefs or rows moved.

4.3 `Worker::load_state` refiles every stored mapping before `emit_chats` (repairs archives that already know the pair).

4.4 Test: insert from-me row on `PEER_LID`, `put_lid`, row is on `PEER`, lid chat gone. Duplicate id on both keeps the canonical copy.

### 5. Reload the open chat (P0, M3, M4)

5.1 `ensure_loaded`: always `Command::LoadChat { before: None }`. Still set `requested`. `Event::Chats` after history then reloads the open thread. Re-opening a chat reads the archive again. Why: intended "show archived messages", currently skipped.

5.2 `Event::Messages` when `!older`: set `complete` if `was_empty` **or** `complete` **or** `messages.len() != 1`. Live `store_message` sends one row with `complete: false` and must not clear paging. A 60-row first page with `complete: false` must clear it.

5.3 Test: `requested` conversation, `open_chat` records another `LoadChat`. First page `complete: true`, then 60-row page `complete: false` updates the flag; a later one-row live event leaves it.

### 6. Docs (P2)

6.1 `ARCHITECTURE.md`: peel loop; `put_lid` moves message rows; opening a chat reloads the newest archive page.

6.2 `README.md` Conversations: phone-sent copies show in the same chat.

6.3 `DECISIONS.md`: nested peel; lid refile; first-page reload.

### 7. Verify

`cargo fmt --all --check`, both clippy lines, both test lines, `cargo doc`. Skip extra debug `cargo build` while `cargo watch` owns the exe. Jev after tests. Version bump, atomic commits, push.
