use crate::constants;
use crate::scan;
use log::info;
use rusqlite::Result;
use std::collections::HashSet;
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

pub fn get_all_files(db: &rusqlite::Connection) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut result = HashSet::<String>::new();
    let mut statement = db.prepare("SELECT filename FROM files;")?;
    let mut result_iter = statement.query([])?;

    while let Some(row) = result_iter.next()? {
        result.insert(row.get(0)?);
    }

    Ok(result)
}

pub fn file_sha512_from_db(db: &rusqlite::Connection, f: &str) -> Result<String, Box<dyn Error>> {
    let count: u64 = db.query_row(
        "SELECT COUNT(sha512) FROM files WHERE filename=:fname;",
        &[(":fname", f)],
        |row| row.get(0),
    )?;
    if count == 0 {
        return Ok(String::new());
    }
    let result: String = db.query_row(
        "SELECT sha512 FROM files WHERE filename=:fname;",
        &[(":fname", f)],
        |row| row.get(0),
    )?;
    Ok(result)
}

pub fn db_update(
    db: &mut rusqlite::Connection,
    ins: Vec<scan::Filehash>,
    upd: Vec<scan::Filehash>,
    del: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let tx = db.transaction()?;

    for i in ins {
        tx.execute(
            "INSERT INTO files (filename, sha512) VALUES (?1, ?2);",
            [i.file, i.hash],
        )?;
    }

    for u in upd {
        tx.execute(
            "UPDATE files SET sha512=?1 WHERE filename=?2;",
            [u.hash, u.file],
        )?;
    }

    for d in del {
        tx.execute("DELETE FROM files WHERE filename=?1", [d])?;
    }

    tx.commit()?;
    Ok(())
}
