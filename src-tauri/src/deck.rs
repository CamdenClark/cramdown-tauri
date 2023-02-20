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

#[tauri::command]
pub fn get_decks() -> Result<Vec<String>, String> {
    match fs::read_dir(get_collection_path()) {
        Ok(paths) => Ok(get_decks_from_paths(paths)),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn create_deck(name: &str) -> String {
    match fs::create_dir(get_deck_path(name)) {
        Ok(..) => "".to_string(),
        Err(..) => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::deck;
    use tempfile::tempdir;
    use std::{env, fs};

    #[test]
    fn create_deck() {
        let collection_path = tempdir().unwrap();
        env::set_var("COLLECTION_PATH", collection_path.path().to_str().unwrap());
        deck::create_deck("testdeck");

        assert!(
            fs::read_dir(collection_path)
                .unwrap()
                .all(|paths| "testdeck" == paths.unwrap().file_name().to_str().unwrap()),
            "There should only be one deck (folder) in the collection (folder) with name testdeck"
        )
    }

    #[test]
    fn list_decks() {
        let collection_path = tempdir().unwrap();
        env::set_var("COLLECTION_PATH", collection_path.path().to_str().unwrap());
        deck::create_deck("testdeck");

        let decks = deck::get_decks().unwrap();

        assert!(
            decks.into_iter().all(|deck| "testdeck" == deck),
            "There should only be one deck (folder) in the collection (folder) with name testdeck"
        )
    }
}
