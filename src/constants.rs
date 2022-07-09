pub const NAME: &str = "indexnow-updater";
pub const VERSION: &str = "1.0.1";
pub const REPO: &str = "https://git.ypbind.de/cgit/indexnow-updater";
pub const SQLITE3_SCHEMA: &str = "CREATE TABLE \"files\" (
    filename TEXT PRIMARY KEY,
    sha512 VARCHAR(128)
);";
pub const BATCH_SIZE: usize = 9000;
pub const DEFAULT_TIMEOUT: u64 = 300;

pub fn generate_user_agent() -> String {
    format!("{}/{} ({})", NAME, VERSION, REPO)
}
