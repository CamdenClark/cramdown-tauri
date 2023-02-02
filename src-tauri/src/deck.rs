use std::fs;
use std::fs::ReadDir;
use std::path::PathBuf;

use crate::collection::get_collection_path;

pub fn get_deck_path(deck: &str) -> PathBuf {
    get_collection_path().join(deck)
}


fn get_decks_from_paths(paths: ReadDir) -> Vec<String> {
    paths
        .map(|path| match path {
            Ok(p) => Some(p.path().file_stem().unwrap().to_str().unwrap().to_string()),
            Err(_) => None,
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect()
}


pub fn get_decks() -> Result<Vec<String>, String> {
    match fs::read_dir(get_collection_path()) {
        Ok(paths) => Ok(get_decks_from_paths(paths)),
        Err(err) => Err(err.to_string()),
    }
}

pub fn create_deck(name: &str) -> String {
    match fs::create_dir(get_deck_path(name)) {
        Ok(..) => "".to_string(),
        Err(..) => "".to_string(),
    }
}
