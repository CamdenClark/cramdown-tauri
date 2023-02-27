#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod deck;
pub mod note;
pub mod card;
pub mod review;
pub mod context;

use std::env;

fn main() {
    let collection_path = &env::var("COLLECTION_PATH").unwrap();
    tauri::Builder::default()
        .manage(context::Context::from(collection_path))
        .invoke_handler(tauri::generate_handler![
            deck::get_decks_handler,
            deck::create_deck,
            note::list_notes,
            note::create_note,
            note::preview_note,
            note::read_note,
            note::update_note,
            card::list_cards_to_review,
            card::render_card,
            card::review_card,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
