use std::fs;
use std::fs::ReadDir;
use std::path::{Path, PathBuf};

use tauri::State;

use crate::context::Context;


pub fn get_deck_path(collection: &str, deck: &str) -> PathBuf {
    Path::new(collection).join(deck)
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

fn get_decks(context: &Context) -> Result<Vec<String>, String> {
    match fs::read_dir(context.get_collection_path()) {
        Ok(paths) => Ok(get_decks_from_paths(paths)),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn get_decks_handler(state: State<'_, Context>) -> Result<Vec<String>, String> {
    match fs::read_dir(state.inner().get_collection_path()) {
        Ok(paths) => Ok(get_decks_from_paths(paths)),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn create_deck_handler(state: State<'_, Context>, name: &str) -> String {
    create_deck(state.inner(), name)
}

pub fn create_deck(context: &Context, name: &str) -> String {
    match fs::create_dir(get_deck_path(&context.get_collection_path(), name)) {
        Ok(..) => "".to_string(),
        Err(..) => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{deck, context::Context};
    use tempfile::{tempdir, TempDir};
    use std::fs;
    use tauri::State;

    fn scaffold_collection() -> TempDir {
        tempdir().unwrap()
    }

    #[test]
    fn create_deck() {
        let collection_path = scaffold_collection();
        deck::create_deck(&Context { collection_path: collection_path.into_path() }, "testdeck");

        assert!(
            fs::read_dir(collection_path)
                .unwrap()
                .all(|paths| "testdeck" == paths.unwrap().file_name().to_str().unwrap()),
            "There should only be one deck (folder) in the collection (folder) with name testdeck"
        )
    }

    #[test]
    fn list_decks() {
        let collection_path = scaffold_collection();
        let context = Context { collection_path: collection_path.into_path() };
        deck::create_deck(&context, "testdeck");

        let state = tauri::Manager::manage(context);

        let decks = deck::get_decks_handler(context.into()).unwrap();

        assert!(
            decks.into_iter().all(|deck| "testdeck" == deck),
            "There should only be one deck (folder) in the collection (folder) with name testdeck"
        )
    }
}
