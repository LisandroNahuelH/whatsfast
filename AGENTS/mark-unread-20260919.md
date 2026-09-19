# Mark as unread (sidebar context menu)

Date: 2026-09-19

Add **Mark as unread** on the left chat-list context menu. The chat looks unread with an empty round badge (no number). Real unread counts and **Mark as read** stay as they are.

## Phase 1–2 findings

| id | sev | evidence | why | risk if skipped |
|----|-----|----------|-----|-----------------|
| F1 | alta | `src/ui/chats.rs` ~1486: menu only has Mark as read when `unread > 0` | No way to flag a read chat | Feature missing |
| F2 | alta | `widgets::badge` always paints a digit | Faking `unread = 1` would show "1" | Wrong vs WhatsApp empty dot |
| F3 | media | Compact rail (`compact_list`) has no context menu | User said any left-sidebar chat | Compact would lack the action |
| F4 | baja | Compact rail paints no unread mark today | Do not add count badges there | Would change existing compact look |

No over-work, loop, resource, or race findings with evidence.

## Do not touch

- Real `Chat.unread` counts, `bump_unread`, history snapshot unread, receipts.
- Conversation jump-to-bottom numbered badge (`conversation.rs`).
- Compact numbered badges (none today; leave it).
- Protocol `mark_chat_as_read`; local archive flag only.
- `src/archive/encryption.rs` and unrelated dirty files.

## P0 persist + clear

### 1.1 `Chat.marked_unread`

- File: `src/model.rs` `Chat` + `Chat::new`.
- Add `pub marked_unread: bool` (default false).
- Add `looks_unread()`: `unread > 0 || marked_unread`.
- Observable unread counts unchanged.

Verify: `Chat::new` tests still compile; new unit check on `looks_unread`.

### 1.2 Archive column

- `MIGRATIONS`: `("chats", "marked_unread", "INTEGER NOT NULL DEFAULT 0")`.
- `CHAT_COLUMNS` append `c.marked_unread`; `chat_from_row` index 17.
- `mark_read`: also `marked_unread = 0`.
- `bump_unread`: also `marked_unread = 0`.
- `set_unread`: `marked_unread = 0` only when `unread > 0` (keep flag if phone reports 0).
- `set_marked_unread(id, bool)`.
- Do not change `mark_read_through` (history unread=0 would wipe the local flag).

Verify: in-memory test persist / clear / keep on `set_unread(0)`.

### 1.3 Command + worker

- `Command::SetMarkedUnread { chat, marked }`.
- Worker: archive write + `emit_chat`. No phone call.

Verify: match is exhaustive (compile).

### 1.4 App

- `Action::MarkUnread(ChatId)`.
- `mark_unread`: set flag if `unread == 0`; send command.
- `mark_read`: also `marked_unread = false` locally.
- `open_chat`: call `mark_read` when `unread > 0 || marked_unread`.
- Do not auto-clear `marked_unread` in `handle_chat_updated` while focused (only real `unread > 0`).

Verify: headless test mark then open.

## P1 UI

### 2.1 Empty badge

- `widgets::unread_dot`: 10px filled circle, accent or dim if muted; return width 10.
- `widgets::unread_indicator(count, marked, muted)`: count > 0 → existing `badge`; else marked → dot; else 0.
- Chat row: `unread = chat.looks_unread()` for bold/preview; indicator uses count + flag.

Verify: numbered path still calls `badge` with the real count.

### 2.2 Context menu

- If `unread > 0 || marked_unread` → Mark as read (existing action).
- Else → Mark as unread (`Action::MarkUnread`), icon `MessageCircle`.
- Compact list: same `Popup::context_menu`. Compact paints the empty dot only when `marked_unread && unread == 0`.

Verify: menu conditions exclusive; compact unread counts still have no number badge.

## P2 demo + tests + docs

### 3.1 Demo

- Grace Hopper sample: `marked_unread = true` after `Chat::new`.

### 3.2 Docs

- `README.md` user-visible item.
- `ARCHITECTURE.md` one structural line (local flag, empty badge).
- `DECISIONS.md` row: local flag, not protocol; empty circle vs count.

## Verify (phase 5)

- Diff vs this plan only.
- Full definition-of-done checks + debug `cargo build --locked`.
- Verdict needs evidence per change.
