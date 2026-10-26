//! T40 import Ditto.db (Windows) -> DittoX clips text.
//! Ditto original guarda en SQLite (Ditto.db). Schema varia por version, asi que
//! probamos tablas/columnas comunes y extraemos texto.
use rusqlite::{Connection, Result};

pub fn import_ditto_db(dst: &Connection, src_path: &str) -> Result<usize> {
    let src = Connection::open(src_path)?;
    let mut count = 0;
    let mut tables_stmt = src.prepare("SELECT name FROM sqlite_master WHERE type='table'")?;
    let tables: Vec<String> = tables_stmt
        .query_map([], |r| r.get(0))?
        .collect::<Result<Vec<_>>>()?;
    for table in tables {
        if table.starts_with("sqlite_") {
            continue;
        }
        let pragma = format!("PRAGMA table_info('{table}')");
        let mut col_stmt = src.prepare(&pragma)?;
        let cols: Vec<(i64, String, String)> = col_stmt
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
            .collect::<Result<Vec<_>>>()?;
        let text_col = cols
            .iter()
            .find(|(_, n, t)| {
                let nl = n.to_lowercase();
                (nl.contains("text") || nl.contains("clip") || nl.contains("data") || nl == "mtext")
                    && t.to_uppercase().contains("TEXT")
            })
            .map(|(_, n, _)| n.clone());
        if let Some(col) = text_col {
            let q = format!("SELECT \"{col}\" FROM \"{table}\" WHERE \"{col}\" IS NOT NULL");
            let mut stmt = match src.prepare(&q) {
                Ok(s) => s,
                Err(_) => continue,
            };
            let rows = match stmt.query_map([], |r| r.get::<_, Option<String>>(0)) {
                Ok(r) => r,
                Err(_) => continue,
            };
            for row in rows.flatten().flatten() {
                let t = row.trim().to_string();
                if t.is_empty() || t.len() > 1_000_000 {
                    continue;
                }
                if super::insert_clip(dst, &t).is_ok() {
                    count += 1;
                }
                if count >= 5000 {
                    return Ok(count);
                }
            }
        }
    }
    Ok(count)
}
