//! SQLite schema v1 — FTS5
//! CREATE TABLE clips(id INTEGER PRIMARY KEY, text TEXT, html TEXT, kind TEXT, app TEXT, created_at INTEGER, pinned INTEGER DEFAULT 0);
//! CREATE VIRTUAL TABLE clips_fts USING fts5(text, content=clips, content_rowid=id);
//! + triggers ai/ad/au. Path: ~/.local/share/ditto-deb/ditto.db
use rusqlite::{Connection, Result};

pub fn init_db(path: &str) -> Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS clips(id INTEGER PRIMARY KEY, text TEXT, html TEXT, kind TEXT DEFAULT 'text', app TEXT DEFAULT '', created_at INTEGER, pinned INTEGER DEFAULT 0);
         CREATE VIRTUAL TABLE IF NOT EXISTS clips_fts USING fts5(text, content=clips, content_rowid=id);
         CREATE TRIGGER IF NOT EXISTS clips_ai AFTER INSERT ON clips BEGIN INSERT INTO clips_fts(rowid,text) VALUES(new.id,new.text); END;
         CREATE TRIGGER IF NOT EXISTS clips_ad AFTER DELETE ON clips BEGIN INSERT INTO clips_fts(clips_fts,rowid,text) VALUES('delete',old.id,old.text); END;
         CREATE TRIGGER IF NOT EXISTS clips_au AFTER UPDATE ON clips BEGIN INSERT INTO clips_fts(clips_fts,rowid,text) VALUES('delete',old.id,old.text); INSERT INTO clips_fts(rowid,text) VALUES(new.id,new.text); END;",
    )?;
    Ok(conn)
}

pub fn insert_clip(conn: &Connection, text: &str) -> Result<i64> {
    conn.execute(
        "INSERT INTO clips(text,created_at) VALUES(?1,strftime('%s','now'))",
        [text],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn search_clips(conn: &Connection, q: &str, limit: i64) -> Result<Vec<(i64, String)>> {
    let mut stmt =
        conn.prepare("SELECT rowid,text FROM clips_fts WHERE clips_fts MATCH ?1 LIMIT ?2")?;
    let rows = stmt.query_map([q, &limit.to_string()], |r| Ok((r.get(0)?, r.get(1)?)))?;
    rows.collect()
}
