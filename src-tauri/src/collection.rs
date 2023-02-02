use std::path::Path;

const COLLECTION_DIR: &str = "/home/camden/flashcards";

pub fn get_collection_path() -> &'static Path {
    Path::new(COLLECTION_DIR)
}
