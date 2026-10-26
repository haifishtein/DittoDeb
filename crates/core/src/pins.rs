//! T22 pins/grupos — toggle_pin + list_pinned + groups tabla.
use rusqlite::{Connection, Result};

pub fn ensure_groups_table(conn: &Connection) -> Result<()> {
    conn.execute_batch("CREATE TABLE IF NOT EXISTS groups(id INTEGER PRIMARY KEY, name TEXT UNIQUE); CREATE TABLE IF NOT EXISTS clip_groups(clip_id INTEGER, group_id INTEGER, PRIMARY KEY(clip_id, group_id));")?;
    Ok(())
}

pub fn toggle_pin(conn: &Connection, id: i64) -> Result<bool> {
    conn.execute("UPDATE clips SET pinned = 1 - pinned WHERE id=?1", [id])?;
    let pinned: i64 = conn.query_row("SELECT pinned FROM clips WHERE id=?1", [id], |r| r.get(0))?;
    Ok(pinned == 1)
}

pub fn list_pinned(conn: &Connection, limit: i64) -> Result<Vec<(i64, String)>> {
    let mut s =
        conn.prepare("SELECT id,text FROM clips WHERE pinned=1 ORDER BY id DESC LIMIT ?1")?;
    let rows: Vec<(i64, String)> = s
        .query_map([limit], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init_db;
    #[test]
    fn toggle_pin_works() {
        let conn = init_db(":memory:").unwrap();
        conn.execute("INSERT INTO clips(text,created_at) VALUES('x',1)", [])
            .unwrap();
        let id: i64 = conn.last_insert_rowid();
        toggle_pin(&conn, id).unwrap();
        let pinned: i64 = conn
            .query_row("SELECT pinned FROM clips WHERE id=?1", [id], |r| r.get(0))
            .unwrap();
        assert_eq!(pinned, 1);
        toggle_pin(&conn, id).unwrap();
        let pinned2: i64 = conn
            .query_row("SELECT pinned FROM clips WHERE id=?1", [id], |r| r.get(0))
            .unwrap();
        assert_eq!(pinned2, 0);
    }
}
