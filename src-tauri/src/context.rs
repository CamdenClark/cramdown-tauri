use std::path::{Path, PathBuf};

pub struct Context {
    collection_path: PathBuf,
}

impl From<&str> for Context {
    fn from(collection_path: &str) -> Self {
        Context { collection_path: Path::from(collection_path).to_path_buf() }
    }
}

impl Context {
    pub fn get_collection_path(&self) -> String {
        self.collection_path
    }
}

