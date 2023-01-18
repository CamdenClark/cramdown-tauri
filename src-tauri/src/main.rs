#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use regex::Regex;
use serde::{Serialize, Deserialize};
use std::fs;
use std::fs::ReadDir;
use std::path::Path;

use std::io;

use comrak::nodes::{AstNode, NodeValue};
use comrak::{format_html, markdown_to_html, parse_document, Arena, ComrakOptions};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Card {
    front: String,
    back: String,
}

fn parse_card(md: String) -> Card {
    let re = Regex::new("# (.*)").unwrap();
    
    let mut new_line = "".to_string();

    let mut front = "Hello".to_string();

    for line in md.split("\n") {
        if let Some(heading) = re.captures(line) {
            if heading.get(1).unwrap().as_str() == "Back" { 
                front = new_line.trim().to_string();
                new_line = "".to_string();
            }
        } else {
            if new_line.is_empty() {
                new_line = line.to_string();
            } else {
                new_line = format!("{}\n{}", new_line, line);
            }
        }
    }

    Card {
        front,
        back: new_line.trim().to_string()
    }

}



#[test]
fn test_parse_card() {
    assert_eq!(
        Card {
            front: "Hello".into(),
            back: "World".into()
        },
        parse_card("# Front\nHello\n# Back\nWorld".into())
    );
    assert_eq!(
        Card {
            front: "Hello\nSomething else".into(),
            back: "World".into()
        },
        parse_card("# Front\nHello\nSomething else\n\n# Back\nWorld".into())
    );
}

fn render_front(card: Card) -> String {
    markdown_to_html(
        card.front.as_str(),
        &ComrakOptions::default(),
    )
}

fn render_back(card: Card) -> String {
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
fn preview_note(show_back: bool, card: Card) -> String {
    if show_back {
        render_back(card)
    } else {
        render_front(card)
    }
}

#[tauri::command]
fn create_note(deck: &str, front: &str, back: &str) -> String {
    match fs::write(
        Path::new(COLLECTION_DIR).join(deck).join("test.md"),
        format!("# Front\n{}\n# Back\n{}", front, back),
    ) {
        Ok(..) => "".to_string(),
        Err(..) => "".to_string(),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_decks,
            create_deck,
            create_note,
            preview_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
