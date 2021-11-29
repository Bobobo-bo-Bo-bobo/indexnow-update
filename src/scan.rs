use log::{debug, info, warn};
use std::error::Error;
use std::path::Path;
use rusqlite;
use walkdir::WalkDir;

pub fn build_update_list(p: &str, db: &rusqlite::Connection, extlist: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    let mut result = Vec::<String>::new();

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
            warn!("{} is not a file - skipping", fs_obj.path().display());
            continue;
        }

        // Matches extension?
        debug!("Processing {}", fs_obj.path().display());
        if !match_extension_list(fs_obj.path(), &extlist) {
            debug!("Skipping {} because extension does not match the extension list {:?}", fs_obj.path().display(), extlist);
            continue;
        }

        // Check if file exists in the database
    }

    Ok(result)
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