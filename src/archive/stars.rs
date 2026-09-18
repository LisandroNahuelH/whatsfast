//! Messages the user starred, kept in the encrypted archive.
//!
//! The list is local. A star is written here only after WhatsApp confirms it,
//! so the archive never claims something the server refused. The phone's own
//! starred messages are not imported yet: the client sends the star and does
//! not read the reverse path back. The lib's event bus carries a `StarUpdate`
//! for that direction, so wiring it is the upgrade when someone asks for it.
//!
//! ponytail: one table, no message column. The text is read from the message
//! row itself, so an edited message shows its current words and a message
//! deleted here leaves the list on its own.

use std::collections::HashSet;

use rusqlite::params;

use super::{Archive, Result};

pub const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS stars (
    chat TEXT NOT NULL,
    id TEXT NOT NULL,
    starred_at INTEGER NOT NULL,
    PRIMARY KEY (chat, id)
);
CREATE INDEX IF NOT EXISTS stars_by_time ON stars (starred_at);
";

/// One starred message, as the list shows it.
#[derive(Clone, Debug)]
pub struct Starred {
    pub chat: String,
    pub id: String,
    /// Unix seconds of the moment the star was confirmed.
    pub starred_at: i64,
    /// The message's own words. Personal data: never logged.
    pub text: String,
    /// Whether we sent it, so the list can draw it on the same side with the
    /// same colour as in the chat.
    pub from_me: bool,
}

impl Archive {
    /// Marks a message as starred, remembering when.
    pub fn star(&self, chat: &str, id: &str, at: i64) -> Result<()> {
        self.connection.execute(
            "INSERT INTO stars (chat, id, starred_at) VALUES (?1, ?2, ?3)
             ON CONFLICT(chat, id) DO UPDATE SET starred_at = excluded.starred_at",
            params![chat, id, at],
        )?;
        Ok(())
    }

    pub fn unstar(&self, chat: &str, id: &str) -> Result<()> {
        self.connection.execute(
            "DELETE FROM stars WHERE chat = ?1 AND id = ?2",
            params![chat, id],
        )?;
        Ok(())
    }

    /// The starred messages of one chat, for the mark in the conversation.
    pub fn starred_ids(&self, chat: &str) -> Result<HashSet<String>> {
        let mut statement = self
            .connection
            .prepare("SELECT id FROM stars WHERE chat = ?1")?;
        let rows = statement.query_map(params![chat], |row| row.get::<_, String>(0))?;
        let mut ids = HashSet::new();
        for row in rows {
            ids.insert(row?);
        }
        Ok(ids)
    }

    /// Every starred message, newest star first, for the list. The join hides
    /// anything deleted here, so the list never shows a ghost.
    pub fn starred(&self, limit: usize) -> Result<Vec<Starred>> {
        let mut statement = self.connection.prepare(
            "SELECT s.chat, s.id, s.starred_at, m.content, m.from_me
             FROM stars s JOIN messages m ON m.chat = s.chat AND m.id = s.id
             ORDER BY s.starred_at DESC, s.rowid DESC LIMIT ?1",
        )?;
        let rows = statement.query_map(params![limit as i64], |row| {
            let raw: String = row.get(3)?;
            let content: crate::model::Content =
                serde_json::from_str(&raw).unwrap_or(crate::model::Content::Unsupported {
                    what: "starred".to_owned(),
                });
            Ok(Starred {
                chat: row.get(0)?,
                id: row.get(1)?,
                starred_at: row.get(2)?,
                text: content.summary(),
                from_me: row.get(4)?,
            })
        })?;
        let mut list = Vec::new();
        for row in rows {
            list.push(row?);
        }
        Ok(list)
    }
}
