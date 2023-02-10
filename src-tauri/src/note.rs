use std::collections::HashMap;
use std::fs;
use std::fs::ReadDir;
use std::io::prelude::*;
use std::path::PathBuf;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use comrak::{markdown_to_html, ComrakOptions};
use regex::Regex;

use crate::deck;
use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Note {
    note_id: String,
    deck_id: String,
    template: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CardState {
    New,
    Learning,
    Review,
    Relearning,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Card {
    note_id: String,
    deck_id: String,
    card_num: u32,
    interval: u32,
    due: Option<DateTime<Utc>>,
    ease: u32,
    state: CardState,
    steps: u32,
    template: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Review {
    note_id: String,
    card_num: String,
    due: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ReviewScore {
    Again,
    Hard,
    Good,
    Easy,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct BasicCard {
    front: String,
    back: String,
}

impl From<Card> for Note {
    fn from(card: Card) -> Self {
        Note {
            note_id: card.note_id,
            deck_id: card.deck_id,
            template: card.template,
        }
    }
}

// Note fields are a hashmap of String => String

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NoteCard {
    front: String,
    back: String,
}

pub fn get_note_path(note: Note) -> PathBuf {
    deck::get_deck_path(&note.deck_id).join(format!("{}_{}.md", &note.note_id, &note.template))
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

fn get_card_from_fields(
    fields: HashMap<String, String>,
    _template: String,
    _card_num: u32,
) -> NoteCard {
    // TODO: Handle different template types
    NoteCard {
        front: fields.get("Front").unwrap().clone(),
        back: fields.get("Back").unwrap().clone(),
    }
}

fn parse_card(md: String, template: String, card_num: u32) -> NoteCard {
    let fields = parse_note_into_fields(md);

    get_card_from_fields(fields, template, card_num)
}

fn render_front(fields: HashMap<String, String>, _template: String, _card_num: u32) -> String {
    let empty = String::default();
    let front = fields.get("Front").unwrap_or(&empty);

    markdown_to_html(front, &ComrakOptions::default())
}

fn render_back(
    fields: HashMap<String, String>,
    _template: String,
    _card_num: u32,
) -> String {
    let empty = String::default();
    let front = fields.get("Front").unwrap_or(&empty);
    let back = fields.get("Back").unwrap_or(&empty);

    markdown_to_html(
        format!("{}\n\n---\n\n{}", front, back).as_str(),
        &ComrakOptions::default(),
    )
}

#[tauri::command]
pub fn read_note(note: Note) -> Result<HashMap<String, String>, String> {
    match fs::read(get_note_path(note)) {
        Ok(f) => Ok(parse_note_into_fields(String::from_utf8(f).unwrap())),
        Err(err) => Err(err.to_string()),
    }
}

pub fn get_note_md(fields: HashMap<String, String>) -> String {
    let mut md = "".to_string();
    for (field, value) in fields.iter() {
        md = format!("{}# {}\n{}\n", md, field, value);
    }
    md
}

#[tauri::command]
pub fn create_note(mut note: Note, fields: HashMap<String, String>) -> String {
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();
    note.note_id = format!("{}_{}", time, note.template);

    match fs::write(get_note_path(note), get_note_md(fields)) {
        Ok(..) => "".to_string(),
        Err(..) => "".to_string(),
    }
}

#[tauri::command]
pub fn update_note(note: Note, fields: HashMap<String, String>) -> String {
    match fs::write(get_note_path(note), get_note_md(fields)) {
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
pub fn preview_note(
    fields: HashMap<String, String>,
    template: String,
    card_num: u32,
    show_back: bool,
) -> String {
    if show_back {
        render_back(fields, template, card_num)
    } else {
        render_front(fields, template, card_num)
    }
}

#[tauri::command]
pub fn render_card(card: Card, back: bool) -> Result<String, String> {
    match fs::read_to_string(get_note_path(card.clone().into())) {
        Ok(content) => {
            if back {
                Ok(render_back(
                    parse_note_into_fields(content),
                    card.template,
                    card.card_num,
                ))
            } else {
                Ok(render_front(
                    parse_note_into_fields(content),
                    card.template,
                    card.card_num,
                ))
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn review_card(card: Card, _score: ReviewScore) -> Result<String, String> {
    match fs::OpenOptions::new().append(true).create(true).open(
        deck::get_deck_path(&card.deck_id)
            .join("reviews")
            .join(format!("{}.json", &card.note_id)),
    ) {
        Ok(mut file) => match file.write_all(&serde_json::to_vec(&card).unwrap()) {
            Ok(..) => Ok("".to_string()),
            Err(..) => Err("".to_string()),
        },
        Err(..) => Err("".to_string()),
    }
}

fn get_due_cards_from_paths(deck: &str, paths: ReadDir) -> Vec<Card> {
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

                    Some(Card {
                        deck_id: deck.to_string(),
                        card_num: 1,
                        due: Option::None,
                        ease: 200,
                        interval: 100,
                        state: CardState::New,
                        steps: 0,
                        template: "basic".to_string(),
                        note_id: note_id.to_string(),
                    })
                }
            },
        )
        .filter(|x| match x.due {
            None => true,
            Some(due) => due < Utc::now(),
        })
        // This is where in the future we'll want to derive other cards based on
        // their templates / cloze deletions
        // we'll also need to parse the filename to get the note id + the template
        .collect()
}

#[tauri::command]
pub fn list_cards_to_review(deck: &str) -> Result<Vec<Card>, String> {
    match fs::read_dir(deck::get_deck_path(deck)) {
        Ok(paths) => Ok(get_due_cards_from_paths(deck, paths)),
        Err(err) => Err(err.to_string()),
    }
}
