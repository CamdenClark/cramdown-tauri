#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::ReadDir;
use std::path::Path;

use std::io::{self, Write};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use comrak::nodes::{AstNode, NodeValue};
use comrak::{format_html, markdown_to_html, parse_document, Arena, ComrakOptions};
use std::fs::File;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Review {
    note_id: String,
    card_num: String,
    due: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum ReviewScore {
    Again,
    Hard,
    Good,
    Easy,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum CardState {
    New,
    Learning,
    Review,
    Relearning,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Note {
    note_id: String,
    deck_id: String,
    template: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Card {
    note_id: String,
    deck_id: String,
    card_num: u32,
    interval: u32,
    due: Option<DateTime<Utc>>,
    ease: u32,
    state: CardState,
    steps: u32,
}

#[tauri::command]
fn review_card(card: Card, score: ReviewScore) -> Result<String, String> {
    let time = Utc::now();
    match fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(COLLECTION_DIR)
    {
        Ok(mut file) => match file.write(&serde_json::to_vec(&card).unwrap()) {
            Ok(..) => Ok("".to_string()),
            Err(..) => Err("".to_string()),
        },
        Err(..) => Err("".to_string()),
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct BasicCard {
    front: String,
    back: String,
}

fn parse_note_into_fields(md: String) -> HashMap<String, String> {
    let re = Regex::new("# (.*)").unwrap();
    let mut fields = HashMap::new();
    let mut current_field: Option<String> = None;
    let mut current_str: String = "".to_string();

    for line in md.split("\n") {
        let current = current_str.clone();
        if let Some(heading) = re.captures(line) {
            match current_field {
                Some(field) => {
                    fields.insert(field.to_string(), current.clone().trim().to_string());
                }
                None => {}
            };
            current_field = Some(heading.get(1).unwrap().as_str().to_string());
            current_str = "".to_string();
        } else {
            if current.is_empty() {
                current_str = line.to_string();
            } else {
                current_str = format!("{}\n{}", current, line);
            }
        }
    }

    match current_field {
        Some(field) => {
            fields.insert(field, current_str.trim().to_string());
        }
        None => {}
    }

    fields
}

fn parse_card(md: String) -> BasicCard {
    let fields = parse_note_into_fields(md);

    BasicCard {
        front: fields.get("Front").unwrap().clone(),
        back: fields.get("Back").unwrap().clone(),
    }
}

#[test]
fn test_parse_card() {
    assert_eq!(
        BasicCard {
            front: "Hello".into(),
            back: "World".into()
        },
        parse_card("# Front\nHello\n# Back\nWorld".into())
    );
    assert_eq!(
        BasicCard {
            front: "Hello\nSomething else".into(),
            back: "World".into()
        },
        parse_card("# Front\nHello\nSomething else\n\n# Back\nWorld".into())
    );
}

fn render_front(card: BasicCard) -> String {
    markdown_to_html(card.front.as_str(), &ComrakOptions::default())
}

fn render_back(card: BasicCard) -> String {
    markdown_to_html(
        format!("{}\n\n---\n\n{}", card.front, card.back).as_str(),
        &ComrakOptions::default(),
    )
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
fn preview_note(show_back: bool, card: BasicCard) -> String {
    if show_back {
        render_back(card)
    } else {
        render_front(card)
    }
}

#[tauri::command]
fn read_note(deck_id: &str, note_id: &str) -> Result<BasicCard, String> {
    match fs::read(
        Path::new(COLLECTION_DIR)
            .join(deck_id)
            .join(format!("{}.md", note_id)),
    ) {
        Ok(f) => Ok(parse_card(String::from_utf8(f).unwrap())),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn update_note(deck_id: &str, note_id: &str, front: &str, back: &str) -> String {
    match fs::write(
        Path::new(COLLECTION_DIR)
            .join(deck_id)
            .join(format!("{}.md", note_id)),
        format!("# Front\n{}\n# Back\n{}", front, back),
    ) {
        Ok(..) => "".to_string(),
        Err(..) => "".to_string(),
    }
}
#[tauri::command]
fn create_note(deck: &str, front: &str, back: &str) -> String {
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();
    match fs::write(
        Path::new(COLLECTION_DIR)
            .join(deck)
            .join(format!("{}_{}.md", time, "basic")),
        format!("# Front\n{}\n# Back\n{}", front, back),
    ) {
        Ok(..) => "".to_string(),
        Err(..) => "".to_string(),
    }
}

fn get_due_cards_from_paths(deck: &str, paths: ReadDir) -> Vec<Card> {
    paths
        .map(|path| match path {
            Ok(p) => Some(p.path().file_stem().unwrap().to_str().unwrap().to_string()),
            Err(_) => None,
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        // This is where in the future we'll want to derive other cards based on
        // their templates / cloze deletions
        // we'll also need to parse the filename to get the note id + the template
        .map(|x| Card {
            deck_id: deck.to_string(),
            card_num: 1,
            due: Option::None,
            ease: 200,
            interval: 100,
            state: CardState::New,
            steps: 0,
            note_id: x,
        })
        .filter(|x| match x.due {
            None => true,
            Some(due) => due < Utc::now()
        })
        .collect()
}

fn get_notes_from_paths(deck: &str, paths: ReadDir) -> Vec<Note> {
    paths
        .map(|path| match path {
            Ok(p) => Some(p.path().file_stem().unwrap().to_str().unwrap().to_string()),
            Err(_) => None,
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        // This is where in the future we'll want to derive other cards based on
        // their templates / cloze deletions
        // we'll also need to parse the filename to get the note id + the template
        .map(|x| Note {
            deck_id: deck.to_string(),
            note_id: x,
            template: "basic".to_string(),
        })
        .collect()
}

#[tauri::command]
fn list_cards_to_review(deck: &str) -> Result<Vec<Card>, String> {
    match fs::read_dir(Path::new(COLLECTION_DIR).join(deck)) {
        Ok(paths) => Ok(get_due_cards_from_paths(deck, paths)),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn render_card(card: Card, back: bool) -> Result<String, String> {
    match fs::read_to_string(
            Path::new(COLLECTION_DIR)
                .join(card.deck_id)
                .join(format!("{}.md", card.note_id))
        ) {
        Ok(content) => {
            if back {
                Ok(render_back(parse_card(content)))
            } else {
                Ok(render_front(parse_card(content)))
            }
        },
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn list_notes(deck: &str) -> Result<Vec<Note>, String> {
    match fs::read_dir(Path::new(COLLECTION_DIR).join(deck)) {
        Ok(paths) => Ok(get_notes_from_paths(deck, paths)),
        Err(err) => Err(err.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_decks,
            create_deck,
            create_note,
            preview_note,
            list_notes,
            read_note,
            update_note,
            list_cards_to_review,
            render_card
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
