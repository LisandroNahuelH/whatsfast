//! en strings.

use super::Key;

pub(super) fn text(key: Key) -> &'static str {
    match key {
        Key::SettingsThemeDark => "Dark",
        Key::SettingsThemeLight => "Light",
        Key::SettingsThemeSystem => "Follow system",
        Key::SettingsHistoryOff => "Off",
        Key::SettingsHistoryCurrent => "Current Chat",
        Key::SettingsHistoryRecent => "Recent and pinned",
        Key::SettingsHistoryOffHint => "Do not fetch older messages in the background.",
        Key::SettingsHistoryCurrentHint => "Fetch older messages and files for the open chat only.",
        Key::SettingsHistoryRecentHint => {
            "Fetch older history for pinned chats and the ten most recent chats."
        }
        Key::SettingsWallpaperBlack => "Black",
        Key::SettingsWallpaperGray => "Gray",
        Key::SettingsWallpaperGreen => "Green",
        Key::SettingsWallpaperRed => "Red",
        Key::SettingsWallpaperWhite => "White",
        Key::SettingsWallpaperAuto => "Auto",
        Key::SettingsSectionLanguage => "Language",
        Key::SettingsLanguageSystem => "System language",
        Key::SettingsLanguageEnglish => "English",
        Key::SettingsLanguageSpanish => "Español",
        Key::SettingsLanguageHint => {
            "Applies to the whole interface. Menus in the system tray change on restart."
        }
        Key::DateWeekdayMonday => "Monday",
        Key::DateWeekdayTuesday => "Tuesday",
        Key::DateWeekdayWednesday => "Wednesday",
        Key::DateWeekdayThursday => "Thursday",
        Key::DateWeekdayFriday => "Friday",
        Key::DateWeekdaySaturday => "Saturday",
        Key::DateWeekdaySunday => "Sunday",
        Key::DateWeekdayAbbrMonday => "Mon",
        Key::DateWeekdayAbbrTuesday => "Tue",
        Key::DateWeekdayAbbrWednesday => "Wed",
        Key::DateWeekdayAbbrThursday => "Thu",
        Key::DateWeekdayAbbrFriday => "Fri",
        Key::DateWeekdayAbbrSaturday => "Sat",
        Key::DateWeekdayAbbrSunday => "Sun",
        Key::DateMonthJanuary => "January",
        Key::DateMonthFebruary => "February",
        Key::DateMonthMarch => "March",
        Key::DateMonthApril => "April",
        Key::DateMonthMay => "May",
        Key::DateMonthJune => "June",
        Key::DateMonthJuly => "July",
        Key::DateMonthAugust => "August",
        Key::DateMonthSeptember => "September",
        Key::DateMonthOctober => "October",
        Key::DateMonthNovember => "November",
        Key::DateMonthDecember => "December",
        Key::DateMonthAbbrJanuary => "Jan",
        Key::DateMonthAbbrFebruary => "Feb",
        Key::DateMonthAbbrMarch => "Mar",
        Key::DateMonthAbbrApril => "Apr",
        Key::DateMonthAbbrMay => "May",
        Key::DateMonthAbbrJune => "Jun",
        Key::DateMonthAbbrJuly => "Jul",
        Key::DateMonthAbbrAugust => "Aug",
        Key::DateMonthAbbrSeptember => "Sep",
        Key::DateMonthAbbrOctober => "Oct",
        Key::DateMonthAbbrNovember => "Nov",
        Key::DateMonthAbbrDecember => "Dec",
        Key::DateToday => "Today",
        Key::DateYesterday => "Yesterday",
        Key::DateYesterdayAt => "Yesterday at {time}",
        Key::DateAt => "{moment} at {time}",
        Key::DateShort => "{day} {month} {year}",
        Key::DateLong => "{weekday}, {day} {month} {year}",
        Key::DateStampShort => "{weekday} {day} {month}, {time}",
        Key::DateShortWeekday => "{weekday} {day} {month}",
        Key::MonthYear => "{month} {year}",
        Key::CopyStamp => "{time}, {month}/{day}/{year}",
        Key::ScheduleOnce => "Once",
        Key::ScheduleEveryDay => "Every day",
        Key::ScheduleEveryWeekday => "Every {weekday}",
        Key::ScheduleDayOfMonth => "Day {day} of every month",
        Key::ScheduleNthWeekdayOfMonth => "{ordinal} {weekday} of every month",
        Key::ScheduleOrdinal1 => "1st",
        Key::ScheduleOrdinal2 => "2nd",
        Key::ScheduleOrdinal3 => "3rd",
        Key::ScheduleOrdinal4 => "4th",
        Key::ScheduleTitle => "Schedule message",
        Key::ScheduleTimeLabel => "Time",
        Key::ScheduleRepeatLabel => "Repeat",
        Key::ScheduleEarlierHour => "Earlier",
        Key::ScheduleLaterHour => "Later",
        Key::ScheduleEarlierMinutes => "Earlier minutes",
        Key::ScheduleLaterMinutes => "Later minutes",
        Key::ScheduleSend => "Schedule",
        Key::SchedulePreviousMonth => "Previous month",
        Key::ScheduleNextMonth => "Next month",
        Key::ScheduleNeedMessage => "Write the message first; it goes out at the time you pick.",
        Key::ScheduleFrom => "{repeat}, from {when}",
        Key::DialogCancel => "Cancel",
        Key::CommonBack => "Back (Esc)",
        Key::CommonOpen => "Open",
        Key::CommonQuit => "Quit",
        Key::CommonShortcuts => "Shortcuts",
        Key::SettingsTitle => "Settings",
        Key::SettingsSectionAppearance => "Appearance",
        Key::SettingsThemeLabel => "Theme",
        Key::SettingsThemeOmarchyHint => "Follow system uses your Omarchy colours.",
        Key::SettingsThemeSystemHint => {
            "Follow system uses your desktop's light or dark appearance."
        }
        Key::SettingsOpenThemesFolder => "Open themes folder",
        Key::SettingsWallpaperLabel => "Chat wallpaper",
        Key::SettingsWallpaperHint => {
            "Auto picks a doodle from the theme colours. You can force Black 1 to White 3. Right-click the open chat and choose Next wallpaper to step through the three doodles for that family."
        }
        Key::SettingsZoomLabel => "Zoom",
        Key::SettingsZoomHint => "You can also use Ctrl+plus and Ctrl+minus.",
        Key::SettingsZoomLarger => "Larger",
        Key::SettingsZoomSmaller => "Smaller",
        Key::SettingsSectionChats => "Chats",
        Key::SettingsEnterSends => "Enter sends",
        Key::SettingsEnterSendsHint => "When off, Enter adds a line and Ctrl+Enter sends.",
        Key::SettingsReceiptsOffNote => {
            "Read receipts are disabled for your WhatsApp account (Settings Privacy). Direct chats will not send them. When this switch is on, groups still do. Read state syncs between your devices either way."
        }
        Key::SettingsReceiptsOnNote => {
            "Let people see when you read messages or play voice messages on this copy. Your WhatsApp account setting in Privacy still applies. Read state syncs between your devices either way."
        }
        Key::SettingsSendReceipts => "Send read receipts",
        Key::SettingsSendTyping => "Show when you are typing",
        Key::SettingsAutoDownload => "Download attachments automatically",
        Key::SettingsAutoDownloadHint => {
            "Download pictures, videos, voice messages, and documents up to 64 MB when they enter view. When off, click a file to download it."
        }
        Key::SettingsHistoryLabel => "Download older history in the background",
        Key::SettingsHistoryHint => {
            "Slowly fetch older messages and their files (up to 64 MB) so scrolling up does not hit WhatsApp's rate limit. Failed files are asked again with a long wait, for up to 30 days. Recent and pinned covers every pinned chat plus the ten most recently active chats that are not pinned."
        }
        Key::SettingsShowSenderPictures => "Show sender pictures in every chat",
        Key::SettingsShowSenderPicturesHint => "WhatsApp shows them in groups only.",
        Key::SettingsNamesFromContacts => "Names from your address book",
        Key::SettingsNamesFromContactsHint => {
            "Prefer saved contact names. When off, prefer public WhatsApp profile names. This applies throughout the app."
        }
        Key::SettingsSaveContacts => "Save contacts to the phone's address book",
        Key::SettingsSaveContactsHint => {
            "Also add contacts saved here to your phone's address book. When off, they remain WhatsApp contacts. Names sync to linked devices either way."
        }
        Key::SettingsForwardInOrder => "Forward messages in order",
        Key::SettingsForwardInOrderHint => {
            "Send a forwarded batch one message at a time, starting each one when the message before it shows its first tick. Mixed text, pictures, and videos then arrive in their original order. When off, they send together and may arrive out of order."
        }
        Key::SettingsShowPollButton => "Show the create poll button",
        Key::SettingsShowPollButtonHint => {
            "Add the Create poll button beside the composer. When off, the button is hidden and the polls already in a chat keep working."
        }
        Key::SettingsShowShortcutHints => "Show shortcut hints",
        Key::SettingsSectionDownloads => "Downloads",
        Key::SettingsStorageHint => {
            "Weight of pictures, videos, stickers, and GIFs already on this computer. The message count includes text."
        }
        Key::SettingsStorageImages => "Images",
        Key::SettingsStorageVideos => "Videos",
        Key::SettingsStorageStickersGifs => "Stickers and GIFs",
        Key::SettingsStorageOther => "Other",
        Key::SettingsStorageMessagesOne => "1 message",
        Key::SettingsStorageMessagesMany => "{count} messages",
        Key::SettingsSectionPrivacy => "Privacy",
        Key::SettingsPrivacyLoadFailed => {
            "Could not load privacy settings. They load again when WhatsFast reconnects."
        }
        Key::SettingsSectionWindow => "Window",
        Key::SettingsKeepRunning => "Keep running when the window closes",
        Key::SettingsKeepRunningHint => {
            "Keep WhatsFast linked in the system tray. Quit from the tray menu or with Ctrl+Q."
        }
        Key::SettingsNotify => "Notify about new messages",
        Key::SettingsNotifyHint => {
            "Show desktop notifications when the window is hidden, in the background, or showing another chat. Muted chats do not notify you."
        }
        Key::SettingsHideSidebar => "Hide the sidebar completely",
        Key::SettingsHideSidebarHint => {
            "On: hiding the sidebar (Ctrl+B) leaves nothing behind. Off: it narrows to the chat pictures, so search, archived chats, and settings stay one click away."
        }
        Key::SettingsGiphyKey => "GIPHY API key",
        Key::SettingsGiphyKeyHintBuiltIn => {
            "Used for GIF search. This build includes a key. Enter a key from developers.giphy.com to replace it."
        }
        Key::SettingsGiphyKeyHintRequired => {
            "Required for GIF search. Get a free key from developers.giphy.com."
        }
        Key::SettingsSectionAccount => "Account",
        Key::SettingsLinkedDevice => "Linked device",
        Key::SettingsUnlink => "Unlink this computer",
        Key::SettingsSectionFiles => "Files",
        Key::SettingsMessageArchive => "Message archive",
        Key::SettingsOpenFolder => "Open folder",
        Key::SettingsDownloadedAttachments => "Downloaded attachments",
        Key::SettingsAskWhereToSave => "Ask where to save each file",
        Key::SettingsAskWhereToSaveHint => {
            "Open the system save dialog for every attachment you save from the selection bar, so you choose the folder and the file name. When off, saved files go to your Downloads folder."
        }
        Key::SettingsLogOfRun => "Log of this run",
        Key::SettingsSectionAbout => "About",
        Key::SettingsAboutLine => {
            "A native WhatsApp client built with Rust, egui, and whatsapp-rust."
        }
        Key::SettingsCheckUpdates => "Check for updates",
        Key::SettingsCheckUpdatesHint => {
            "Ask GitHub once a day whether a newer WhatsFast release exists. The request identifies only WhatsFast and its version."
        }
        Key::SettingsDownloadUpdates => "Download updates automatically",
        Key::SettingsDownloadUpdatesHint => {
            "Download and verify new releases in the background. A toast offers Update; one click installs and restarts. If you skip the toast, the next start of WhatsFast installs the verified file. Native packages and Flatpak update through their package manager."
        }
        Key::SettingsLanguageLabel => "Interface language",
        Key::ShortcutSearchChats => "Search chats",
        Key::ShortcutSearchMessages => "Search messages in the open chat",
        Key::ShortcutFocusComposer => "Focus the message input",
        Key::ShortcutPreviousNextChat => "Previous / next chat",
        Key::ShortcutSend => "Send (Shift+Enter for a new line)",
        Key::ShortcutDismiss => "Dismiss the current action, return from search, or close the chat",
        Key::ShortcutPhotoViewer => "Media viewer",
        Key::ShortcutPhotoViewerKeys => {
            "Wheel zooms, drag moves, ← / → previous or next photo or video"
        }
        Key::ShortcutPaste => "Paste text, or send a picture from the clipboard",
        Key::ShortcutChatList => "Show or hide the chat list",
        Key::ShortcutNewest => "Jump to the newest message",
        Key::ShortcutZoom => "Zoom in / out",
        Key::ShortcutResetZoom => "Reset zoom",
        Key::ShortcutThisList => "This list",
        Key::ShortcutCloseWindow => "Close the window (WhatsFast remains in the tray)",
        Key::LoginTagline => "A native WhatsApp client.",
        Key::LoginConnecting => "Connecting to WhatsApp…",
        Key::LoginLinkedWaiting => "Linked. Waiting for your chats…",
        Key::LoginUnlinkedRequestingCode => {
            "This computer was unlinked from your phone. Requesting a new code."
        }
        Key::LoginRequestingNewCode => "Requesting a new code…",
        Key::LoginTryAgain => "Try again",
        Key::LoginRequestingCodeFor => "Requesting a code for +{phone}…",
        Key::LoginWaitingForCode => "Waiting for a code from WhatsApp…",
        Key::LoginUnofficialWarning => {
            "Unofficial client. Using it may be against WhatsApp's terms of service."
        }
        Key::LoginLinkThisComputer => "Link this computer",
        Key::LoginOpenWhatsApp => "Open WhatsApp on your phone",
        Key::LoginTapMenu => "Tap Menu or Settings, then Linked devices",
        Key::LoginTapLinkDevice => "Tap Link a device and point the phone at this code",
        Key::LoginLinkWithPhone => "Link with phone number instead",
        Key::LoginEnterCode => "Enter this code on your phone",
        Key::LoginForPhone => "for +{phone}",
        Key::LoginTapLinkPhone => "Tap Link a device, then Link with phone number instead",
        Key::LoginCopyCode => "Copy code",
        Key::DropToSendTo => "Drop to send to {name}",
        Key::HistoryLoadingPercent => "Loading chat history… {percent}%",
        Key::HistoryLoading => "Loading chat history…",
        Key::OfflineReconnecting => "Offline ({reason}). Reconnecting…",
        Key::NotLinkedPhone => "Not linked to a phone",
        Key::CommonRetry => "Retry",
        Key::StatusShowChatList => "Show the chat list (Ctrl+B)",
        Key::PollCreateTitle => "Create poll",
        Key::CommonClose => "Close",
        Key::PollQuestion => "Question",
        Key::PollAskQuestion => "Ask a question",
        Key::PollAnswers => "Answers",
        Key::PollAnswerHint => "Answer {number}",
        Key::PollRemoveAnswer => "Remove answer",
        Key::PollAddAnswer => "Add answer",
        Key::PollAllowMultiple => "Allow multiple answers",
        Key::PollSending => "Sending…",
        Key::PollSend => "Send poll",
        Key::PollSelectOne => "Select one answer",
        Key::PollSelectMany => "Select answers",
        Key::PollSendingVote => "Sending vote…",
        Key::PollWaitingPhone => "Waiting for your phone · earlier votes may be missing",
        Key::PollLoadingVotes => "Loading earlier votes from your phone…",
        Key::PollVotesNotLoaded => "Earlier votes have not been loaded yet",
        Key::PollVotingKeyMissing => "Voting key unavailable · use your phone",
        Key::PollReconnectToVote => "Reconnect to vote",
        Key::UpdateNewVersion => "There's a new version available",
        Key::UpdateDownloadFromGitHub => "Download from GitHub",
        Key::UpdateUpdate => "Update",
        Key::ViewerCouldNotDisplay => "Could not display this file.",
        Key::ViewerClose => "Close",
        Key::ViewerPrevious => "Previous",
        Key::ViewerNext => "Next",
        Key::ViewerZoomIn => "Zoom in",
        Key::ViewerZoomOut => "Zoom out",
        Key::ViewerGoToMessage => "Go to message",
        Key::ViewerReact => "React",
        Key::ChatPinMessage => "Pin",
        Key::ChatUnpinMessage => "Unpin",
        Key::ChatListPinned => "Pinned",
        Key::ChatListPinnedMessages => "Pinned messages",
        Key::ChatListNoPinned => "No pinned messages",
        Key::ChatListNoPinnedHint => "Pin a message from the chat or the media viewer.",
        Key::ToastPinned => "Pinned for 7 days",
        Key::ToastUnpinned => "Pin removed",
        Key::ToastPinLimit => "This chat already has 3 pinned messages.",
        Key::ChatOnline => "online",
        Key::PaneSearchTitle => "Search messages",
        Key::PaneFilterByDate => "Filter by date",
        Key::PaneSearchHint => "Search",
        Key::PaneSearchWith => "Search messages with {title}",
        Key::PaneNoMessages => "No messages found",
        Key::PaneTryAnother => "Try another word or pick a different day.",
        Key::PickerTabEmoji => "Emoji",
        Key::PickerTabGif => "GIF",
        Key::PickerTabStickers => "Stickers",
        Key::PickerGroupSmileys => "Smileys & Emotion",
        Key::PickerGroupPeople => "People & Body",
        Key::PickerGroupAnimals => "Animals & Nature",
        Key::PickerGroupFood => "Food & Drink",
        Key::PickerGroupTravel => "Travel & Places",
        Key::PickerGroupActivities => "Activities",
        Key::PickerGroupObjects => "Objects",
        Key::PickerGroupSymbols => "Symbols",
        Key::PickerGroupFlags => "Flags",
        Key::PickerRecent => "Recent",
        Key::PickerFrequentlyUsed => "Frequently Used",
        Key::PickerNothingMatches => "Nothing matches",
        Key::PickerSearchEmoji => "Search emoji",
        Key::PickerSearchGifs => "Search GIFs via GIPHY",
        Key::PickerSearching => "Searching…",
        Key::PickerGifEmpty => "Search for a GIF or browse trending results.",
        Key::PickerGiphyKeyRejected => {
            "This GIPHY API key was rejected. Create a free key at developers.giphy.com and paste it here. It is saved in your settings."
        }
        Key::PickerGiphyKeyNeeded => {
            "GIF search needs a GIPHY API key. Create a free key at developers.giphy.com and paste it here. It is saved in your settings."
        }
        Key::PickerStickersLoading => "Loading your stickers…",
        Key::PickerStickersEmpty => {
            "Recent stickers appear here. Right-click one to save it. To import a pack, paste a signal.art link or open a .wastickers file."
        }
        Key::PickerStickersSaved => "Saved",
        Key::PickerStickerRemovePack => "Remove this pack",
        Key::PickerFindPacks => "Find packs",
        Key::CommonOpenFile => "Open file",
        Key::PickerPasteLink => "Paste a signal.art link",
        Key::PickerBrowseStickers => "Browse signalstickers.org",
        Key::PickerImportingPack => "Importing the pack…",
        Key::PickerRemoveFromSaved => "Remove from saved",
        Key::PickerSaveSticker => "Save sticker",
        Key::DialogForwardOne => "Forward message",
        Key::DialogForwardMany => "Forward {count} messages",
        Key::DialogNoWritableChats => "No writable chats found",
        Key::DialogEditList => "Edit list",
        Key::DialogNewList => "New list",
        Key::DialogListName => "List name",
        Key::DialogSave => "Save",
        Key::DialogCreate => "Create",
        Key::DialogShortcutsTitle => "Keyboard shortcuts",
        Key::DialogAboutLine => {
            "A native WhatsApp client written in Rust with egui. It connects through whatsapp-rust. Messages are end-to-end encrypted on this device."
        }
        Key::DialogUnofficialWarning => {
            "This is an unofficial client. Using it may be against WhatsApp's terms of service and could get an account suspended."
        }
        Key::DialogSourceCode => "Source code",
        Key::DialogVersion => "Version {version}",
        Key::DialogUnlinkTitle => "Unlink this computer?",
        Key::DialogUnlinkBody => {
            "This removes the device from WhatsApp and deletes the chats stored here. You can link again with a new code."
        }
        Key::DialogUnlinkButton => "Unlink",
        Key::DialogLeaveChannelTitle => "Leave this channel?",
        Key::DialogLeaveGroupTitle => "Leave this group?",
        Key::DialogLeaveBody => {
            "You will not receive new messages. The local history stays on this computer."
        }
        Key::DialogLeaveChannelAction => "Leave channel",
        Key::DialogLeaveGroupAction => "Leave group",
        Key::DialogLeaveChannelArchive => "Leave channel and archive",
        Key::DialogLeaveGroupArchive => "Leave group and archive",
        Key::DialogLinkPhoneTitle => "Link with a phone number",
        Key::DialogPhoneInstructions => {
            "Enter the WhatsApp phone number with its country code. Do not include a plus sign or leading zero. You will get a code to enter on the phone."
        }
        Key::DialogGetCode => "Get a code",
        Key::DialogNewContactTitle => "New contact",
        Key::DialogNewContactBody => {
            "Enter a phone number with its country code, without a plus sign or leading zero. Add a name to save the contact, or leave it blank to open the chat. WhatsApp uses the first name as the display name."
        }
        Key::DialogFirstName => "First name",
        Key::DialogSurname => "Surname",
        Key::DialogCheckingNumber => "Checking the number…",
        Key::DialogSaveContact => "Save contact",
        Key::DialogMessage => "Message",
        Key::KindGroup => "Group",
        Key::KindChannel => "Channel",
        Key::KindContact => "Contact",
        Key::DialogSaveName => "Save name (Enter)",
        Key::DialogMembersOne => "1 member",
        Key::DialogMembersMany => "{count} members",
        Key::DialogMembersTitle => "Members ({count})",
        Key::DialogLastSeen => "last seen {when}",
        Key::DialogMuted => "Muted",
        Key::DialogMutedUntil => "Muted until {when}",
        Key::DialogRename => "Rename",
        Key::DialogAddToContacts => "Add to contacts",
        Key::DialogCopyNumber => "Copy number",
        Key::DialogUnmute => "Unmute",
        Key::DialogMute => "Mute",
        Key::DialogUnfavorite => "Unfavorite",
        Key::DialogFavorite => "Favorite",
        Key::DialogUnpin => "Unpin",
        Key::DialogPin => "Pin",
        Key::ChatArchive => "Archive",
        Key::ChatUnarchive => "Unarchive",
        Key::TrayShowHide => "Show or hide WhatsFast",
        Key::TrayThreadStopped => "The tray thread stopped responding",
        Key::DisplayYou => "You",
        Key::DisplayYouPrefix => "You: ",
        Key::TypingOne => "{who} is typing…",
        Key::TypingMany => "{others} and {last} are typing…",
        Key::ChatListHideCtrl => "Hide the chat list (Ctrl+B)",
        Key::ChatListHideCmd => "Hide the chat list (⌘B)",
        Key::ChatListNewContactCmd => "New contact (⌘N)",
        Key::ChatListSearchCtrl => "Search (Ctrl+F)",
        Key::ChatListArchived => "Archived chats",
        Key::ChatListBack => "Back to chats",
        Key::ChatListScheduled => "Scheduled",
        Key::ChatListArchivedShort => "Archived",
        Key::ChatListStarred => "Starred",
        Key::ChatListScheduledMessages => "Scheduled messages",
        Key::ChatListStarredMessages => "Starred messages",
        Key::ChatListSettingsCtrl => "Settings (Ctrl+,)",
        Key::ChatListFilterAll => "All",
        Key::ChatListFilterUnread => "Unread",
        Key::ChatListFilterFavorites => "Favorites",
        Key::ChatListFilterGroups => "Groups",
        Key::CommonEdit => "Edit",
        Key::CommonDelete => "Delete",
        Key::ChatListNoScheduled => "No scheduled messages",
        Key::ChatListNoScheduledHint => {
            "Write a message and pick a time with the clock beside the send button."
        }
        Key::ChatListNoStarred => "No starred messages",
        Key::ChatListNoStarredHint => "Pick messages in a chat and press Star to keep them here.",
        Key::ChatListNothingArchived => "Nothing archived",
        Key::ChatListArchivedHint => "Archived chats appear here.",
        Key::ChatListLoading => "Loading your chats",
        Key::ChatListLoadingHint => "Receiving history from your phone.",
        Key::ChatListNoUnread => "No unread chats",
        Key::ChatListNoUnreadHint => "Chats with unread messages appear here.",
        Key::ChatListNoFavorites => "No favorites yet",
        Key::ChatListNoFavoritesHint => "Right-click a chat and add it to favorites.",
        Key::ChatListNoGroups => "No groups",
        Key::ChatListNoGroupsHint => "Group chats appear here.",
        Key::ChatListNothingInList => "Nothing in this list",
        Key::ChatListNothingInListHint => "Right-click this chip and choose Edit to add chats.",
        Key::ChatListNoChats => "No chats yet",
        Key::ChatListNoChatsHint => "New chats appear here. You can start one from your phone.",
        Key::ChatListNoResults => "No results",
        Key::ChatListNoResultsHint => "Try another name, number, or message text.",
        Key::ChatSearchSectionChats => "Chats",
        Key::ChatSearchSectionMessages => "Messages",
        Key::ChatSearchSectionContacts => "Contacts",
        Key::ChatMenuMarkRead => "Mark as read",
        Key::ChatMenuMarkUnread => "Mark as unread",
        Key::ChatMenuRemoveFavorite => "Remove from favorites",
        Key::ChatMenuAddFavorite => "Add to favorites",
        Key::ChatPinToTop => "Pin to top",
        Key::ChatMuteFor8Hours => "Mute for 8 hours",
        Key::ChatMuteForWeek => "Mute for a week",
        Key::ChatMuteForever => "Mute indefinitely",
        Key::ChatInfo => "Info",
        Key::TypingShort => "typing…",
        Key::ChatEmptyLoading => "Your chats appear on the left as they load.",
        Key::ChatEmptySelect => "Select a chat on the left.",
        Key::ChatEmptyHints => "Ctrl+K to search · Ctrl+/ for shortcuts",
        Key::ChatMore => "More",
        Key::ChatClose => "Close chat",
        Key::ChatSearchMessages => "Search messages (Ctrl+G)",
        Key::ChatSelectMessages => "Select messages",
        Key::ChatSelectAll => "Select all",
        Key::ChatNextWallpaper => "Next wallpaper",
        Key::ChatCancelSelection => "Cancel selection",
        Key::ChatSelectedOne => "1 selected",
        Key::ChatSelectedMany => "{count} selected",
        Key::ChatUnstar => "Unstar",
        Key::ChatStar => "Star",
        Key::CommonDownload => "Download",
        Key::CommonForward => "Forward",
        Key::CommonSend => "Send",
        Key::CommonDiscard => "Discard",
        Key::ChatLeftChannel => "You left this channel",
        Key::ChatLeftGroup => "You left this group",
        Key::ChatOnlyAdmins => "Only admins can send messages",
        Key::ChatSendFiles => "Send files (or drop them on the window)",
        Key::ChatAttachHint => "Emoji, GIFs, and stickers",
        Key::ChatTypeMessage => "Type a message",
        Key::ChatAddCaption => "Add a caption",
        Key::ChatRecordVoice => "Record a voice message",
        Key::ChatScheduleMessage => "Schedule this message",
        Key::ChatHintsEnter => {
            "Enter sends · Shift+Enter for a new line · *bold* _italic_ ~strike~ · Ctrl+V pastes a picture"
        }
        Key::ChatHintsCtrlEnter => {
            "Ctrl+Enter sends · *bold* _italic_ ~strike~ · Ctrl+V pastes a picture"
        }
        Key::ChatHideHints => "Hide shortcut hints (restore in Settings)",
        Key::ChatAllShortcuts => "All shortcuts ({keys})",
        Key::ChatEditingMessage => "Editing message",
        Key::ChatStopEditing => "Stop editing (Esc)",
        Key::ChatReplyingTo => "Replying to {who}",
        Key::ChatCancelReply => "Cancel reply (Esc)",
        Key::ChatNewest => "Newest message",
        Key::ChatLoadingOlder => "Loading older messages from your phone…",
        Key::ChatLoadingMessages => "Loading messages from your phone…",
        Key::ChatNoMessages => "No messages here yet",
        Key::MarkerPhoto => "[photo]",
        Key::MarkerGif => "[GIF]",
        Key::MarkerVideo => "[video]",
        Key::MarkerVoiceWith => "[voice message, {duration}]",
        Key::MarkerVoice => "[voice message]",
        Key::MarkerAudio => "[audio]",
        Key::MarkerDocument => "[document: {name}]",
        Key::MarkerSticker => "[sticker]",
        Key::MarkerLocation => "[location]",
        Key::MarkerContact => "[contact: {name}]",
        Key::MarkerPoll => "[poll: {question}]",
        Key::MarkerReplying => "(replying to {name}: \"{text}\")",
        Key::ChatForwarded => "Forwarded",
        Key::ChatDeleteEveryone => "Delete for everyone",
        Key::ChatDeleteMe => "Delete for me",
        Key::ChatShowInFolder => "Show in folder",
        Key::ChatRemoveReaction => "Remove your reaction",
        Key::ChatReactWithEmoji => "React with any emoji",
        Key::ChatReply => "Reply",
        Key::ChatCopyText => "Copy text",
        Key::ChatSent => "Sent {when}",
        Key::ChatDeliveredAt => "Delivered {when}",
        Key::ChatDelivered => "Delivered",
        Key::ChatPlayed => "Played",
        Key::ChatRead => "Read",
        Key::ChatWhatAt => "{what} {when}",
        Key::ChatPagesOne => "1 page",
        Key::ChatPagesMany => "{count} pages",
        Key::KindLocation => "Location",
        Key::KindGif => "GIF",
        Key::KindVideo => "Video",
        Key::ChatOpenInMap => "Open in a map",
        Key::ChatMessageDeleted => "This message was deleted",
        Key::ChatUnsupported => "Unsupported: {what}",
        Key::ChatPictureFailed => "Could not display this picture. Click to open it.",
        Key::ChatPause => "Pause",
        Key::ChatPlay => "Play",
        Key::ChatPreparingSpeed => "Preparing playback speed",
        Key::ChatPlaybackSpeed => "Playback speed",
        Key::DisplayUnknown => "Unknown",
        Key::ToastHistoryLoaded => "History loaded",
        Key::ToastBackOnline => "Back online",
        Key::ToastUnlinked => "This device was unlinked from your phone",
        Key::ToastOpenChatFirst => "Open a chat first",
        Key::ToastSendingFilesOne => "Sending 1 file…",
        Key::ToastSendingFilesMany => "Sending {count} files…",
        Key::ToastCouldNotOpen => "Could not open {what}: {error}",
        Key::ToastCopied => "Copied",
        Key::ToastStickerSaved => "Sticker saved",
        Key::ToastSendingGif => "Sending GIF…",
        Key::ToastCouldNotRecord => "Could not record: {error}",
        Key::DialogDigitsOnly => "Enter the phone number with its country code, using digits only",
        Key::PollErrorQuestion => "Enter a question of up to 255 characters.",
        Key::PollErrorAnswers => "Add 2–12 answers, each with 1–100 characters.",
        Key::PollErrorDuplicates => "Each answer must be different.",
        Key::KindPhoto => "Photo",
        Key::KindVoice => "Voice message",
        Key::KindAudio => "Audio",
        Key::KindSticker => "Sticker",
        Key::KindDocumentWith => "Document: {name}",
        Key::KindLocationWith => "Location: {name}",
        Key::KindContactWith => "Contact: {name}",
        Key::KindPollWith => "Poll: {question}",
        Key::ChatUnsupportedMessage => "Unsupported message ({what})",
        Key::ChatRetryFile => {
            "We are still trying to get this file automatically. Click to retry manually."
        }
        Key::ChatFileGone => "No longer available on WhatsApp's servers",
        Key::PrivacyLastSeen => "Last seen",
        Key::PrivacyOnline => "Online",
        Key::PrivacyProfilePhoto => "Profile photo",
        Key::PrivacyAbout => "About",
        Key::PrivacyGroupsAdd => "Who can add me to groups",
        Key::PrivacyReadReceipts => "Read receipts",
        Key::PrivacyCalls => "Who can call me",
        Key::PrivacyMessages => "Who can message me",
        Key::PrivacyLastSeenHint => "When people can see you were last using WhatsApp.",
        Key::PrivacyOnlineHint => "When people can see you are online now.",
        Key::PrivacyProfilePhotoHint => "Who can see your profile photo.",
        Key::PrivacyAboutHint => "Who can see your About text. This is not the Status tab.",
        Key::PrivacyGroupsHint => "Who can add you to a group.",
        Key::PrivacyReceiptsHint => {
            "Everyone or nobody on this WhatsApp account. The Chats switch still applies to this copy."
        }
        Key::PrivacyCallsHint => "Who can call you on WhatsApp.",
        Key::PrivacyMessagesHint => "Who can start a chat with you.",
        Key::PrivacyHideLastSeen => "Hide last seen from",
        Key::PrivacyHidePhoto => "Hide profile photo from",
        Key::PrivacyHideAbout => "Hide About from",
        Key::PrivacyWhoCannotAdd => "Who cannot add you to groups",
        Key::PrivacyExcept => "Except",
        Key::PrivacyEveryone => "Everyone",
        Key::PrivacyMyContacts => "My contacts",
        Key::PrivacyExceptEllipsis => "Except…",
        Key::PrivacyNobody => "Nobody",
        Key::PrivacySameAsLastSeen => "Same as last seen",
        Key::PrivacyContactsWithNumber => "My contacts and other people with my number",
        Key::ToastNotConnected => "Not connected to WhatsApp",
        Key::ToastNotConnectedYet => "Not connected to WhatsApp yet",
        Key::PollErrCantSendHere => "Polls cannot be sent to this chat.",
        Key::PollErrDisappearing => {
            "Poll creation in disappearing-message chats is not supported yet."
        }
        Key::PollErrRecipients => "Could not load the group recipients",
        Key::PollErrSend => "Could not send the poll. Please try again.",
        Key::PollErrAccountChanged => "The account changed while the poll was being sent.",
        Key::PollErrKeyNotSaved => "The poll was sent, but its voting key could not be saved.",
        Key::PollErrNotReady => "This poll is not ready for voting. Reconnect and try again.",
        Key::PollErrVoteSend => "Could not send your vote. Please try again.",
        Key::PollErrVoteNotSaved => "Your vote was sent, but could not be saved locally.",
        Key::PollVoterOne => "1 voter",
        Key::PollVoterMany => "{count} voters",
        Key::ChatWaitingMessage => "Waiting for this message. Open WhatsApp on your phone",
        Key::ToastHistoryPartFailed => "Could not read part of the chat history: {error}",
        Key::ToastNoOlder => "Your phone did not send older messages. Check that it is online",
        Key::ToastOlderFailed => "Could not request older messages from your phone: {error}",
        Key::ToastStarred => "Starred",
        Key::ToastStarRemoved => "Star removed",
        Key::ToastStarredProgress => "Starred {done} of {total}",
        Key::ChatSendToWhatsApp => "Send to WhatsApp",
        Key::ChatEdited => "edited",
        Key::ToastStickerSaveFailed => "Could not save sticker: {error}",
        Key::DialogAddPack => "Add a sticker pack",
        Key::DialogStickerPacks => "Sticker packs",
        Key::ToastContactSaveFailed => "Could not save contact: {error}",
        Key::ToastContactAdded => "Added {name} to contacts",
        Key::ToastNotOnWhatsApp => "{name} is not on WhatsApp",
        Key::ToastPackAdded => "Added sticker pack \"{name}\"",
        Key::ToastPackFailed => "Could not add sticker pack: {error}",
        Key::ToastPrivacyFailed => "Could not update privacy settings.",
        Key::ToastLinkPhoneFailed => "Could not link by phone number: {error}",
        Key::ToastMessageNotSent => "Message not sent: {error}",
        Key::ToastSavedTo => "Saved to {path}",
        Key::ToastDownloadNotSaved => "The download failed, so it was not saved",
        Key::ToastLeaveChannelFailed => "Could not leave the channel.",
        Key::ToastLeaveGroupFailed => "Could not leave the group.",
        Key::ToastScheduled => "Message scheduled",
        Key::ToastForwardBusy => "Wait for the current forward to finish",
        Key::ToastForwardedProgress => "Forwarded {done} of {total}",
        Key::ToastNotStored => "This message is not stored on this computer",
        Key::ToastNotOnComputer => "This message is not on this computer",
        Key::ToastCannotForward => "This message cannot be forwarded",
        Key::ToastForwardNoData => "The original message data is not available to forward",
        Key::ToastForwardUnreadable => "The original message data could not be read",
        Key::ToastReadChatFailed => "Could not read the chat: {error}",
        Key::ToastKeysMissing => "Attachment download keys are missing",
        Key::ToastNoFile => "This message has no downloadable file",
        Key::ToastNoDownloads => "No Downloads folder on this computer",
        Key::ToastSaveDialogOpen => "A save dialog is already open",
        Key::DialogSaveAttachment => "Save attachment",
        Key::ToastSaveCancelled => "Save cancelled",
        Key::ToastSavingProgress => "Saving {done} of {total}",
        Key::ToastSearchFailed => "Could not search: {error}",
        Key::ToastEditFailed => "Could not send the edit: {error}",
        Key::ToastDeleteFailed => "Could not delete the message for everyone: {error}",
        Key::ToastFileFailed => "Could not send the file: {error}",
        Key::ToastClipboardInvalid => "Clipboard image data is invalid",
        Key::ToastPictureFailed => "Could not send the picture: {error}",
        Key::ToastVoiceFailed => "Could not send the voice message: {error}",
        Key::ToastStickerSendFailed => "Could not send the sticker: {error}",
        Key::ToastGifFailed => "Could not send the GIF: {error}",
        Key::ToastEncodeFailed => "Could not encode the attachment",
        Key::ToastShuttingDown => "The application is shutting down",
        Key::ToastRecipientsSaveFailed => "Could not save the group message recipients",
        Key::ToastStartFailed => "Could not start WhatsApp: {error}",
        Key::ToastDeviceStoreFailed => "Could not open the device store: {error}",
        Key::ErrDetail => ": {message}",
        Key::ErrConnectFailed => "WhatsApp connection failed ({reason}){detail}",
        Key::ErrStreamReplaced => "Another WhatsApp Web session replaced this one",
        Key::ErrTemporaryBan => "WhatsApp has temporarily blocked this account ({code})",
        Key::ErrClientOutdated => "WhatsApp rejected this version of WhatsFast. Update the app",
        Key::KindDocument => "Document",
        Key::KindLiveLocation => "Live location",
        Key::KindContactsOne => "1 contact",
        Key::KindContactsMany => "{count} contacts",
        Key::KindPoll => "Poll",
        Key::KindGroupInvite => "group invite",
        Key::KindStickerPack => "sticker pack",
        Key::KindInteractive => "interactive message",
        Key::KindAnimatedSticker => "animated sticker",
        Key::GifErrNoKey => "GIF search needs a GIPHY API key.",
        Key::GifErrRequest => "GIPHY request failed: {error}",
        Key::GifErrKeyRejected => "GIPHY rejected the API key (error {code}).",
        Key::GifErrResponse => "Invalid GIPHY response: {error}",
        Key::GifErrMessage => "GIPHY: {message}",
        Key::GifErrNoResults => "GIPHY response contained no results",
        Key::ToastClipEmpty => "The clip is empty",
        Key::ToastAudioDecodeFailed => "Could not decode audio: {error}",
        Key::ToastNoSoundOutput => "No sound output: {error}",
        Key::ToastAudioReadFailed => "Could not read the audio: {error}",
        Key::ToastAudioDecodeFailed2 => "Could not decode the audio: {error}",
        Key::ToastNoAudioRecorded => "No audio was recorded",
        Key::ToastMicMissing => "No microphone available: {error}",
        Key::ToastMicFormat => "The microphone has no supported format: {error}",
        Key::ToastMicOpenFailed => "Could not open the microphone: {error}",
        Key::ToastMicEmpty => "The microphone did not record any audio",
        Key::StickerErrMissingPackId => "Missing pack_id. Copy the full signal.art link",
        Key::StickerErrMissingPackKey => {
            "Missing or invalid pack_key. Copy the full signal.art link"
        }
        Key::StickerErrIncompleteData => "The sticker data is incomplete",
        Key::StickerErrKeyDerive => "Could not derive the sticker key",
        Key::StickerErrKeyMismatch => "The key does not match this pack",
        Key::StickerErrDecrypt => "Could not decrypt the sticker pack",
        Key::StickerManifestIncomplete => "The sticker manifest is incomplete",
        Key::StickerManifestNumber => "The sticker manifest contains an invalid number",
        Key::StickerManifestField => "The sticker manifest contains an unknown field",
        Key::StickerErrCertificate => "Could not read the Signal certificate: {error}",
        Key::StickerErrRequest => "signal.art request failed: {error}",
        Key::StickerErrResponse => "Could not read the signal.art response: {error}",
        Key::StickerErrNoStickers => "This pack contains no stickers",
        Key::StickerErrOpenFile => "Could not open the file: {error}",
        Key::StickerErrNotArchive => "This file is not a sticker archive: {error}",
        Key::StickerErrNoneRead => "No stickers could be read from this pack",
        Key::StickerErrWrite => "Could not write the sticker pack: {error}",
        Key::StickerErrCreateFolder => "Could not create the pack folder: {error}",
        Key::StickerErrTooMany => "Too many sticker packs have this name",
        Key::KindStickerPackTitle => "Sticker Pack",
        Key::KindStickers => "Stickers",
        Key::MenuAbout => "About WhatsFast",
        Key::MenuSettings => "Settings…",
        Key::MenuHide => "Hide WhatsFast",
        Key::MenuQuit => "Quit WhatsFast",
        Key::MenuFile => "File",
        Key::MenuNewContact => "New Contact…",
        Key::MenuCloseWindow => "Close Window",
        Key::MenuEdit => "Edit",
        Key::MenuUndo => "Undo",
        Key::MenuRedo => "Redo",
        Key::MenuCut => "Cut",
        Key::MenuCopy => "Copy",
        Key::MenuPaste => "Paste",
        Key::MenuSelectAll => "Select All",
        Key::MenuFind => "Find…",
        Key::MenuView => "View",
        Key::MenuToggleSidebar => "Toggle Sidebar",
        Key::MenuZoomIn => "Zoom In",
        Key::MenuZoomOut => "Zoom Out",
        Key::MenuActualSize => "Actual Size",
        Key::MenuShowWhatsFast => "Show WhatsFast",
        Key::MenuHelp => "Help",
        Key::MenuKeyboardShortcuts => "Keyboard Shortcuts",
        Key::MenuWhatsFastHelp => "WhatsFast Help",
        Key::UpdFlatpak => {
            "Update this installation through your software center or flatpak update."
        }
        Key::UpdSnap => "Update this installation with snap refresh.",
        Key::UpdCargo => "Update this installation with cargo install.",
        Key::UpdNix => "Update this installation with Nix or Homebrew.",
        Key::UpdThrough => {
            "Update this installation through {instruction} or your software center."
        }
        Key::UpdSystemDir => {
            "This installation is in a system directory. Use your package manager or the download page."
        }
        Key::UpdNoInstallDir => "The application has no installation directory",
        Key::UpdNotPortable => {
            "This installation does not identify itself as a portable download. Use the download page to install an update-enabled build."
        }
        Key::UpdMissingInstallDir => "Missing installation directory",
        Key::UpdCannotReplaceStaged => "Cannot replace the staged update",
        Key::UpdCannotWriteInstallDir => "Cannot write to the installation directory",
        Key::UpdCannotRunTar => "Cannot run tar to unpack the update",
        Key::UpdMissingArchiveStream => "Missing archive stream",
        Key::UpdExeInvalidSize => "The update executable has an invalid size",
        Key::UpdCannotUnpackExe => "Cannot unpack the update executable",
        Key::UpdDownloadCannotRun => "The downloaded app cannot run on this computer",
        Key::UpdStartupCheckFailed => "The downloaded app failed its startup check",
        Key::UpdMissingVersionOutput => "Missing version output",
        Key::UpdWrongVersion => "The downloaded app has the wrong version",
        Key::UpdStartupCheckNoAnswer => "The downloaded app did not answer its startup check",
        Key::UpdStagedChanged => "The staged update changed. Download it again.",
        Key::UpdCannotStartHelper => "Cannot start the update helper",
        Key::UpdHelperExited => "The update helper exited before it was ready",
        Key::UpdHelperNoStart => "The update helper did not start. Try again.",
        Key::UpdCannotWatchApp => "Cannot watch the running app",
        Key::UpdAppDidNotClose => "The app did not close within one minute",
        Key::UpdCannotIdentifyApp => "Cannot identify the running app",
        Key::UpdStagedChecksumChanged => "The staged update checksum changed",
        Key::UpdAlreadyApplied => "This update was already applied",
        Key::UpdCannotBackUp => "Cannot back up the current app",
        Key::UpdAppStillRunning => "The app is still running or cannot be replaced",
        Key::UpdCannotRestore => "Could not restore the previous app",
        Key::UpdCannotReplaceApp => "Could not replace the app",
        Key::UpdInstallerFailed => "The installer failed. See the update installer log.",
        Key::UpdHasBackup => "This update already has a backup",
        Key::UpdInvalidJobDir => "Invalid update job directory",
        Key::UpdInvalidPayload => "Invalid staged payload",
        Key::UpdInvalidInstallDir => "Invalid installation directory",
        Key::UpdFailed => "Update failed: {error}",
        Key::UpdCannotLaunchUpdated => "Could not launch the updated app",
        Key::UpdUpdatedAppExited => "The updated app exited before opening its window",
        Key::UpdUpdatedAppNoWindow => "The updated app did not open its window within one minute",
        Key::UpdFailedRestored => "Update failed; restored the previous app: {error}",
        Key::UpdUpdatedTo => "Updated to {version}",
        Key::UpdCouldNotStartRestored => {
            "The update could not start. The previous version has been restored."
        }
        Key::UpdCannotRestartPrevious => "Could not restart the previous app",
        Key::UpdInvalidReceiptDir => "Invalid update receipt directory",
        Key::UpdReceiptOtherInstall => "The receipt belongs to a different installation",
        Key::UpdUpdatedWrongVersion => "The updated app reports the wrong version",
        Key::UpdNoUniqueDownload => "The release has no unique {name} download",
        Key::UpdInvalidDownloadSize => "Invalid update download size",
        Key::UpdDuplicateChecksum => "Duplicate checksum for the update",
        Key::UpdInvalidChecksum => "Invalid update checksum",
        Key::UpdMissingChecksum => "The release is missing the update checksum",
        Key::UpdInvalidReleaseVersion => "Invalid release version",
        Key::UpdRedirectNotAllowed => "Update redirect is not allowed",
        Key::UpdReleaseChanged => "The release changed. Check for updates again.",
        Key::UpdUseDownloadPage => {
            "Use the download page for this operating system or architecture"
        }
        Key::UpdNotOnReleaseHost => "Update download is not on the release host",
        Key::UpdAssetOtherRelease => "Update asset does not belong to this release",
        Key::UpdExceedsSize => "Update download exceeds its published size",
        Key::UpdInterrupted => "The update download was interrupted",
        Key::UpdCouldNotVerifyDownload => {
            "The download couldn't be verified. Try downloading it again."
        }
        Key::UpdMoveToApplications => "Move the app to Applications, then open it to update.",
        Key::UpdBundleMissingKey => "The app bundle is missing {key}",
        Key::UpdNotAnAppBundle => "The download is not a WhatsFast app bundle",
        Key::UpdHomebrew => "Update this installation with Homebrew.",
        Key::UpdSignatureUnverified => "The app signature could not be verified",
        Key::UpdCannotReadSigning => "Cannot read the app signing identity",
        Key::UpdBundleWrongVersion => "The app bundle has the wrong version",
        Key::UpdSignedByOther => "The update was signed by a different publisher",
        Key::UpdMacosNotApproved => "macOS could not approve this update for launch",
        Key::UpdMissingAppBundle => "Missing app bundle",
        Key::UpdMissingUpdateDir => "Missing update directory",
        Key::UpdCannotOpenDmg => "macOS could not open the downloaded disk image",
        Key::UpdDmgInvalidBundle => "The disk image has an invalid app bundle",
        Key::UpdDmgNoBundle => "The disk image has no WhatsFast app bundle",
        Key::UpdCannotCopyBundle => "Could not copy the downloaded app bundle",
        Key::UpdCannotBackUpBundle => "Cannot back up the current app bundle",
        Key::UpdCannotRestoreBundle => "Could not restore the previous app bundle",
        Key::UpdCannotReplaceBundle => "Could not replace the app bundle",
        Key::UpdCannotMoveFailed => "Could not move the failed update aside",
        Key::ThemeErrHexColor => "{name}: expected #RRGGBB or #RRGGBBAA",
        Key::ThemeErrUnknownColor => "unknown color: {name}",
        Key::ThemeErrJsonFilename => "expected a JSON filename in the themes folder",
        Key::ThemeErrRegularFile => "expected a regular file, not a directory or symbolic link",
        Key::ThemeErrTooBig => "theme exceeds the 64 KiB file limit",
        Key::ThemeErrUtf8 => "expected UTF-8 JSON",
        Key::ThemeErrFolderUnreadable => {
            "The themes folder could not be read. See the log for details."
        }
        Key::ThemeErrTooManyEntries => {
            "The themes folder has more than 512 entries. Keep fewer files there to list the custom palettes."
        }
        Key::ThemeErrTooManyThemes => {
            "Only 128 custom palettes can be listed. Keep fewer JSON files in the themes folder to see the rest."
        }
        Key::ThemeErrOmarchyLoad => {
            "The Omarchy palette could not be loaded. Keeping the last usable appearance. See the log for details."
        }
        Key::ThemeErrReload => {
            "Custom themes could not be loaded. Run whatsfast reload-themes to try again."
        }
        Key::ThemeLoadingLocal => "Loading local themes…",
        Key::ThemeErrSelectedUnavailable => {
            "The selected theme is unavailable. Keeping the last usable appearance. See the log for details."
        }
        Key::ThemeErrOmarchyColors => "Omarchy's current colors could not be read",
        Key::ThemeErrOmarchyTooBig => "Omarchy theme file exceeds 64 KiB",
        Key::ThemeErrOmarchyIncomplete => "incomplete palette placeholder",
        Key::ThemeErrOmarchyMissingColor => "missing Omarchy color",
        Key::ThemeErrOmarchyMix => "unsupported palette mix",
        Key::ThemeErrRgb => "expected an RGB color",
        Key::ThemeErrOmarchyPlaceholder => "unsupported palette placeholder",
        Key::ArchiveErrBadKeyringKey => "The archive key in the OS keyring is invalid",
        Key::ArchiveErrNoParentDir => "Archive has no parent directory",
        Key::ArchiveErrUnlockKeyring => "Unlock your OS keyring and restart WhatsFast",
        Key::ArchiveErrKeyringOpen => "The OS keyring could not open WhatsFast's archive key",
        Key::ArchiveErrKeySaveMigrated => {
            "Could not save the migrated archive key in the OS keyring"
        }
        Key::ArchiveErrKeyMissing => {
            "The archive is encrypted but its OS keyring key is missing. Restore the original keyring; the archive has not been changed"
        }
        Key::ArchiveErrKeyGenerate => "Could not generate an archive key",
        Key::ArchiveErrKeySave => "Could not save the archive key in the OS keyring",
        Key::ArchiveErrKeyVerify => "Could not verify the saved archive key",
        Key::ArchiveErrKeyNotRetained => "The OS keyring did not retain the archive key",
        Key::ArchiveErrNoCipher => "This build does not support encrypted archives",
        Key::ArchiveErrUnlockFailed => "The archive could not be unlocked with its OS keyring key",
        Key::ArchiveErrClosePrograms => {
            "Close other programs using the archive before migrating it"
        }
        Key::ArchiveErrPathNotUtf8 => "Archive path is not UTF-8",
        Key::ArchiveErrIntegrity => "The encrypted archive failed its integrity check",
        Key::ArchiveErrReplaceFailed => "Could not replace the archive with its encrypted copy",
        Key::NotifyErrIdentity => "notification identity unavailable: {error}",
        Key::SearchClear => "Clear",
    }
}
