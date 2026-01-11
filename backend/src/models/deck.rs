use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::card::Card;

#[derive(Debug, Insertable, Deserialize, Selectable, Serialize, Queryable)]
#[diesel(table_name = crate::schema::deck)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Deck {
    pub id: i32,
    pub name: String,
    pub desired_retention: f32,
    /// w0, the initial stability assigned to a card when it is first and it is failed
    pub initial_stability_again_0: f32,
    /// w1, the initial stability assigned to a card when it is first and it is passed with higher
    /// than 1 to 2
    pub initial_stability_hard_1: f32,
    /// w2, the initial stability assigned to a card when it is first and it is passed with higher
    /// than 2 to 3
    pub initial_stability_good_2: f32,
    /// w3, the initial stability assigned to a card when it is first and it is
    /// passed with higher than 3 to 4
    pub initial_stability_easy_3: f32,
    /// Called w4 in FSRS: determines the initial difficulty of a card, alongside w5
    pub initial_difficulty_4: f32,
    /// Called w5 in FSRS: determines the initial difficulty of a card, alongside w4
    pub initial_difficulty_multiplier_5: f32,
    /// Called w6 in FSRS: determines how much the difficulty changes after each review
    pub difficulty_adjustment_6: f32,
    /// Called w7 in FSRS: mean regresion factor for difficulty
    pub difficulty_mean_regression_7: f32,
    /// Called w8 in FSRS: exponent for stability
    pub stability_exponent_8: f32,
    /// Called w9 in FSRS: negative power for stability
    pub stability_negative_power_9: f32,
    /// Called w10 in FSRS: exponent for stability
    pub stability_exponent_10: f32,
    /// Called w11 in FSRS: fail stability multiplier
    pub fail_stability_multiplier_11: f32,
    /// Called w12 in FSRS: negative power fail for stability
    pub fail_stability_negative_power_12: f32,
    /// Called w13 in FSRS: fail stability power
    pub fail_stability_power_13: f32,
    /// Called w14 in FSRS: fail stability exponent
    pub fail_stability_exponent_14: f32,
    /// Called w15 in FSRS: stability multiplier for grade between 1 to 2
    /// punishment added to retrievability when the card is marked between 2 and 3 (Hard or Good)
    /// Goes from 0.7 to 1
    pub hard_stability_multiplier_15: f32,
    /// Called w16 in FSRS: negative power stability for grade between 3 and 4
    /// Bonus added to retrievability when the card is marked between 3 and 4 (Good or Easy)
    /// The value itself, goes from 1 to 6
    pub easy_stability_multiplier_16: f32,
    /// Called w17 in FSRS: short term stability exponent
    pub short_term_stability_exponent_17: f32,
    /// Called w18 in FSRS: short term stability exponent 2
    pub short_term_stability_exponent_2_18: f32,
    /// Called w19 in FSRS: short term last stability exponent
    pub short_term_last_stability_exponent_19: f32,
    /// w20 in FSRS: determines how quickly intervals grow or shrink
    /// It goes from 0.1 to 0.8, for most users is 0.2
    /// Also is used to calculate the retrievability.
    /// Lower values will reduce the interval length. For example, with a Stability of 2, and a
    /// Decay factor of 0.2, the interval goes from 1.2 days to almost 2.
    /// It allows to correct the curve, essentially
    pub interval_decay_factor_20: f32,
}

impl Default for Deck {
    /// Values obtained from open-spaced-repetition.github.io/anki_fsrs_visualizer. I don't know if
    /// they are up to date
    fn default() -> Self {
        Deck {
            id: 0,
            name: "Default Deck".to_string(),
            desired_retention: 0.9,
            initial_stability_again_0: 0.212,
            initial_stability_hard_1: 1.2931,
            initial_stability_good_2: 2.3065,
            initial_stability_easy_3: 8.2956,
            initial_difficulty_4: 6.4133,
            initial_difficulty_multiplier_5: 0.8334,
            difficulty_adjustment_6: 3.0194,
            difficulty_mean_regression_7: 0.001,
            stability_exponent_8: 1.8722,
            stability_negative_power_9: 0.0614,
            stability_exponent_10: 0.796,
            fail_stability_multiplier_11: 1.4835,
            fail_stability_negative_power_12: 0.0614,
            fail_stability_power_13: 0.2629,
            fail_stability_exponent_14: 1.6483,
            hard_stability_multiplier_15: 0.6014,
            easy_stability_multiplier_16: 1.8729,
            short_term_stability_exponent_17: 0.5425,
            short_term_stability_exponent_2_18: 0.0912,
            short_term_last_stability_exponent_19: 0.0658,
            interval_decay_factor_20: 0.1542,
        }
    }
}

impl Deck {
    /// Get all cards belonging to this deck
    pub fn get_cards(&self, con: &mut PgConnection) -> Vec<Card> {
        use crate::schema::card::dsl::*;
        card.filter(deck_id.eq(self.id))
            .load::<Card>(con)
            .expect("Error loading cards")
    }

    pub fn calculate_next_training(&self, card: &mut Card, grade: u8) {
        card.stability = Some(card.stability.unwrap() + grade as f32);
    }

    /// Needed because update_difficulty requires a D(easy_start_diff)
    fn initial_difficulty_calculation(&self, grade: u8) -> f32 {
        self.initial_difficulty_4
            - (self.initial_difficulty_multiplier_5 * (5 - grade) as f32).exp()
    }

    /// Only for the first ever training of a card
    /// D_0 = initial_difficulty - e^(initial_difficulty_multiplier_5 * (5 - q))
    pub fn calculate_first_difficulty(&self, card: &mut Card, grade: u8) {
        if card.difficulty.is_some() {
            //TODO crash
            return;
        }
        card.difficulty = Some(
            self.initial_difficulty_4
                - (self.initial_difficulty_multiplier_5 * (5 - grade) as f32).exp(),
        );
    }

    pub fn update_difficulty(&self, card: &mut Card, grade: u8) {
        let current_difficulty = card.difficulty.unwrap();
        let difficulty_delta = self.difficulty_adjustment_6 * (grade as f32 - 3.0);
        // the second multiplication will be always 1 or less, so it makes the difficulty delta
        // smaller, to never allow it to reach 10 (the max diff)
        let linear_dampened_delta = difficulty_delta * (10.0 - current_difficulty) / 10.0;
        let new_difficulty = current_difficulty + linear_dampened_delta;
        // apply mean regression: The idea here is that if we grade it 3, the difficulty should
        // converge to its default value. This is important! the default difficulty is whatever the
        // "good" difficulty is, not the "easy" one.
        let mean_regressed_difficulty = new_difficulty * (1.0 - self.difficulty_mean_regression_7)
            + self.difficulty_mean_regression_7 * self.initial_difficulty_calculation(3)
            + (1.0 - self.difficulty_mean_regression_7) * new_difficulty;
        card.difficulty = Some(mean_regressed_difficulty);
    }
}

#[derive(Serialize)]
pub struct DeckWithCards {
    pub deck: Deck,
    pub card_ids: Vec<i32>,
}
