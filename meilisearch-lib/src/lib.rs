#[macro_use]
pub mod error;
pub mod options;

mod analytics;
mod dump;
pub mod index;
pub mod index_controller;
mod index_resolver;
mod snapshot;
pub mod tasks;
mod update_file_store;

use std::env::VarError;
use std::ffi::OsStr;
use std::path::Path;

pub use index_controller::MeiliSearch;
pub use milli;
pub use milli::heed;

mod compression;
pub mod document_formats;

/// Check if a db is empty. It does not provide any information on the
/// validity of the data in it.
/// We consider a database as non empty when it's a non empty directory.
pub fn is_empty_db(db_path: impl AsRef<Path>) -> bool {
    let db_path = db_path.as_ref();

    if !db_path.exists() {
        true
    // if we encounter an error or if the db is a file we consider the db non empty
    } else if let Ok(dir) = db_path.read_dir() {
        dir.count() == 0
    } else {
        true
    }
}

/// Checks if the key is defined in the environment variables.
/// If not, inserts it with the given value.
pub fn export_to_env_if_not_present<T>(key: &str, value: T)
where
    T: AsRef<OsStr>,
{
    if let Err(VarError::NotPresent) = std::env::var(key) {
        std::env::set_var(key, value);
    }
}
