use blinds::{BlindType, Scaling, BASE_SCORES, GREEN_SCORES, PURPLE_SCORES};
use deck::{Deck, DeckType, PlasmaDeck};
use serde::{Deserialize, Serialize};
use std::{any::TypeId, collections::HashMap};
use crate::{cards::{Card, CardKey, CardType}, jokers::Joker};
pub mod blinds;
pub mod deck;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Stake {
    White
    //todo!()
} impl Stake {
    pub fn scaling(self) -> Scaling { todo!() }
}
#[derive(Serialize, Deserialize)]
pub struct Game<T: DeckType + 'static> {
    pub jokers: Vec<Joker>,
    pub stake: Stake,
    scaling: Scaling,
    pub deck: Deck::<T>,
    pub blinds: [BlindType; 3],
    pub game_data: GameData,
    pub round_data: RoundData
}

impl<T: DeckType + 'static> Game<T> {
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

    pub fn play(&mut self) -> Result<f64, NoCards> {
        let mut chips = 0.0;
        let mut mult = 0.0;
        self.round_data.selected.sort_by(|x, y| y.rank.cmp(&x.rank));
        let mut card_map = HashMap::<CardKey, Card>::new();
        for i in 0..self.round_data.selected.len() {
            let key = CardKey::from(self.round_data.selected[i]);
            card_map.entry(key)
                .and_modify(|e| e.count += 1)
                .or_insert(self.round_data.selected[i]);
        }
        self.round_data.selected = card_map.values().cloned().collect();
        let count = self.round_data.selected.iter().fold(0, |acc, x| acc + x.count);
        if count == 1 { 
            let d = self.round_data.selected[0].clone().score(&mut chips, &mut mult, self); 
            self.deck.remaining_cards.iter_mut().find(|x| x.1 == self.round_data.selected[0]).unwrap().0 += self.round_data.selected[0].count - 2;
            if d { 
                self.deck.cards.iter_mut().find(|x| **x == self.round_data.selected[0]).unwrap().count -= 1;
            }
        }
        for card in card_map.values() {
            if card.enhancements.card_type == CardType::Stone {
                card.score(chips, mult, game)
            }
        }
        let score = T::score(chips, mult);
        Ok(score.0 * score.1)
    }

    pub fn draw(&mut self) {
        let n = if self.round_data.blind == BlindType::Serpent { 3 } else { self.hand_size() - self.round_data.hand.len() };
        for _ in 0..n {
            self.round_data.hand.push(self.deck.pick());
        }
    }

    pub fn hand_score(&mut self) -> (f64, f64) {
        todo!()
    }
    
    pub fn hand_size(&self) -> usize {
        (8 + T::CONFIG.hand_size) as usize
    }

    pub fn new(stake: Stake) -> Game<T> {
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

pub struct NoCards;
impl std::fmt::Display for NoCards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "no cards selected")
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
    pub hand: Vec<Card>,
    pub selected: Vec<Card>
}
impl RoundData {
    fn new<T: DeckType>() -> Self {
        Self {
            hands: (4 + T::CONFIG.hands) as usize,
            discards: (3 + T::CONFIG.discards) as usize,
            blind: BlindType::Small,
            hand: Vec::new(),
            selected: Vec::new()
        }
    }
    fn new_round<T: DeckType>(game: &Game<T>) -> Self {
        Self {
            hands: game.game_data.hands,
            discards: game.game_data.discards,
            blind: game.blinds[game.game_data.current_blind],
            hand: Vec::new(),
            selected: Vec::new()
        }
    }
}