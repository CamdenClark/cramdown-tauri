use serde::{Deserialize, Serialize};

use chrono::{DateTime, Duration, Utc};

// TODO: Reconsider the interfaces that are used to
// render notes -- should probably just have a function
// that takes a note and returns all the cards generated
// from that note
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReviewState {
    New,
    Learned,
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
    interval: f64,
    ease: f64,
    state: ReviewState,
    steps: u32,
}

impl Review {
    pub fn new(
        due: Option<DateTime<Utc>>,
        interval: f64,
        ease: f64,
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
const EASY_BONUS: f64 = 1.3;
const GRADUATION_INTERVAL: u32 = 1;
const NEW_STEPS: [u32; 2] = [1, 10];

const RELEARNING_STEPS: [u32; 1] = [10];

const MINIMUM_INTERVAL: f64 = 1.0;
const MINIMUM_EASE: f64 = 1.3;

pub fn score_card(mut review: Review, time: DateTime<Utc>, score: ReviewScore) -> Review {
    match review.state {
        ReviewState::New => match score {
            ReviewScore::Easy => {
                review.state = ReviewState::Learned;
                review.interval = EASY_INTERVAL.into();
                if let Some(due) = time.checked_add_signed(Duration::days(GRADUATION_INTERVAL.into())) {
                    review.due = Some(due);
                }
                review.steps = 0;
                review
            }
            ReviewScore::Again => {
                review.steps = 0;
                if let Some(due) = time.checked_add_signed(Duration::minutes(NEW_STEPS[0].into())) {
                    review.due = Some(due);
                }
                review
            }
            ReviewScore::Hard => {
                if let Some(due) = time.checked_add_signed(Duration::minutes(1)) {
                    review.due = Some(due);
                }
                review
            }
            ReviewScore::Good => {
                review.steps += 1;
                if review.steps >= NEW_STEPS.len().try_into().unwrap() {
                    review.state = ReviewState::Learned;
                    review.steps = 0;
                    if let Some(due) =
                        time.checked_add_signed(Duration::days(GRADUATION_INTERVAL.into()))
                    {
                        review.due = Some(due);
                    }
                } else {
                    if let Some(due) = time.checked_add_signed(Duration::minutes(NEW_STEPS[1].into())) {
                        review.due = Some(due);
                    }
                }
                review
            }
        },
        ReviewState::Learned => match score {
            ReviewScore::Good => {
                review.interval *= review.ease;
                if let Some(due) = time.checked_add_signed(Duration::seconds(
                    (review.interval * 24.0 * 60.0 * 60.0).round() as i64,
                )) {
                    review.due = Some(due);
                }
                review
            }
            ReviewScore::Easy => {
                review.interval *= review.ease * EASY_BONUS;
                review.ease += 0.15;
                if let Some(due) = time.checked_add_signed(Duration::seconds(
                    (review.interval * 24.0 * 60.0 * 60.0).round() as i64,
                )) {
                    review.due = Some(due);
                }
                review
            }
            ReviewScore::Hard => {
                review.interval *= 1.2;
                review.ease = (review.ease - 0.15).max(MINIMUM_EASE);
                if let Some(due) = time.checked_add_signed(Duration::seconds(
                    (review.interval * 24.0 * 60.0 * 60.0).round() as i64,
                )) {
                    review.due = Some(due);
                }
                review
            }
            ReviewScore::Again => {
                review.state = ReviewState::Relearning;
                review.steps = 0;
                review.interval = (review.interval * 0.7).max(MINIMUM_INTERVAL);
                review.ease = (review.ease - 0.2).max(MINIMUM_EASE);
                if let Some(due) = time.checked_add_signed(Duration::minutes(RELEARNING_STEPS[0].into())) {
                    review.due = Some(due);
                }
                review
            }
        },
        ReviewState::Relearning => match score {
            ReviewScore::Good | ReviewScore::Easy => {
                review.state = ReviewState::Learned;
                if let Some(due) = time.checked_add_signed(Duration::seconds(
                    (review.interval * 24.0 * 60.0 * 60.0).round() as i64,
                )) {
                    review.due = Some(due);
                }
                review
            }
            ReviewScore::Hard | ReviewScore::Again=> {
                review.steps = 0;
                if let Some(due) = time.checked_add_signed(Duration::minutes(RELEARNING_STEPS[0].into())) {
                    review.due = Some(due);
                }
                review
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::review::{
        score_card, Review, ReviewScore, ReviewState, EASY_BONUS, GRADUATION_INTERVAL, NEW_STEPS, RELEARNING_STEPS
    };
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
            assert_eq!(
                review.state, $expected_state,
                "Test: {}\nIssue: Card state doesn't match",
                $message
            );
            assert_eq!(
                review.due.unwrap().signed_duration_since(time),
                $expected_duration,
                "Test: {}\nIssue: Duration doesn't match",
                $message
            );
            assert_eq!(
                review.interval, $expected_interval,
                "Test: {}\nIssue: Interval doesn't match",
                $message
            );
            assert_eq!(
                review.ease, $expected_ease,
                "Test: {}\nIssue: Ease factor doesn't match",
                $message
            );
            assert_eq!(
                review.steps, $expected_steps,
                "Test: {}\nIssue: Review steps don't match",
                $message
            );
        };
    }

    #[test]
    fn new_card_scored_easy() {
        test_card!(
            "New card scored easy should graduate immediately",
            1.0,
            2.5,
            0,
            ReviewState::New,
            ReviewScore::Easy,
            4.0,
            2.5,
            0,
            Duration::days(GRADUATION_INTERVAL.into()),
            ReviewState::Learned
        );
    }

    #[test]
    fn new_card_scored_again() {
        test_card!(
            "New card scored again should reset steps",
            1.0,
            2.5,
            1,
            ReviewState::New,
            ReviewScore::Again,
            1.0,
            2.5,
            0,
            Duration::minutes(NEW_STEPS[0].into()),
            ReviewState::New
        );
    }

    #[test]
    fn new_card_scored_hard() {
        test_card!(
            "New card scored hard should reset steps",
            1.0,
            2.5,
            0,
            ReviewState::New,
            ReviewScore::Hard,
            1.0,
            2.5,
            0,
            Duration::minutes(1),
            ReviewState::New
        );
    }

    #[test]
    fn new_card_scored_good() {
        test_card!(
            "New card scored good should go up one in steps, but stay in new state",
            1.0,
            2.5,
            0,
            ReviewState::New,
            ReviewScore::Good,
            1.0,
            2.5,
            1,
            Duration::minutes(NEW_STEPS[1].into()),
            ReviewState::New
        );
        test_card!(
            "New card scored good with one step should graduate",
            1.0,
            2.5,
            1,
            ReviewState::New,
            ReviewScore::Good,
            1.0,
            2.5,
            0,
            Duration::days(GRADUATION_INTERVAL.into()),
            ReviewState::Learned
        );
    }

    #[test]
    fn learned_card_scored_good() {
        test_card!(
            "Learned card scored good should keep same ease factor, extend interval",
            1.0,
            2.5,
            0,
            ReviewState::Learned,
            ReviewScore::Good,
            2.5,
            2.5,
            0,
            Duration::days(2) + Duration::hours(12),
            ReviewState::Learned
        );
        test_card!(
            "Learned card scored good with 1.5 ease factor should be reviewed in 1.5 days",
            1.0,
            1.5,
            0,
            ReviewState::Learned,
            ReviewScore::Good,
            1.5,
            1.5,
            0,
            Duration::days(1) + Duration::hours(12),
            ReviewState::Learned
        );
    }

    #[test]
    fn learned_card_scored_easy() {
        test_card!(
            "Learned card scored good should keep same ease factor, extend interval",
            1.0,
            2.5,
            0,
            ReviewState::Learned,
            ReviewScore::Easy,
            3.25,
            2.65,
            0,
            Duration::seconds((1.0 * EASY_BONUS * 2.5 * 60.0 * 60.0 * 24.0).round() as i64),
            ReviewState::Learned
        );
    }

    #[test]
    fn learned_card_scored_hard() {
        test_card!(
            "Learned card scored hard should multiply interval by 1.2, decrease ease score by .15",
            1.0,
            2.5,
            0,
            ReviewState::Learned,
            ReviewScore::Hard,
            1.2,
            2.35,
            0,
            Duration::seconds((1.2 * 60.0 * 60.0 * 24.0 as f64).round() as i64),
            ReviewState::Learned
        );

        test_card!(
            "Learned card scored hard should not go below 1.3 ease factor",
            1.0,
            1.3,
            0,
            ReviewState::Learned,
            ReviewScore::Hard,
            1.2,
            1.3,
            0,
            Duration::seconds((1.2 * 60.0 * 60.0 * 24.0 as f64).round() as i64),
            ReviewState::Learned
        );
    }

    #[test]
    fn learned_card_scored_again() {
        test_card!(
            "Learned card scored again should multiply interval by 1.2, decrease ease score by .15",
            2.0,
            2.5,
            0,
            ReviewState::Learned,
            ReviewScore::Again,
            1.4,
            2.3,
            0,
            Duration::minutes(10),
            ReviewState::Relearning
        );

        test_card!(
            "Learned card scored again should not go below 1.3 ease factor",
            2.0,
            1.3,
            0,
            ReviewState::Learned,
            ReviewScore::Again,
            1.4,
            1.3,
            0,
            Duration::minutes(10),
            ReviewState::Relearning
        );
    }

    #[test]
    fn relearning_card_scored_again() {
        test_card!(
            "Relearning card scored again should reset steps, review again in a minute",
            1.0,
            2.5,
            1,
            ReviewState::Relearning,
            ReviewScore::Again,
            1.0,
            2.5,
            0,
            Duration::minutes(RELEARNING_STEPS[0].into()),
            ReviewState::Relearning
        );
    }

    #[test]
    fn relearning_card_scored_hard() {
        test_card!(
            "Relearning card scored again should reset steps, review again in a minute",
            1.0,
            2.5,
            1,
            ReviewState::Relearning,
            ReviewScore::Hard,
            1.0,
            2.5,
            0,
            Duration::minutes(RELEARNING_STEPS[0].into()),
            ReviewState::Relearning
        );
    }
    
    #[test]
    fn relearning_card_scored_good() {
        test_card!(
            "Relearning card scored good should return card to learned state",
            1.0,
            2.5,
            0,
            ReviewState::Relearning,
            ReviewScore::Good,
            1.0,
            2.5,
            0,
            Duration::days(1),
            ReviewState::Learned
        );
    }

    #[test]
    fn relearning_card_scored_easy() {
        test_card!(
            "Relearning card scored easy should return card to learned state",
            1.0,
            2.5,
            0,
            ReviewState::Relearning,
            ReviewScore::Easy,
            1.0,
            2.5,
            0,
            Duration::days(1),
            ReviewState::Learned
        );
    }
}
