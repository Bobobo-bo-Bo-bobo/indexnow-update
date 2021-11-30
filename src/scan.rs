use crate::sqlite3;

use log::{debug, error, info, warn};
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use rusqlite;
use sha2::{Digest, Sha512};
use walkdir::WalkDir;

pub struct Filehash {
    pub file: String,
    pub hash: String,
}

impl std::fmt::Debug for Filehash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Filehash")
        .field("file", &self.file)
        .field("hash", &self.hash)
        .finish()
    }
}

pub fn build_update_list(p: &str, db: &mut rusqlite::Connection, extlist: Vec<String>, purge: bool) -> Result<Vec<String>, Box<dyn Error>> {
    let mut result = Vec::<String>::new();
    let mut inserts = Vec::<Filehash>::new();
    let mut deletes = Vec::<String>::new();
    let mut updates = Vec::<Filehash>::new();
    let mut seen_files = HashSet::<String>::new();

    debug!("Scanning files in {}", p);
    for entry in WalkDir::new(p) {
        let fs_obj = match entry {
            Ok(v) => v,
            Err(e) => {
                warn!("Skipping: {}", e);
                continue;
            }
        };

        let meta = match fs_obj.metadata() {
            Ok(v) => v,
            Err(e) => {
                warn!("Unable to read metadata for {}: {} - skipping", fs_obj.path().display(), e);
                continue;
            }
        };

        if !meta.is_file() {
            if meta.is_dir() {
                continue;
            };

            warn!("{} is not a file - skipping", fs_obj.path().display());
            continue;
        }

        // Matches extension?
        debug!("Processing {}", fs_obj.path().display());
        if !match_extension_list(fs_obj.path(), &extlist) {
            debug!("Skipping {} because extension does not match the extension list {:?}", fs_obj.path().display(), extlist);
            continue;
        }
        debug!("Extension of {} matches list of extensions {:?}", fs_obj.path().display(), extlist);

        let fname = match fs_obj.path().to_str() {
            Some(v) => v,
            None => {
                warn!("Invalid filename {} - skipping", fs_obj.path().display());
                continue;
            }
        };

        // Check if file exists in the database
        let sha512_from_db = match sqlite3::file_sha512_from_db(db, fname) {
            Ok(v) => v,
            Err(e) => {
                error!("Unable to query database: {}", e);
                return Err(e);
            },
        };

        // File not in database -> Generate sha512 hash add to insertion list and list for IndexNow
        if sha512_from_db.is_empty() {
            debug!("SHA512 for {} not found in database, adding it to insertion list", fname);
            let sha512_from_file = match file_sha512_from_file(fname) {
                Ok(v) => v,
                Err(e) => {
                    warn!("Can't read {}: {} - skipping", fname, e);
                    continue;
                },
            };

            debug!("Calculated SHA512 hash of {} from file -> {}", fname, sha512_from_file);
            let fhash = Filehash{
                file: fname.to_string(),
                hash: sha512_from_file,
            };
            inserts.push(fhash);

            seen_files.insert(fname.to_string());

            result.push(fname.to_string());
            continue;
        }

        // File exists in database -> calculate it's current SHA512
        let sha512_from_file = match file_sha512_from_file(fname) {
            Ok(v) => v,
            Err(e) => {
                warn!("Can't read {}: {} - skipping", fname, e);
                continue;
            },
        };

        seen_files.insert(fname.to_string());

        debug!("Calculated SHA512 hash of {} from file -> {}", fname, sha512_from_file);
        // File has changed, add it to update list for the database and IndexNow
        if sha512_from_db != sha512_from_file {
            debug!("File {} has changed DB:{} != FILE:{} - adding it to update list", fname, sha512_from_db, sha512_from_file);
            let fhash = Filehash{
                file: fname.to_string(),
                hash: sha512_from_file,
            };
            updates.push(fhash);
            result.push(fname.to_string());
            continue;
        }

        debug!("File {} has not changed DB:{} == FILE:{}", fname, sha512_from_db, sha512_from_file);
    }

    if purge {
        debug!("Purge option found, getting list of files from database");
        let files_from_db = sqlite3::get_all_files(db)?;
        let purge_list = files_from_db.difference(&seen_files);
        for p in purge_list {
            deletes.push(p.to_string());
        }
    }

    info!("Updating database: {} inserts, {} updates, {} deletions", inserts.len(), updates.len(), deletes.len());
    sqlite3::db_update(db, inserts, updates, deletes)?;
    Ok(result)
}

fn file_sha512_from_file(f: &str) -> Result<String, Box<dyn Error>> {
    let _raw = fs::read(f)?;
    let raw: &[u8] = &_raw;
    let mut sha512sum = Sha512::new();
    sha512sum.update(raw);
    let hash = sha512sum.finalize();

    Ok(hex::encode(hash))
}

pub fn match_extension_list(f: &std::path::Path, e: &Vec<String>) -> bool {
    let _fext = match Path::extension(f) {
        Some(v) => v,
        None => return false,
    };
    let fext = match _fext.to_str() {
        Some(v) => v,
        None => return false,
    };

    for _ext in e {
        if _ext.to_lowercase() == fext.to_lowercase() {
            return true;
        }
    }

    return false;
}