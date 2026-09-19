//! Local chat lists and per-list pins. Favorites live on `chats.favorite`.
//!
//! These filters do not sync to WhatsApp. Pins in All still use `chats.pinned`.

use rusqlite::params;

use crate::model::ChatList;

use super::{Archive, Result};

pub const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS chat_lists (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    sort INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS chat_list_members (
    list_id TEXT NOT NULL,
    chat_id TEXT NOT NULL,
    PRIMARY KEY (list_id, chat_id)
);
CREATE TABLE IF NOT EXISTS chat_list_pins (
    list_id TEXT NOT NULL,
    chat_id TEXT NOT NULL,
    pinned_at INTEGER NOT NULL,
    PRIMARY KEY (list_id, chat_id)
);
";

impl Archive {
    pub fn set_favorite(&self, id: &str, favorite: bool) -> Result<()> {
        self.connection.execute(
            "UPDATE chats SET favorite = ?2 WHERE id = ?1",
            params![id, favorite],
        )?;
        Ok(())
    }

    pub fn chat_lists(&self) -> Result<Vec<ChatList>> {
        let mut statement = self
            .connection
            .prepare("SELECT id, name FROM chat_lists ORDER BY sort, name")?;
        let rows = statement.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        let mut lists = Vec::new();
        for row in rows {
            let (id, name) = row?;
            lists.push(ChatList {
                members: self.list_members(&id)?,
                id,
                name,
            });
        }
        Ok(lists)
    }

    fn list_members(&self, list_id: &str) -> Result<Vec<String>> {
        let mut statement = self
            .connection
            .prepare("SELECT chat_id FROM chat_list_members WHERE list_id = ?1")?;
        let rows = statement.query_map(params![list_id], |row| row.get::<_, String>(0))?;
        rows.collect()
    }

    pub fn save_chat_list(&self, id: &str, name: &str, members: &[String]) -> Result<()> {
        let sort = self
            .connection
            .query_row(
                "SELECT COALESCE(MAX(sort), 0) + 1 FROM chat_lists WHERE id != ?1",
                params![id],
                |row| row.get::<_, i64>(0),
            )
            .unwrap_or(1);
        self.connection.execute(
            "INSERT INTO chat_lists (id, name, sort) VALUES (?1, ?2, ?3)
             ON CONFLICT(id) DO UPDATE SET name = excluded.name",
            params![id, name, sort],
        )?;
        self.connection.execute(
            "DELETE FROM chat_list_members WHERE list_id = ?1",
            params![id],
        )?;
        for chat in members {
            self.connection.execute(
                "INSERT INTO chat_list_members (list_id, chat_id) VALUES (?1, ?2)",
                params![id, chat],
            )?;
        }
        Ok(())
    }

    pub fn delete_chat_list(&self, id: &str) -> Result<()> {
        self.connection.execute(
            "DELETE FROM chat_list_members WHERE list_id = ?1",
            params![id],
        )?;
        self.connection
            .execute("DELETE FROM chat_list_pins WHERE list_id = ?1", params![id])?;
        self.connection
            .execute("DELETE FROM chat_lists WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn list_pins(&self) -> Result<Vec<(String, String, i64)>> {
        let mut statement = self
            .connection
            .prepare("SELECT list_id, chat_id, pinned_at FROM chat_list_pins")?;
        let rows = statement.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;
        rows.collect()
    }

    pub fn set_list_pinned(&self, list_id: &str, chat_id: &str, pinned: bool) -> Result<()> {
        if pinned {
            let at = jiff::Timestamp::now().as_millisecond();
            self.connection.execute(
                "INSERT INTO chat_list_pins (list_id, chat_id, pinned_at) VALUES (?1, ?2, ?3)
                 ON CONFLICT(list_id, chat_id) DO UPDATE SET pinned_at = excluded.pinned_at",
                params![list_id, chat_id, at],
            )?;
        } else {
            self.connection.execute(
                "DELETE FROM chat_list_pins WHERE list_id = ?1 AND chat_id = ?2",
                params![list_id, chat_id],
            )?;
        }
        Ok(())
    }

    pub fn reorder_list_pins(&self, list_id: &str, order: &[String]) -> Result<()> {
        let mut current = Vec::new();
        {
            let mut statement = self.connection.prepare(
                "SELECT chat_id FROM chat_list_pins WHERE list_id = ?1 ORDER BY pinned_at DESC, chat_id",
            )?;
            let rows = statement.query_map(params![list_id], |row| row.get::<_, String>(0))?;
            for row in rows {
                current.push(row?);
            }
        }
        let mut wanted: Vec<String> = order
            .iter()
            .filter(|id| current.iter().any(|known| known == *id))
            .cloned()
            .collect();
        for known in current {
            if !wanted.contains(&known) {
                wanted.push(known);
            }
        }
        let base = jiff::Timestamp::now().as_millisecond();
        let count = wanted.len() as i64;
        for (index, chat_id) in wanted.iter().enumerate() {
            let stamp = base + count - index as i64;
            self.connection.execute(
                "UPDATE chat_list_pins SET pinned_at = ?3 WHERE list_id = ?1 AND chat_id = ?2",
                params![list_id, chat_id, stamp],
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::archive::Archive;

    #[test]
    fn a_custom_list_keeps_members_and_its_own_pins() {
        let archive = Archive::in_memory().expect("opens");
        archive
            .ensure_chat("a@s.whatsapp.net", "Ada")
            .expect("chat");
        archive.ensure_chat("b@g.us", "Group").expect("chat");
        archive
            .save_chat_list("l1", "Work", &["a@s.whatsapp.net".into(), "b@g.us".into()])
            .expect("save");
        archive.set_list_pinned("l1", "b@g.us", true).expect("pin");
        archive.set_favorite("a@s.whatsapp.net", true).expect("fav");
        let lists = archive.chat_lists().expect("lists");
        assert_eq!(lists.len(), 1);
        assert_eq!(lists[0].name, "Work");
        assert_eq!(lists[0].members.len(), 2);
        let pins = archive.list_pins().expect("pins");
        assert_eq!(pins.len(), 1);
        assert_eq!(pins[0].1, "b@g.us");
        let chat = archive.chat("a@s.whatsapp.net").expect("row").expect("ada");
        assert!(chat.favorite);
        assert!(!chat.pinned);
        archive.delete_chat_list("l1").expect("delete");
        assert!(archive.chat_lists().expect("empty").is_empty());
        assert!(archive.list_pins().expect("no pins").is_empty());
        assert!(
            archive
                .chat("a@s.whatsapp.net")
                .expect("kept")
                .expect("ada")
                .favorite
        );
    }
}
