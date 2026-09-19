//! Messages the user scheduled, stored in the encrypted archive and fired by
//! the worker's tick.

use super::{Archive, Result};
use rusqlite::{OptionalExtension, params};

pub const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS scheduled (
    id TEXT PRIMARY KEY,
    chat TEXT NOT NULL,
    text TEXT NOT NULL,
    kind TEXT NOT NULL,
    hour INTEGER NOT NULL,
    minute INTEGER NOT NULL,
    weekday INTEGER,
    day_of_month INTEGER,
    nth INTEGER,
    next_at INTEGER NOT NULL,
    state TEXT NOT NULL,
    message_id TEXT,
    last_error TEXT,
    created_at INTEGER NOT NULL,
    last_fired_at INTEGER
);
CREATE INDEX IF NOT EXISTS scheduled_due ON scheduled (next_at) WHERE state = 'pending';
";

/// One scheduled message, as the archive keeps it.
#[derive(Clone, Debug)]
pub struct Scheduled {
    pub id: String,
    pub chat: String,
    /// The text to send. Personal data: never logged.
    pub text: String,
    pub kind: String,
    pub hour: i8,
    pub minute: i8,
    pub weekday: Option<i8>,
    pub day_of_month: Option<i8>,
    pub nth: Option<i8>,
    /// Unix seconds of the next attempt.
    pub next_at: i64,
    pub state: String,
    /// The outgoing message id, known once the send starts.
    pub message_id: Option<String>,
    pub last_error: Option<String>,
    pub created_at: i64,
    pub last_fired_at: Option<i64>,
}

/// How a scheduled message ended.
#[derive(Clone, Debug)]
pub enum Outcome {
    /// Sent. `next_at` is the next occurrence; `None` closes a one-off.
    Sent { next_at: Option<i64> },
    /// Failed for good. A one-off stays visible as failed; a recurrence keeps
    /// its series and shows the error.
    Failed { next_at: Option<i64>, error: String },
}

impl Archive {
    pub fn insert_scheduled(&self, entry: &Scheduled) -> Result<()> {
        self.connection.execute(
            "INSERT INTO scheduled (id, chat, text, kind, hour, minute, weekday, day_of_month, nth, next_at, state, message_id, last_error, created_at, last_fired_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 'pending', NULL, NULL, ?11, NULL)
             ON CONFLICT(id) DO UPDATE SET chat = excluded.chat, text = excluded.text, kind = excluded.kind,
                 hour = excluded.hour, minute = excluded.minute, weekday = excluded.weekday,
                 day_of_month = excluded.day_of_month, nth = excluded.nth, next_at = excluded.next_at,
                 state = 'pending', message_id = NULL, last_error = NULL",
            params![
                entry.id,
                entry.chat,
                entry.text,
                entry.kind,
                entry.hour,
                entry.minute,
                entry.weekday,
                entry.day_of_month,
                entry.nth,
                entry.next_at,
                entry.created_at,
            ],
        )?;
        Ok(())
    }

    /// Every scheduled message, soonest first, for the list.
    pub fn scheduled(&self) -> Result<Vec<Scheduled>> {
        let mut statement = self.connection.prepare(
            "SELECT id, chat, text, kind, hour, minute, weekday, day_of_month, nth, next_at, state, message_id, last_error, created_at, last_fired_at
             FROM scheduled ORDER BY next_at",
        )?;
        let rows = statement.query_map([], row_of)?;
        let mut list = Vec::new();
        for row in rows {
            list.push(row?);
        }
        Ok(list)
    }

    /// The ones whose time has come.
    pub fn due_scheduled(&self, now: i64) -> Result<Vec<Scheduled>> {
        self.pending_where("next_at <= ?1", params![now])
    }

    /// Ones left mid-send by an earlier run, or stuck for too long.
    pub fn stale_scheduled(&self, before: i64) -> Result<Vec<Scheduled>> {
        self.pending_where(
            "state = 'sending' AND (last_fired_at IS NULL OR last_fired_at < ?1)",
            params![before],
        )
    }

    fn pending_where(&self, clause: &str, args: impl rusqlite::Params) -> Result<Vec<Scheduled>> {
        let sql = format!(
            "SELECT id, chat, text, kind, hour, minute, weekday, day_of_month, nth, next_at, state, message_id, last_error, created_at, last_fired_at
             FROM scheduled WHERE {clause}"
        );
        let mut statement = self.connection.prepare(&sql)?;
        let rows = statement.query_map(args, row_of)?;
        let mut list = Vec::new();
        for row in rows {
            list.push(row?);
        }
        Ok(list)
    }

    /// Takes an occurrence: only one caller wins, so the startup catch-up and
    /// the tick can never both send it. The next occurrence is written when
    /// the send ends, so a failure can simply be retried.
    pub fn claim_scheduled(&self, id: &str, now: i64) -> Result<bool> {
        let changed = self.connection.execute(
            "UPDATE scheduled SET state = 'sending', message_id = NULL, last_fired_at = ?2
             WHERE id = ?1 AND state = 'pending'",
            params![id, now],
        )?;
        Ok(changed > 0)
    }

    /// Puts an occurrence back after a send could not start.
    pub fn release_scheduled(&self, id: &str) -> Result<()> {
        self.connection.execute(
            "UPDATE scheduled SET state = 'pending' WHERE id = ?1 AND state = 'sending'",
            params![id],
        )?;
        Ok(())
    }

    /// Remembers which outgoing message carries this occurrence.
    pub fn set_scheduled_message_id(&self, id: &str, message_id: &str) -> Result<()> {
        self.connection.execute(
            "UPDATE scheduled SET message_id = ?2 WHERE id = ?1",
            params![id, message_id],
        )?;
        Ok(())
    }

    /// The scheduled message waiting for this outgoing message, if any.
    pub fn scheduled_for_message(&self, chat: &str, message_id: &str) -> Result<Option<Scheduled>> {
        self.connection
            .query_row(
                "SELECT id, chat, text, kind, hour, minute, weekday, day_of_month, nth, next_at, state, message_id, last_error, created_at, last_fired_at
                 FROM scheduled WHERE chat = ?1 AND message_id = ?2 AND state = 'sending'",
                params![chat, message_id],
                row_of,
            )
            .optional()
    }

    /// Closes an occurrence the way its kind asks for.
    pub fn finish_scheduled(&self, id: &str, now: i64, outcome: Outcome) -> Result<()> {
        match outcome {
            Outcome::Sent { next_at: None } => {
                self.connection
                    .execute("DELETE FROM scheduled WHERE id = ?1", params![id])?;
            }
            Outcome::Sent {
                next_at: Some(next),
            } => {
                self.connection.execute(
                    "UPDATE scheduled SET state = 'pending', next_at = ?2, last_fired_at = ?3, last_error = NULL
                     WHERE id = ?1",
                    params![id, next, now],
                )?;
            }
            Outcome::Failed {
                next_at: Some(next),
                error,
            } => {
                self.connection.execute(
                    "UPDATE scheduled SET state = 'pending', next_at = ?2, last_error = ?3
                     WHERE id = ?1",
                    params![id, next, error],
                )?;
            }
            Outcome::Failed {
                next_at: None,
                error,
            } => {
                self.connection.execute(
                    "UPDATE scheduled SET state = 'failed', last_error = ?2 WHERE id = ?1",
                    params![id, error],
                )?;
            }
        }
        Ok(())
    }

    pub fn delete_scheduled(&self, id: &str) -> Result<()> {
        self.connection
            .execute("DELETE FROM scheduled WHERE id = ?1", params![id])?;
        Ok(())
    }
}

fn row_of(row: &rusqlite::Row<'_>) -> rusqlite::Result<Scheduled> {
    Ok(Scheduled {
        id: row.get(0)?,
        chat: row.get(1)?,
        text: row.get(2)?,
        kind: row.get(3)?,
        hour: row.get(4)?,
        minute: row.get(5)?,
        weekday: row.get(6)?,
        day_of_month: row.get(7)?,
        nth: row.get(8)?,
        next_at: row.get(9)?,
        state: row.get(10)?,
        message_id: row.get(11)?,
        last_error: row.get(12)?,
        created_at: row.get(13)?,
        last_fired_at: row.get(14)?,
    })
}
