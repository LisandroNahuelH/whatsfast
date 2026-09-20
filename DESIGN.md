---
icon: packaging/icons/whatsfast-1024.png
---

# WhatsFast design

WhatsFast uses a **green chat bubble** as its app icon. The product is **unofficial** and not affiliated with WhatsApp or Meta.

Source artwork: [packaging/icons/whatsfast-1024.png](packaging/icons/whatsfast-1024.png).

Interface text is Montserrat Variable (`wght` 100–900). Size and weight pick the face; there is no `wdth` axis. Scale: display 28/700 (login brand), page 24/700 (Settings), heading 20/700 (chat-list headers), section 17/700, chat name 14.5/500, body 14/400, meta 12.5, caption 11.5. Reason: one variable file covers titles and body without a second family.

The empty message composer is 35% taller than one text line, the caret fills that band, and typed letters sit on the vertical center of that band with attach, emoji, send, and schedule. The bubble keeps 20% more space above the window's bottom edge than the old 8px inset. Reason: a tight one-line field against the window reads cheap; extra air does not change send keys.

Chat wallpaper names in Settings are Auto or `Black 1` to `White 3`. Hover a name to preview it in the empty Settings margin. Right-click the thread for Next wallpaper. Reason: one doodle per colour felt static; the preview uses spare width instead of covering the combo.

The chat side of the sidebar is a 1-physical-pixel hairline, faint white on dark palettes and faint black on light. Reason: panel `outline` matches the sidebar and the CentralPanel used to cover a line painted on the panel edge.

The chat header Search (Ctrl+G) opens a right inspector for messages in the open chat. Ctrl+F still searches the left list. A hit pulses the full message row three times, not only the bubble. Double-click a bubble or its row to reply; the row flashes once with the same wash as pick. A click on a sticker or GIF uses that same row, not the system editor. The day-filter calendar is a popup under the calendar icon, centered on that icon, and a click elsewhere closes it. Reason: WhatsApp Desktop puts the lupa next to More; in-chat hits need their own pane so they do not mix with the chat list.

Check for updates and Download updates automatically sit in Settings **About**, under the version row. Reason: they describe this build, not the window.

Hover a name in the **Download older history** list to show a short card on the right of that row. The open-chat choice is **Current Chat**. Reason: the three modes look alike until you read what each one fetches.

Settings **Downloads** paints attachment weight as 6 px accent bars on a dim track. Reason: a chart crate would add weight for three fractions the painter already draws.

**Settings → Language** is its own section with the same row family as Theme; the combo lists **System language**, **English**, and **Español** as endonyms. Reason: a picker that translates its own options strands the reader who chose the wrong one, and the system option must stay findable in either language.

**Send read receipts** and **Show when you are typing** sit in Settings **Privacy**, above the account rows. Reason: they control what others see of you, not chat chrome.

The media viewer is a full-window overlay: a 56 px header (avatar, name, last seen, zoom, go-to-message, reply, star, pin, react, forward, download, close), the photo or video in the centre, and a full-width filmstrip of that chat's images and videos. Reason: WhatsApp Web keeps actions and the rest of the album in reach; the gallery is the archive, not the loaded page.

A message pin is not a chat pin. Pin for everyone for 7 days, at most three per chat, with a left-list copy of Starred, chips under the chat header, and the same footer mark a star uses. A click on the chip or on the rest of that header row jumps to the message. Reason: WhatsApp Web's pin-in-chat is a protocol act, and the row itself must show the pin.

The bubble footer uses one 8 px gap from the time to the nearest mark and between pin and star. Reason: a 16 px slot with a 13 px icon left about 1.5 px of air and the two marks read as one blob.

A voice bubble keeps play and waveform on one 36 px row, centred together. Duration sits on the footer row at the waveform's left edge, beside the clock. Reason: stacking duration under the bars stretched the row, sat the button high, and Label padding shifted the time off the first bar.
