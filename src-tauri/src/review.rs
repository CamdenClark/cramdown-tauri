use std::fs;
use std::fs::ReadDir;
use std::io::prelude::*;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use regex::Regex;

use chrono::{DateTime, Duration, Utc};

use crate::deck;
// TODO: Reconsider the interfaces that are used to
// render notes -- should probably just have a function
// that takes a note and returns all the cards generated
// from that note
use crate::card::{Card, CardState};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReviewScore {
    Again,
    Hard,
    Good,
    Easy,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Review {
    note_id: String,
    card_num: u32,
    due: DateTime<Utc>,
    interval: u32,
    ease: u32,
    last_interval: u32,
    state: CardState,
    score: ReviewScore,
    steps: u32,
}

// What if we have a really clean review module
// that just returns an updated card? What's the difference
// really between a card at a certain state and its review?
// Having the coupled logic here with note id, card num, etc.
// makes it harder to reason about the review contents abstractly
// as well
impl From<Card> for Review {
    fn from(card: Card) -> Self {
        Review {
            note_id: card.note_id,
            card_num: card.card_num,
            due: Utc::now(),
            interval: card.interval,
            ease: card.ease,
            last_interval: card.interval,
            state: CardState::New,
            score: score.clone(),
            steps: card.steps,
        }
    }
}
        

const EASY_INTERVAL: u32 = 4;
const GRADUATION_INTERVAL: u32 = 1;
const AGAIN_STEPS: u32 = 2;

pub fn get_review_path(card: Card) -> PathBuf {
    deck::get_deck_path(&card.deck_id)
        .join("reviews")
        .join(format!("{}.jsonl", &card.note_id))
}

pub fn score_card(card: Card, time: DateTime<Utc>, score: ReviewScore) -> Review {
    let mut review = Review {
        note_id: card.note_id,
        card_num: card.card_num,
        due: Utc::now(),
        interval: card.interval,
        ease: card.ease,
        last_interval: card.interval,
        state: CardState::New,
        score: score.clone(),
        steps: card.steps,
    };
    match card.state {
        CardState::New => match score {
            ReviewScore::Easy => {
                review.state = CardState::Graduated;
                review.interval = EASY_INTERVAL;
                if let Some(due) = time.checked_add_signed(Duration::days(EASY_INTERVAL.into())) {
                    review.due = due;
                }
                review.steps = 0;
                review
            }
            ReviewScore::Again => {
                review.steps = AGAIN_STEPS;
                if let Some(due) = time.checked_add_signed(Duration::minutes(1)) {
                    review.due = due;
                }
                review
            }
            ReviewScore::Hard => {
                review.steps = AGAIN_STEPS;
                if let Some(due) = time.checked_add_signed(Duration::minutes(1)) {
                    review.due = due;
                }
                review
            }
            ReviewScore::Good => {
                if card.steps <= 1 {
                    review.state = CardState::Graduated;
                    review.steps = 0;
                    if let Some(due) =
                        time.checked_add_signed(Duration::days(GRADUATION_INTERVAL.into()))
                    {
                        review.due = due;
                    }
                } else {
                    if let Some(due) = time.checked_add_signed(Duration::minutes(1)) {
                        review.due = due;
                    }
                    review.steps -= 1;
                }
                review
            }
        },
        _ => review,
    }
}

#[cfg(test)]
mod tests {
    use crate::card::{Card, CardState};
    use crate::review::{score_card, ReviewScore, GRADUATION_INTERVAL};
    use chrono::{Duration, Utc};

    macro_rules! test_card {
        ($message:literal, $interval:literal, $ease:literal, $steps:literal, $state:expr,
     $score:expr,
     $expected_interval:literal, $expected_ease:literal, $expected_steps:literal, $expected_duration:expr, $expected_state:expr) => {
            let card = Card {
                note_id: String::from("test"),
                card_num: 1,
                interval: $interval,
                ease: $ease,
                steps: $steps,
                template: String::from("basic"),
                due: Some(Utc::now()),
                deck_id: String::from("test"),
                state: $state,
            };
            let time = Utc::now();
            let review = score_card(card, time, $score);
            assert_eq!(review.state, $expected_state, "Test: {}\nIssue: Card state doesn't match", $message);
            assert_eq!(review.due.signed_duration_since(time), $expected_duration, "Test: {}\nIssue: Duration doesn't match", $message);
            assert_eq!(review.interval, $expected_interval, "Test: {}\nIssue: Interval doesn't match", $message);
            assert_eq!(review.ease, $expected_ease, "Test: {}\nIssue: Ease factor doesn't match", $message);
            assert_eq!(review.steps, $expected_steps, "Test: {}\nIssue: Review steps don't match", $message);
        };
    }

    #[test]
    fn new_card_scored_easy() {
        test_card!(
            "New card scored easy should graduate immediately",
            1,
            250,
            0,
            CardState::New,
            ReviewScore::Easy,
            4,
            250,
            0,
            Duration::days(4),
            CardState::Graduated
        );
    }

    #[test]
    fn new_card_scored_again() {
        test_card!(
            "New card scored again should reset steps",
            1,
            250,
            0,
            CardState::New,
            ReviewScore::Again,
            1,
            250,
            2,
            Duration::minutes(1),
            CardState::New
        );
    }

    #[test]
    fn new_card_scored_hard() {
        test_card!(
            "New card scored hard should reset steps",
            1,
            250,
            0,
            CardState::New,
            ReviewScore::Hard,
            1,
            250,
            2,
            Duration::minutes(1),
            CardState::New
        );
    }

    #[test]
    fn new_card_scored_good() {
        test_card!(
            "New card scored good should graduate",
            1,
            250,
            0,
            CardState::New,
            ReviewScore::Good,
            1,
            250,
            0,
            Duration::days(GRADUATION_INTERVAL.into()),
            CardState::Graduated
        );
        test_card!(
            "New card scored good with one step should graduate",
            1,
            250,
            1,
            CardState::New,
            ReviewScore::Good,
            1,
            250,
            0,
            Duration::days(GRADUATION_INTERVAL.into()),
            CardState::Graduated
        );
        test_card!(
            "New card scored good with 2 steps should remove a step but stay in new",
            1,
            250,
            2,
            CardState::New,
            ReviewScore::Good,
            1,
            250,
            1,
            Duration::minutes(1),
            CardState::New
        );
    }
}

#[tauri::command]
pub fn review_card(card: Card, _score: ReviewScore) -> Result<String, String> {
    match fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(get_review_path(card.clone().into()))
    {
        Ok(mut file) => match file.write_all(&serde_json::to_vec(&card).unwrap()) {
            Ok(..) => Ok("".to_string()),
            Err(..) => Err("".to_string()),
        },
        Err(..) => Err("".to_string()),
    }
}
