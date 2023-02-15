use serde::{Deserialize, Serialize};

use chrono::{DateTime, Duration, Utc};

// TODO: Reconsider the interfaces that are used to
// render notes -- should probably just have a function
// that takes a note and returns all the cards generated
// from that note
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReviewState {
    New,
    Graduated,
    Relearning,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReviewScore {
    Again,
    Hard,
    Good,
    Easy,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Review {
    due: Option<DateTime<Utc>>,
    interval: u32,
    ease: u32,
    state: ReviewState,
    steps: u32,
}

impl Review {
    pub fn new(
    due: Option<DateTime<Utc>>,
    interval: u32,
    ease: u32,
    state: ReviewState,
    steps: u32,
        ) -> Self {
        Review {
    due,
    interval,
    ease,
    state,
    steps,
        }

        
    }
}


const EASY_INTERVAL: u32 = 4;
const GRADUATION_INTERVAL: u32 = 1;
const AGAIN_STEPS: u32 = 2;

pub fn score_card(mut review: Review, time: DateTime<Utc>, score: ReviewScore) -> Review {
    match review.state {
        ReviewState::New => match score {
            ReviewScore::Easy => {
                review.state = ReviewState::Graduated;
                review.interval = EASY_INTERVAL;
                if let Some(due) = time.checked_add_signed(Duration::days(EASY_INTERVAL.into())) {
                    review.due = Some(due);
                }
                review.steps = 0;
                review
            }
            ReviewScore::Again => {
                review.steps = AGAIN_STEPS;
                if let Some(due) = time.checked_add_signed(Duration::minutes(1)) {
                    review.due = Some(due);
                }
                review
            }
            ReviewScore::Hard => {
                review.steps = AGAIN_STEPS;
                if let Some(due) = time.checked_add_signed(Duration::minutes(1)) {
                    review.due = Some(due);
                }
                review
            }
            ReviewScore::Good => {
                if review.steps <= 1 {
                    review.state = ReviewState::Graduated;
                    review.steps = 0;
                    if let Some(due) =
                        time.checked_add_signed(Duration::days(GRADUATION_INTERVAL.into()))
                    {
                        review.due = Some(due);
                    }
                } else {
                    if let Some(due) = time.checked_add_signed(Duration::minutes(1)) {
                        review.due = Some(due);
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
    use crate::review::{score_card, Review, ReviewState, ReviewScore, GRADUATION_INTERVAL};
    use chrono::{Duration, Utc};

    macro_rules! test_card {
        ($message:literal, $interval:literal, $ease:literal, $steps:literal, $state:expr,
     $score:expr,
     $expected_interval:literal, $expected_ease:literal, $expected_steps:literal, $expected_duration:expr, $expected_state:expr) => {
            let card = Review {
                interval: $interval,
                ease: $ease,
                steps: $steps,
                due: Some(Utc::now()),
                state: $state,
            };
            let time = Utc::now();
            let review = score_card(card, time, $score);
            assert_eq!(review.state, $expected_state, "Test: {}\nIssue: Card state doesn't match", $message);
            assert_eq!(review.due.unwrap().signed_duration_since(time), $expected_duration, "Test: {}\nIssue: Duration doesn't match", $message);
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
            ReviewState::New,
            ReviewScore::Easy,
            4,
            250,
            0,
            Duration::days(4),
            ReviewState::Graduated
        );
    }

    #[test]
    fn new_card_scored_again() {
        test_card!(
            "New card scored again should reset steps",
            1,
            250,
            0,
            ReviewState::New,
            ReviewScore::Again,
            1,
            250,
            2,
            Duration::minutes(1),
            ReviewState::New
        );
    }

    #[test]
    fn new_card_scored_hard() {
        test_card!(
            "New card scored hard should reset steps",
            1,
            250,
            0,
            ReviewState::New,
            ReviewScore::Hard,
            1,
            250,
            2,
            Duration::minutes(1),
            ReviewState::New
        );
    }

    #[test]
    fn new_card_scored_good() {
        test_card!(
            "New card scored good should graduate",
            1,
            250,
            0,
            ReviewState::New,
            ReviewScore::Good,
            1,
            250,
            0,
            Duration::days(GRADUATION_INTERVAL.into()),
            ReviewState::Graduated
        );
        test_card!(
            "New card scored good with one step should graduate",
            1,
            250,
            1,
            ReviewState::New,
            ReviewScore::Good,
            1,
            250,
            0,
            Duration::days(GRADUATION_INTERVAL.into()),
            ReviewState::Graduated
        );
        test_card!(
            "New card scored good with 2 steps should remove a step but stay in new",
            1,
            250,
            2,
            ReviewState::New,
            ReviewScore::Good,
            1,
            250,
            1,
            Duration::minutes(1),
            ReviewState::New
        );
    }
}

