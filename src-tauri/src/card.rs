use std::fs;
use std::fs::ReadDir;
use std::io::prelude::*;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use regex::Regex;

use chrono::{DateTime, Utc};

use crate::{deck, review, note};
use crate::note::Note;
use crate::review::{Review, ReviewScore, ReviewState};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Card {
    note_id: String,
    deck_id: String,
    card_num: u32,
    interval: f64,
    due: Option<DateTime<Utc>>,
    ease: f64,
    state: ReviewState,
    steps: u32,
    template: String,
    score: ReviewScore,
}

pub fn get_review_path(card: Card) -> PathBuf {
    deck::get_deck_path(&card.deck_id)
        .join("reviews")
        .join(format!("{}.jsonl", &card.note_id))
}

impl From<Card> for Note {
    fn from(card: Card) -> Self {
        Note::new(card.note_id, card.deck_id, card.template)
    }
}

impl From<Card> for Review {
    fn from(card: Card) -> Self {
        Review::new(card.due, card.interval, card.ease, card.state, card.steps)
    }
}

impl Card {
    fn update_from_review(self, review: Review, score: ReviewScore) -> Card {
        Card {
            due: review.due,
            interval: review.interval,
            ease: review.ease,
            state: review.state,
            steps: review.steps,
            score,
            ..self
        }
    }
}

impl Default for Card {
    fn default() -> Self {
        Card {
            note_id: String::from("test"),
            card_num: 1,
            interval: 1.0,
            ease: 2.5,
            steps: 0,
            template: String::from("basic"),
            due: None,
            deck_id: String::from("test"),
            state: ReviewState::New,
            score: ReviewScore::Good,
        }
    }
}

#[tauri::command]
pub fn render_card(card: Card, back: bool) -> Result<String, String> {
    note::render_note_card(card.clone().into(), card.card_num, back)
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
                        note_id: note_id.to_string(),
                        ..Card::default()
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

#[tauri::command]
pub fn review_card(card: Card, score: ReviewScore) -> Result<String, String> {
    let new_review = review::score_card(card.clone().into(), Utc::now(), score.clone());

    let new_card = card.clone().update_from_review(new_review, score.clone());

    match fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(get_review_path(card.clone().into()))
    {
        Ok(mut file) => match file.write_all(&serde_json::to_vec(&new_card).unwrap()) {
            Ok(..) => Ok("".to_string()),
            Err(..) => Err("".to_string()),
        },
        Err(..) => Err("".to_string()),
    }
}

