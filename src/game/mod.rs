use blinds::{BlindType, Scaling, BASE_SCORES, GREEN_SCORES, PURPLE_SCORES};
use deck::{Deck, DeckType, PlasmaDeck};
use serde::{Deserialize, Serialize};
use serde_binary::{to_vec, from_vec, binary_stream::Endian};
use std::{any::{Any, TypeId}, collections::HashMap, env::{current_dir, current_exe}, fs, iter::Map};
use crate::{cards::Card, jokers::Joker};
mod blinds;
mod deck;

#[derive(Serialize, Deserialize)]
struct Game<'a, T: DeckType + Any> {
    jokers: Vec<Joker<'a>>,
    scaling: Scaling,
    deck: Deck::<T>,
    blinds: [BlindType; 3],
    game_data: GameData
}

impl<'a, T: DeckType + Any> Game<'a, T> {
    const K: f64 = 0.75;
    fn ante_base_score(&self) -> f64 {
        if self.game_data.ante < 1 {
            return 100.0;
        }
        if self.game_data.ante <= 8 {
            return match self.scaling {
                Scaling::Base => BASE_SCORES[self.game_data.ante],
                Scaling::Green => GREEN_SCORES[self.game_data.ante],
                Scaling::Purple => PURPLE_SCORES[self.game_data.ante],
            }
        }
        let a = match self.scaling {
            Scaling::Base => BASE_SCORES[8],
            Scaling::Green => GREEN_SCORES[8],
            Scaling::Purple => PURPLE_SCORES[8],
        } as f64;
        let b = 1.6;
        let c = self.game_data.ante as f64 - 8.0;
        let d = 1.0 + 0.2 * c;
        let mut amt = (a * (b + (Self::K * c).powf(d)).powf(c)).floor();
        amt -= amt.rem_euclid(10.0f64.powf((amt.log10()).floor() - 1.0));
        amt
    }
    fn blind_score_requirement(&self) -> f64 {
        self.blinds[self.game_data.current_blind as usize].score_requirement(self.ante_base_score())
        * if TypeId::of::<T>() == TypeId::of::<PlasmaDeck>() { 2.0 } else { 1.0 } 
    }

    fn init(&mut self) {
        self.deck.initialise_cards();
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct GameData {
    hands: usize,
    discards: usize,
    money: usize,
    round: usize,
    ante: usize,
    current_blind: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoundData {
    pub hands: usize,
    pub discards: usize,
    pub money: usize,
    pub ante: usize,
    pub blind: BlindType,
}