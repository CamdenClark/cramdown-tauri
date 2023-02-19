use std::path::{Path, PathBuf};
use std::env;

pub fn get_collection_path() -> PathBuf {
    Path::new(&env::var("COLLECTION_PATH").unwrap()).to_path_buf()
}
