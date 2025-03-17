use blinds::{BlindType, Scaling, BASE_SCORES, GREEN_SCORES, PURPLE_SCORES};
use deck::Deck;
use serde::{Deserialize, Serialize};
use serde_binary::{to_vec, from_vec, binary_stream::Endian};
use std::{collections::HashMap, env::{current_dir, current_exe}, fs, iter::Map};
use crate::{cards::Card, jokers::Joker};
mod blinds;
mod deck;

#[derive(Serialize, Deserialize)]
struct Game<'a> {
    jokers: Vec<Joker<'a>>,
    cards: Vec<Card>,
    ante: usize,
    round: usize,
    money: usize,
    scaling: Scaling,
    deck: Deck,
    blinds: [BlindType; 3],
    current_blind: u8
}

impl<'a> Game<'a> {
    const K: f64 = 0.75;
    fn ante_base_score(&self) -> f64 {
        if self.ante < 1 {
            return 100.0;
        }
        if self.ante <= 8 {
            return match self.scaling {
                Scaling::Base => BASE_SCORES[self.ante],
                Scaling::Green => GREEN_SCORES[self.ante],
                Scaling::Purple => PURPLE_SCORES[self.ante],
            }
        }
        let a = match self.scaling {
            Scaling::Base => BASE_SCORES[8],
            Scaling::Green => GREEN_SCORES[8],
            Scaling::Purple => PURPLE_SCORES[8],
        } as f64;
        let b = 1.6;
        let c = self.ante as f64 - 8.0;
        let d = 1.0 + 0.2 * c;
        let mut amt = (a * (b + (Self::K * c).powf(d)).powf(c)).floor();
        amt -= amt.rem_euclid(10.0f64.powf((amt.log10()).floor() - 1.0));
        amt
    }
    fn blind_score_requirement(&self) -> f64 {
        self.blinds[self.current_blind as usize].score_requirement(self.ante_base_score())
    }

    fn init() {
        
    }
}