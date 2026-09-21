//! Unsent composer text, kept in the encrypted archive.
//!
//! Local to this copy. WhatsApp app-state drafts are not wired. The body is
//! personal data: never log it.

use rusqlite::params;

use super::{Archive, Result};

pub const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS drafts (
    chat TEXT PRIMARY KEY,
    text TEXT NOT NULL,
    mentions TEXT NOT NULL DEFAULT '[]',
    reply_to TEXT,
    updated_at INTEGER NOT NULL
);
";

/// One composer draft. Personal data: never logged.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Draft {
    pub chat: String,
    pub text: String,
    pub mentions: String,
    pub reply_to: Option<String>,
    pub updated_at: i64,
}

impl Archive {
    pub fn set_draft(
        &self,
        chat: &str,
        text: &str,
        mentions: &str,
        reply_to: Option<&str>,
        updated_at: i64,
    ) -> Result<()> {
        self.connection.execute(
            "INSERT INTO drafts (chat, text, mentions, reply_to, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(chat) DO UPDATE SET
                text = excluded.text,
                mentions = excluded.mentions,
                reply_to = excluded.reply_to,
                updated_at = excluded.updated_at",
            params![chat, text, mentions, reply_to, updated_at],
        )?;
        Ok(())
    }

    pub fn clear_draft(&self, chat: &str) -> Result<()> {
        self.connection
            .execute("DELETE FROM drafts WHERE chat = ?1", params![chat])?;
        Ok(())
    }

    pub fn drafts(&self) -> Result<Vec<Draft>> {
        let mut statement = self
            .connection
            .prepare("SELECT chat, text, mentions, reply_to, updated_at FROM drafts")?;
        let rows = statement.query_map([], |row| {
            Ok(Draft {
                chat: row.get(0)?,
                text: row.get(1)?,
                mentions: row.get(2)?,
                reply_to: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;
        rows.collect()
    }
}
