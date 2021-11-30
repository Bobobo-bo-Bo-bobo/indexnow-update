use crate::constants;
use log::info;
use rusqlite;
use std::error::Error;
use std::fs;

pub fn open(f: &str) -> Result<rusqlite::Connection, Box<dyn Error>> {
    let exists = match fs::metadata(f) {
        Ok(_) => true,
        Err(_) => false,
    };

    let db = match rusqlite::Connection::open(f) {
        Ok(v) => v,
        Err(e) => return Err(Box::new(e)),
    };

    if !exists {
        info!("Database file {} does not exist, creating it", f);
        db.execute(constants::SQLITE3_SCHEMA, [])?;
    }

    Ok(db)
}


pub fn file_sha512_from_db(db: &rusqlite::Connection, f: &str) -> Result<String, Box<dyn Error>> {
    let count: u64 = db.query_row("SELECT COUNT(sha512) FROM files WHERE filename=:fname;", &[(":fname", f)], |row| row.get(0))?;
    if count == 0 {
        return Ok(String::new());
    }
    let result: String = db.query_row("SELECT sha512 FROM files WHERE filename=:fname;", &[(":fname", f)], |row| row.get(0))?;
    Ok(result)
}
