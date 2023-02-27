use std::collections::HashMap;
use std::fs;
use std::fs::ReadDir;
// Might need this, unsure... use std::io::prelude::*;
use std::path::PathBuf;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use comrak::{markdown_to_html, ComrakOptions};
use regex::Regex;
use tauri::State;

use crate::collection::CollectionPath;

use crate::deck;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Note {
    note_id: String,
    deck_id: String,
    template: String,
}

impl Note {
    pub fn new(note_id: String, deck_id: String, template: String) -> Self {
        Note {
            note_id,
            deck_id,
            template,
        }
    }
    pub fn get_path(&self) -> PathBuf {
        deck::get_deck_path(&self.deck_id).join(format!("{}_{}.md", self.note_id, self.template))
    }
}

// Note fields are a hashmap of String => String
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NoteCard {
    front: String,
    back: String,
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
    _template: &str,
    _card_num: u32,
) -> NoteCard {
    // TODO: Handle different template types
    NoteCard {
        front: fields
            .get("Front")
            .unwrap_or(&String::default())
            .to_string(),
        back: fields.get("Back").unwrap_or(&String::default()).to_string(),
    }
}

fn render_front(fields: HashMap<String, String>, _template: &str, _card_num: u32) -> String {
    // TODO: Handle different template types
    let empty = String::default();
    let front = fields.get("Front").unwrap_or(&empty);

    markdown_to_html(front, &ComrakOptions::default())
}

fn render_back(fields: HashMap<String, String>, template: &str, card_num: u32) -> String {
    // TODO: Handle different template types
    let display_card = get_card_from_fields(fields, template, card_num);

    markdown_to_html(
        format!("{}\n\n---\n\n{}", display_card.front, display_card.back).as_str(),
        &ComrakOptions::default(),
    )
}

#[tauri::command]
pub fn read_note(note: Note) -> Result<HashMap<String, String>, String> {
    match fs::read(note.get_path()) {
        Ok(f) => Ok(parse_note_into_fields(String::from_utf8(f).unwrap())),
        Err(err) => Err(err.to_string()),
    }
}

fn get_note_md(fields: HashMap<String, String>) -> String {
    let mut md = "".to_string();
    for (field, value) in fields.iter() {
        md = format!("{}# {}\n{}\n", md, field, value);
    }
    md
}

#[tauri::command]
pub fn create_note(mut note: Note, fields: HashMap<String, String>) -> Result<(), String> {
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();
    note.note_id = format!("{}_{}", time, note.template);

    match fs::write(note.get_path(), get_note_md(fields)) {
        Ok(..) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn update_note(note: Note, fields: HashMap<String, String>) -> Result<(), String> {
    match fs::write(note.get_path(), get_note_md(fields)) {
        Ok(..) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn list_notes(state: State<CollectionPath>, deck: &str) -> Result<Vec<Note>, String> {
    match fs::read_dir(deck::get_deck_path(deck, &state.0)) {
        Ok(paths) => Ok(get_notes_from_paths(deck, paths)),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn preview_note(
    fields: HashMap<String, String>,
    template: String,
    card_num: u32,
    back: bool,
) -> String {
    if back {
        render_back(fields, &template, card_num)
    } else {
        render_front(fields, &template, card_num)
    }
}

#[tauri::command]
pub fn render_note_card(note: Note, card_num: u32, back: bool) -> Result<String, String> {
    let fields = read_note(note.clone())?;
    if back {
        Ok(render_back(fields, &note.template, card_num))
    } else {
        Ok(render_front(fields, &note.template, card_num))
    }
}

#[cfg(test)]
mod tests {
    use crate::note::{preview_note, read_note, Note};
    use std::ops::Deref;
    use std::path::{Path, PathBuf};
    use std::{collections::HashMap, fs};
    use std::{env, io};

    use tempfile::TempDir;

    struct Fixture {
        path: PathBuf,
        source: PathBuf,
        _tempdir: TempDir,
    }

    pub fn copy_recursively(
        source: impl AsRef<Path>,
        destination: impl AsRef<Path>,
    ) -> io::Result<()> {
        fs::create_dir_all(&destination)?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let filetype = entry.file_type()?;
            if filetype.is_dir() {
                copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
            }
        }
        Ok(())
    }

    #[test]
    fn read_note_basic() {
        let root_dir = &env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
        let mut source = PathBuf::from(root_dir);
        source.push("tests/fixtures");
        source.push("basicdeck");

        let tempdir = tempfile::tempdir().unwrap();
        let mut path = PathBuf::from(&tempdir.path());
        env::set_var("COLLECTION_PATH", path.to_str().unwrap());
        path.push("basicdeck");
        copy_recursively(source, path).unwrap();

        let fields = read_note(Note {
            deck_id: "basicdeck".into(),
            note_id: "123".into(),
            template: "basic".into(),
        })
        .unwrap();

        assert_eq!("Question", fields.get("Front").unwrap());
        assert_eq!("Answer", fields.get("Back").unwrap());

        
        tempdir.close().unwrap();
    }

    #[test]
    fn preview_note_basic() {
        let mut fields = HashMap::<String, String>::new();
        fields.insert("Front".into(), "Front Text".into());
        fields.insert("Back".into(), "Back Text".into());

        let preview = preview_note(fields, "basic".into(), 1, true);

        assert_eq!("<p>Front Text</p>\n<hr />\n<p>Back Text</p>\n", preview);
    }

    #[test]
    fn preview_note_basic_front_only() {
        let mut fields = HashMap::<String, String>::new();
        fields.insert("Front".into(), "Front Text".into());
        fields.insert("Back".into(), "Back Text".into());

        let preview = preview_note(fields, "basic".into(), 1, false);

        assert_eq!("<p>Front Text</p>\n", preview);
    }

    #[test]
    fn preview_note_basic_no_back_data() {
        let mut fields = HashMap::<String, String>::new();
        fields.insert("Front".into(), "Front Text".into());

        let preview = preview_note(fields, "basic".into(), 1, true);

        assert_eq!("<p>Front Text</p>\n<hr />\n", preview);
    }
}
