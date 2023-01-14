#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Serialize;
use std::fs;
use std::fs::ReadDir;
use std::path::Path;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize)]
struct Card {
    front: String,
    back: String,
}

const COLLECTION_DIR: &str = "/home/camden/flashcards";

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
fn get_decks() -> Result<Vec<String>, String> {
    match fs::read_dir(COLLECTION_DIR) {
        Ok(paths) => Ok(get_decks_from_paths(paths)),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn create_deck(name: &str) -> String {
    match fs::create_dir(Path::new(COLLECTION_DIR).join(name)) {
        Ok(..) => "".to_string(),
        Err(..) => "".to_string(),
    }
}

#[tauri::command]
fn create_card(deck: &str, front: &str, back: &str) -> String {
    match fs::write(Path::new(COLLECTION_DIR).join(deck).join("test.md"), front) {
        Ok(..) => "".to_string(),
        Err(..) => "".to_string(),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_decks, create_deck, create_card])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
