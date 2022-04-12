pub const NAME: &str = "indexnow-updater";
pub const VERSION: &str = "0.1.0-20220412";
pub const REPO: &str = "https://git.ypbind.de/cgit/indexnow-updater";
pub const SQLITE3_SCHEMA: &str = "CREATE TABLE \"files\" (
    filename TEXT PRIMARY KEY,
    sha512 VARCHAR(128)
);";
pub const BATCH_SIZE: usize = 9000;
