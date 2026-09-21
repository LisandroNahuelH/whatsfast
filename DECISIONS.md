# Decision log

Append-only, newest first.

| Date | Decision | Why | Replaces |
|------|----------|-----|----------|
| 2026-09-20 | Bubble-footer pin mark is black | Dark green `#166534` blended into outgoing bubbles | Filled pin in dark green |
| 2026-09-20 | Incoming REVOKE stores `revoked_at` and keeps `content` | Recover the body on this copy; Settings can still paint the deleted placeholder | `set_content(Revoked)`, which destroyed the JSON body |
| 2026-09-20 | Selection-bar Delete is delete-for-me for every picked row | Mixed incoming and outgoing picks cannot all revoke; the bubble menu still has both items | No batch delete on the bar |
| 2026-09-20 | Live `cargo watch` uses `-d 30` | Agents edit many files; 15 s still restarted compile mid-turn | `-d 15` after last crate-input change |
| 2026-09-20 | Ctrl+V image paste uses Ctrl chord memory and skips a duplicate pending picture | egui-winit drops V-press; Windows often reports V-up after Ctrl-up; a time debounce still cloned the same clip | Retries, `CF_DIB`, and a 0.7 s paste timer |
| 2026-09-20 | `names_from_contacts` on prefers `push_name` | The switch showed public profile names when on and saved labels when off | On preferred `full_name`, off preferred `push_name` |
| 2026-09-20 | Pin-banner `OpenMessage` uses the whole header row | The chip was a 24 px pill inside a 36 px bar; padding and empty strip did not open the pin | Click only on the chip rect |
| 2026-09-20 | Voice duration paints on the footer at the waveform's left edge | A vertical under the bars stretched the player row and Label padding sat the time off the first bar | Duration under the waveform in `ui.vertical` |
| 2026-09-20 | UI face is Montserrat Variable (`wght` 100–900), including Monospace | Match the PutMeInThatState product face; official Montserrat has no `wdth` | Inter Variable on Proportional, Hack on Monospace |
| 2026-09-20 | A pinned message paints `Icon::Pin` in the bubble footer | Stars already mark the row; pins only lived on chips and the menu | Footer reserved star room only |
| 2026-09-20 | Viewer sizes header, stage, and strip from window `content_rect` | A default egui Modal Area shrinks to allocated widgets after frame 0 | Custom Modal `.area()` plus `ui.max_rect()` |
| 2026-09-20 | In-chat pins use `Client::pin_message` for 7 days, cap 3 per chat | WhatsApp Web default; no duration menu in this cut | Dropping `pin_in_chat_message` (`return None`) |
| 2026-09-20 | Viewer gallery lists Image and non-GIF Video from SQLite | The loaded page is not the whole album | `neighbor_image` on in-memory photos with a path |
| 2026-09-20 | Local send-receipts and typing switches live in Settings Privacy | They describe what this copy discloses, not chat chrome | The same two switches in Settings Chats |
| 2026-09-20 | Prefetch hover card sits to the right of the hovered list row | A card at the top of Settings did not point at the mode under the pointer | Card in the empty Settings margin (`settings_hover_slot`) |
| 2026-09-19 | Interface text lives in `src/i18n` key tables, never in a paint call | The compiler then forces a value for every locale, and the residual scan can name a string left in English | A runtime string map or gettext |
| 2026-09-19 | The interface language is read at startup and from Settings, not rebuilt live | Tray and macOS menus are handed their labels when they are created | Repainting native menus on every settings change |
| 2026-09-19 | Only English and Spanish, behind a `Language` enum | An enum keeps both tables exhaustive and the system option is resolved once | Loading locale files at runtime |
| 2026-09-19 | Live watch runs `cargo build` before `taskkill` | Killing first left the window closed when compile was interrupted | `taskkill` then `cargo run` |
| 2026-09-19 | Settings Downloads stats come from one archive SQL aggregate | Walking `cache/media` on the UI thread would stall; `media.size` is already in each row | Disk walk or a chart crate |
| 2026-09-19 | Live `cargo watch` uses `-d 15` | Instant restart froze the PC and never wrote the debug exe | Watch crate paths with the default 0.5 s delay |
| 2026-09-19 | Live `cargo watch` only watches `src`, `Cargo.toml`, `build.rs`, and `assets` | Markdown and `AGENTS/` restarts aborted the debug link before `whatsfast.exe` existed | Watch the whole tree, then `taskkill` and `cargo run` |
| 2026-09-19 | Search day-filter calendar is a popup under the icon; a click elsewhere closes it | An inline month stole the pane and never closed on an outside click | Calendar filled the inspector until Escape, the icon, or a day |
| 2026-09-19 | Click on a sticker or GIF stays on the message row | Opening the system editor from that click was useless; reply, menu, and select must match a text bubble | Chat photos open in an in-app viewer (system handler on stickers) |
| 2026-09-19 | Double-click a message bubble or its row replies, with one row flash | The documented empty-strip double-click was gone; a flash confirms the quote | Double-click beside a message, or on its edge, replies; text keeps the word |
| 2026-09-19 | OpenMessage pulse washes the full message row | The bubble is only part of the line; search must mark the whole row | Bubble fill lerp toward accent |
| 2026-09-19 | Header Search and Ctrl+G open a right inspector for in-chat search | Ctrl+F keeps the chat list; a second finder in the same field mixed two jobs | Open-chat header Search runs FocusSearch |
| 2026-09-19 | Open-chat header Search runs FocusSearch | Same field as Ctrl+F; a second in-chat finder would split results | Header had More only |
| 2026-09-19 | Channel unfollow reuses LeaveGroup; copy and newsletter().leave differ | Same three surfaces as groups; `@broadcast` lists are not channels | Channels had no leave path |
| 2026-09-19 | A verified pending update installs on the next process start when auto-download is on | Toast click stays immediate; closing to the tray is not a restart | In-memory `Prepared` only |
| 2026-09-19 | Leave group asks Leave or Leave and archive | One click must not drop you from a group; archive is a second, optional hide | No leave path |
| 2026-09-19 | One-line composer letters use `Align::Center` in the tall first row | Default galley valign is bottom, so typed text sat on the floor of the taller bubble | `TextFormat::simple` default `Align::BOTTOM` in the stretched row |
| 2026-09-19 | Composer caret fills the taller one-line band; row controls are vertically centered | Stretching the box left a short caret and bottom-aligned send/schedule | Bottom-aligned row, font-sized caret in a taller field |
| 2026-09-19 | History prefetch names the open chat Current Chat and shows a hover card | This chat was vague; the card matches the wallpaper preview slot | Label This chat, no hover hint |
| 2026-09-19 | Update toggles live in Settings About | They describe this build, not the window | Window section held Check for updates and Download updates automatically |
| 2026-09-19 | Update check is once a day after the installer path is proven | Ten minutes is only for a local test; production should not poll GitHub that often | 10-minute `CHECK_INTERVAL` |
| 2026-09-19 | Update check is every 10 minutes while we prove the installer path | A daily interval hides a failed GitHub download until the next day | 24-hour `CHECK_INTERVAL` |
| 2026-09-19 | Sidebar split is a 1-physical-pixel hairline on the chat panel | `outline` on the panel edge vanished under CentralPanel and matched the sidebar fill | 1px `palette.outline` vline on the SidePanel |
| 2026-09-19 | Each wallpaper family has three doodles; Next wallpaper steps the slot | One PNG per colour felt static; Auto keeps the family while the index wraps | One PNG per family, family-only Settings names |
| 2026-09-19 | Empty composer is 35% taller, first line centered, 20% more gap under the bubble | A one-line field felt tight against the window; extra height and inset give a calmer typing band without changing send keys | One-line composer flush to an 8px bottom inset |
| 2026-09-19 | README is two layers: ZapFast-adapted catalog, then WhatsFast extras | Upstream describes the app; extras stay the Windows delta. Visual identity stays amber PNG + badges. RAM numbers stay attributed to ZapFast Linux | README extras-only, no catalog |
| 2026-09-19 | Chat doodle wallpaper is Auto by theme, with a Settings override | Native WhatsApp uses a low-contrast doodle behind bubbles; five families match bundled palettes and more PNGs can land in the same folders | Solid `palette.chat` fill only |
| 2026-09-19 | Chat-list chips are local filters; All pins stay the WhatsApp pin | Custom lists and favorites must not pretend to be WhatsApp labels or groups | One global pin list for every sidebar view |
| 2026-09-19 | App version is `0.15.(100 + N)` and every git commit bumps the patch by 1 | Agents left `0.15.1` stuck; the millesimal patch tracks `git rev-list --count HEAD` | Bumping `Cargo.toml` only when cutting a GitHub release |
| 2026-09-19 | Failed attachment prefetch retries with backoff for 30 days | A one-shot skip left holes when CDN links expired; the phone can re-upload later | Permanent `media_skip` until reconnect |
| 2026-09-19 | README hero uses PNG (`app-mark.png`, `hero-banner.png`) | GitHub strips or caches SVG; same URL kept the old 3-dot mark | README `<img>` pointing at `.svg` |
| 2026-09-19 | App mark is the ZapFast message-circle on amber `#E85D04` | Same glyph as upstream, colour split from WhatsApp green | Teal 3-dot `whatsfast.svg` / green ZapFast mark |
| 2026-09-19 | Background history prefetch fills the archive and disk, not the open thread | Laying out every old message would stall the UI; `LoadOlder` pages SQLite after the files are local | Fake slow scroll of the conversation |
| 2026-09-19 | README hero uses GitHub HTML, shields, a banner SVG, and a synthetic `--demo-shot` | Visitors need a visual front door; ZapFast Linux charts are not WhatsFast Windows facts | Plain-text README only |
| 2026-09-19 | Mark as unread is a local `marked_unread` flag and an empty sidebar dot | Faking `unread = 1` would show a number; the protocol is not used for this reminder | — |
| 2026-09-19 | Serial forward waits for `Delivery::Sent` (first tick) before the next send | `send_message_with_options` returns before the worker writes the tick; `Command::Sent` is that write | Awaiting `send_outgoing` then starting the next job at once |
| 2026-09-19 | End each coding turn with `cargo build --locked` (debug) | Incremental rebuild so `target/debug/whatsfast.exe` is ready to try at once | Waiting for `--release` or a later `cargo run` that may only surface the installer copy |
| 2026-09-19 | One-click update toast; download auto ON by default | Wizard users update with one click; last chat already restores | Modal Update WhatsFast + explicit Restart |
| 2026-09-19 | Chat photos open in an in-app viewer | Click must show the photo here, with zoom and pan. The system viewer stays on **Open file**, stickers, and video | `Action::OpenFile` on photo click |
| 2026-09-19 | No GitHub Actions; Actions disabled in repo settings | Solo dev on PC; GH CI only slowed pushes | CI + Release workflows |
| 2026-09-19 | Branch protection removed | No required checks without Actions | protection on `main` |
| 2026-09-19 | Removed crmne Actions (triage, docs Pages, flatpak, packaging) | WhatsFast is not the upstream project; CI = one Windows job + Release on tags | — |
| 2026-09-19 | Post-verify fixes: default branch main, no Release→packaging job | GitHub showed inaugural branch; Actions queue blocked Release | — |
| 2026-09-19 | Jev gate skipped on G1–G9 integration PRs | quota/time; full checks run on Windows main before tag | — |
| 2026-09-19 | whatsfast.ico still ZapFast pixels until PNG→ICO tool run | SHA match legacy; PNG 1024 ships in repo | — |
| 2026-09-19 | WhatsFast fork from crmne/zapfast | portfolio + Windows scope | — |
| 2026-09-19 | Windows-only product | no multi-OS releases | — |
| 2026-09-19 | fork allow-forking N/A on personal repo | — | — |
| 2026-09-19 | main recovery force-push | unrelated histories | — |
