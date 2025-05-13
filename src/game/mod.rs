use blinds::{BlindType, Scaling, BASE_SCORES, GREEN_SCORES, PURPLE_SCORES};
use deck::{Deck, DeckType, PlasmaDeck};
use rand::rng;
use serde::{Deserialize, Serialize};
use serde_binary::{to_vec, from_vec, binary_stream::Endian};
use std::{any::{Any, TypeId}, collections::HashMap, env::{current_dir, current_exe}, fs, iter::Map};
use crate::{cards::Card, jokers::Joker};
mod blinds;
mod deck;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Stake {
    White
    //todo!()
} impl Stake {
    pub fn scaling(self) -> Scaling { todo!() }
}
#[derive(Serialize, Deserialize)]
pub struct Game<'a, T: DeckType + 'static> {
    pub jokers: Vec<Joker<'a>>,
    pub stake: Stake,
    scaling: Scaling,
    pub deck: Deck::<T>,
    pub blinds: [BlindType; 3],
    pub game_data: GameData,
    pub round_data: RoundData
}

impl<'a, T: DeckType + 'static> Game<'a, T> {
    const K: f64 = 0.75;
    pub fn ante_base_score(&self) -> f64 {
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
    pub fn blind_score_requirement(&self) -> f64 {
        self.blinds[self.game_data.current_blind as usize].score_requirement(self.ante_base_score())
        * if TypeId::of::<T>() == TypeId::of::<PlasmaDeck>() { 2.0 } else { 1.0 } 
    }

    pub fn init(&mut self) {
        self.deck.initialise_cards();
    }

    pub fn next_round(&mut self) {
        self.game_data.current_blind += 1;
        self.game_data.current_blind %= 3;
        if self.game_data.current_blind == 0 { self.game_data.ante += 1; /* self.generate_blinds() */ todo!() }
        self.round_data = RoundData::new_round(&self);
        self.draw();
    }

    pub fn draw(&mut self) {
        for _ in 0..(self.hand_size() - self.round_data.hand.len()) {
            self.round_data.hand.push(self.deck.pick());
        }
    }
    
    pub fn hand_size(&self) -> usize {
        (8 + T::CONFIG.hand_size) as usize
    }

    pub fn new(stake: Stake) -> Game<'a, T> {
        Self {
            jokers: vec![],
            stake,
            scaling: stake.scaling(),
            deck: Deck::<T>::new(),
            blinds: [BlindType::Small, BlindType::Big, BlindType::Wall],
            game_data: GameData::new::<T>(),
            round_data: RoundData::new::<T>()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameData {
    pub hands: usize,
    pub discards: usize,
    pub money: usize,
    pub round: usize,
    pub ante: usize,
    pub hand_size: usize,
    pub current_blind: usize,
}
impl GameData {
    fn new<T: DeckType>() -> Self {
        Self {
            hands: (4 + T::CONFIG.hands) as usize,
            discards: (3 + T::CONFIG.discards) as usize,
            money: (4 + T::CONFIG.money) as usize,
            hand_size: (8 + T::CONFIG.hand_size) as usize,
            round: 0,
            ante: 1,
            current_blind: 0
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoundData {
    pub hands: usize,
    pub discards: usize,
    pub blind: BlindType,
    pub hand: Vec<Card>
}
impl RoundData {
    fn new<T: DeckType>() -> Self {
        Self {
            hands: (4 + T::CONFIG.hands) as usize,
            discards: (3 + T::CONFIG.discards) as usize,
            blind: BlindType::Small,
            hand: Vec::new()
        }
    }
    fn new_round<T: DeckType>(game: &Game<T>) -> Self {
        Self {
            hands: game.game_data.hands,
            discards: game.game_data.discards,
            blind: game.blinds[game.game_data.current_blind],
            hand: Vec::new()
        }
    }
}