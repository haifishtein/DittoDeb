//! ditto-deb-core — logica sin Tauri/WebKit para cargo test en Deck sin libs sistema.
pub mod html;
pub mod import;
pub mod pins;
use rusqlite::{Connection, Result};
use sha2::{Digest, Sha256};

pub fn init_db(path: &str) -> Result<Connection> {
    if path != ":memory:" {
        if let Some(parent) = std::path::Path::new(path).parent() {
            if !parent.as_os_str().is_empty() {
                let _ = std::fs::create_dir_all(parent);
            }
        }
    }
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
    // FTS5 trata '-' y ':' como operadores; entrecomillamos cada termino para
    // buscar texto literal (incluyendo palabras con guiones).
    let q = q
        .split_whitespace()
        .map(|t| format!("\"{}\"", t.replace('"', "\"\"")))
        .collect::<Vec<_>>()
        .join(" ");
    if q.is_empty() {
        return Ok(Vec::new());
    }
    let mut stmt =
        conn.prepare("SELECT rowid,text FROM clips_fts WHERE clips_fts MATCH ?1 LIMIT ?2")?;
    let mapped = stmt.query_map([q, limit.to_string()], |r| Ok((r.get(0)?, r.get(1)?)))?;
    let rows: Vec<(i64, String)> = mapped.collect::<Result<Vec<_>, _>>()?;
    Ok(rows)
}

pub fn hash_text(text: &str) -> String {
    let mut h = Sha256::new();
    h.update(text.as_bytes());
    format!("{:x}", h.finalize())
}

pub fn should_save(text: &str, last_hash: &str) -> bool {
    if text.trim().is_empty() || text.len() > 1_000_000 {
        return false;
    }
    hash_text(text) != last_hash
}

pub fn get_clip_detail(
    conn: &Connection,
    id: i64,
) -> Result<Option<(String, Option<String>, String)>> {
    let mut stmt = conn.prepare("SELECT text, html, kind FROM clips WHERE id=?1")?;
    let mut rows = stmt.query([id])?;
    if let Some(r) = rows.next()? {
        let text: String = r.get(0)?;
        let html: Option<String> = r.get(1)?;
        let kind: Option<String> = r.get(2)?;
        Ok(Some((
            text,
            html,
            kind.unwrap_or_else(|| "text".to_string()),
        )))
    } else {
        Ok(None)
    }
}

pub fn db_path() -> String {
    std::env::var("DITTO_DEB_DB").unwrap_or_else(|_| {
        let base = dirs::data_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        base.join("ditto-deb/ditto.db").to_string_lossy().into_owned()
    })
}

pub fn is_image_allowed(size_bytes: usize) -> bool {
    size_bytes > 0 && size_bytes <= 10 * 1024 * 1024
}

pub fn thumb_path(cache_dir: &str, id: i64) -> String {
    format!("{cache_dir}/{id}.thumb.png")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insert_and_search() {
        let conn = init_db(":memory:").unwrap();
        insert_clip(&conn, "hello ditto-deb steam deck").unwrap();
        insert_clip(&conn, "other text").unwrap();
        let r = search_clips(&conn, "ditto-deb", 10).unwrap();
        assert_eq!(r.len(), 1);
        assert!(r[0].1.contains("ditto-deb"));
    }
    #[test]
    fn dedup_and_empty() {
        let h = hash_text("abc");
        assert!(!should_save("", &h));
        assert!(!should_save("   ", &h));
        assert!(should_save("abc", "other"));
        assert!(!should_save("abc", &h));
    }
    #[test]
    fn image_limits() {
        assert!(!is_image_allowed(0));
        assert!(is_image_allowed(1024));
        assert!(!is_image_allowed(11 * 1024 * 1024));
        assert_eq!(thumb_path("/tmp/c", 5), "/tmp/c/5.thumb.png");
    }
    #[test]
    fn import_fake_ditto_db() {
        let dir = std::env::temp_dir().join(format!("ditto-deb-t40-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let src_path = dir.join("Ditto.db");
        {
            let src = Connection::open(&src_path).unwrap();
            src.execute_batch("CREATE TABLE MainDeletes(mText TEXT); INSERT INTO MainDeletes(mText) VALUES('hola ditto'),(''),(NULL);").unwrap();
        }
        let dst = init_db(":memory:").unwrap();
        let n = crate::import::import_ditto_db(&dst, src_path.to_str().unwrap()).unwrap();
        assert_eq!(n, 1);
        let _ = std::fs::remove_dir_all(&dir);
    }
}

pub fn insert_clip_full(
    conn: &Connection,
    text: &str,
    html: Option<&str>,
    kind: &str,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO clips(text,html,kind,created_at) VALUES(?1,?2,?3,strftime('%s','now'))",
        (text, html, kind),
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn save_image_png(bytes: &[u8], width: usize, height: usize) -> Result<String> {
    use image::{ImageFormat, RgbaImage};
    use std::fs;
    use std::path::PathBuf;
    let mut h = sha2::Sha256::new();
    use sha2::Digest;
    h.update(bytes);
    let hash = format!("{:x}", h.finalize());
    let dir = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ditto-deb")
        .join("images");
    fs::create_dir_all(&dir).map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let path = dir.join(format!("{}.png", hash));
    if !path.exists() {
        let img = RgbaImage::from_raw(width as u32, height as u32, bytes.to_vec())
            .ok_or_else(|| rusqlite::Error::InvalidColumnName("invalid image dims".to_string()))?;
        img.save_with_format(&path, ImageFormat::Png)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    }
    Ok(path.to_string_lossy().to_string())
}
