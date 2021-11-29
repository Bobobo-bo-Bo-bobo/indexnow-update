pub const NAME: &str = "indexnow-updater";
pub const VERSION: &str = "0.1.0-20211129";
pub const REPO: &str = "https://git.ypbind.de/cgit/indexnow-updater";
pub const SQLITE3_SCHEMA: &str = "CREATE TABLE \"files\" (
    file TEXT PRIMARY KEY,
    sha512 VARCHAR(128)
);";
