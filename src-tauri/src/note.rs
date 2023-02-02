use std::{time::SystemTime};
use std::collections::HashMap;
use std::fs;
use std::fs::ReadDir;
use std::path::Path;

use serde::{Deserialize, Serialize};

use regex::Regex;
use comrak::{markdown_to_html, ComrakOptions};

use crate::deck;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Note {
    note_id: String,
    deck_id: String,
    template: String,
}



fn get_notes_from_paths(deck: &str, paths: ReadDir) -> Vec<Note> {
    let note_filename_regex = Regex::new("([^_]*)?_?(.*).md").unwrap();
    paths
        .filter_map(|path| match path {
            Ok(p) => Some(p),
            Err(_) => None,
        })
        .filter(|x| match x.file_type() {
            Ok(t) => t.is_file(),
            Err(_) => false,
        })
        .map(|path| path.file_name())
        .filter_map(
            |filename| match note_filename_regex.captures(filename.to_str().unwrap()) {
                None => None,
                Some(captures) => {
                    let note_id = captures.get(1).map_or("basic", |x| x.as_str());
                    let template = captures.get(2).map_or("basic", |x| x.as_str());

                    Some(Note {
                        deck_id: deck.to_string(),
                        note_id: note_id.to_string(),
                        template: template.to_string(),
                    })
                }
            },
        )
        // This is where in the future we'll want to derive other cards based on
        // their templates / cloze deletions
        // we'll also need to parse the filename to get the note id + the template
        .collect()
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BasicCard {
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
fn render_front(card: BasicCard) -> String {
    markdown_to_html(card.front.as_str(), &ComrakOptions::default())
}

fn render_back(card: BasicCard) -> String {
    markdown_to_html(
        format!("{}\n\n---\n\n{}", card.front, card.back).as_str(),
        &ComrakOptions::default(),
    )
}

#[tauri::command]
pub fn read_note(deck_id: &str, note_id: &str) -> Result<BasicCard, String> {
    match fs::read(
        deck::get_deck_path(deck_id)
            .join(format!("{}.md", note_id)),
    ) {
        Ok(f) => Ok(parse_card(String::from_utf8(f).unwrap())),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn update_note(deck_id: &str, note_id: &str, front: &str, back: &str) -> String {
    match fs::write(
            deck::get_deck_path(deck_id)
            .join(format!("{}.md", note_id)),
        format!("# Front\n{}\n# Back\n{}", front, back),
    ) {
        Ok(..) => "".to_string(),
        Err(..) => "".to_string(),
    }
}

#[tauri::command]
pub fn create_note(deck: &str, front: &str, back: &str) -> String {
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();
    match fs::write(
        deck::get_deck_path(deck)
            .join(format!("{}_{}.md", time, "basic")),
        format!("# Front\n{}\n# Back\n{}", front, back),
    ) {
        Ok(..) => "".to_string(),
        Err(..) => "".to_string(),
    }
}


#[tauri::command]
pub fn list_notes(deck: &str) -> Result<Vec<Note>, String> {
    match fs::read_dir(deck::get_deck_path(deck)) {
        Ok(paths) => Ok(get_notes_from_paths(deck, paths)),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn preview_note(show_back: bool, note: BasicCard) -> String {
    if show_back {
        render_back(note)
    } else {
        render_front(note)
    }
}

