#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod collection;
pub mod deck;
pub mod note;
//pub mod review;


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            deck::get_decks,
            deck::create_deck,
            note::create_note,
            note::preview_note,
            note::read_note,
            note::update_note,
            note::list_cards_to_review,
            note::render_card,
            note::review_card,
            note::list_notes
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
