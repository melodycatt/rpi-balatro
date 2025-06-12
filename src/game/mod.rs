use blinds::{BlindType, Scaling, BASE_SCORES, GREEN_SCORES, PURPLE_SCORES};
use deck::{Deck, DeckType, PlasmaDeck};
use serde::{Deserialize, Serialize};
use std::{any::TypeId, collections::HashMap};
use crate::{cards::{Card, CardType, HandType}, jokers::{Joker, JokerType}};
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
        #![allow(unused_assignments)]
        let mut chips = 0.0;
        let mut mult = 0.0;
        #[warn(unused_assignments)]

        let splash = self.jokers.iter().any(|x| x.id() == "splash");

        self.round_data.selected.sort_by(|x, y| y.rank.cmp(&x.rank));

        let mut card_map = HashMap::new();
        for card in &self.round_data.selected {
            if card.enhancements.card_type == CardType::Stone { continue; }
            card_map.entry(card.rank)
                .and_modify(|e| *e += 1)
                .or_insert(1usize);
        }

        let count = self.round_data.selected.len();
        let flush = self.round_data.selected.iter().all(|x| x.suit == self.round_data.selected[0].suit);

        let mut selected_aggregated: Vec<_> = card_map.iter().map(|(k, v)| (*k, *v)).collect();
        selected_aggregated.sort_by(|x, y| y.1.cmp(&x.1));

        let hand_type = self.determine_hand(count, flush, &mut selected_aggregated);
        (chips, mult) = *self.game_data.hand_levels.get(&hand_type).expect("unknown hand");
        // Handle splash logic
        if splash {
            // Set chips/mult using detected hand type, if any
            
            // Score all selected cards
            for card in 0..self.round_data.selected.len() {
                self.score_card(&mut chips, &mut mult, &self.round_data.selected[card].clone());
            }

            let score = T::score(chips, mult);
            return Ok(score.0 * score.1);
        }

        // === Normal scoring logic ===
        if count == 1 {
            self.score_card(&mut chips, &mut mult, &self.round_data.hand[0].clone());
        } else if hand_type == HandType::Straight {
            (chips, mult) = *self.game_data.hand_levels.get(&hand_type).expect("unknown hand");
            for card in 0..self.round_data.selected.len() {
                self.score_card(&mut chips, &mut mult, &self.round_data.selected[card].clone());
            }
        } else {
            match selected_aggregated[0].1 {
                4 => {
                    (chips, mult) = *self.game_data.hand_levels.get(&HandType::FourOak).expect("unknown hand");
                    let scoring: Vec<_> = self.round_data.selected.iter().filter(|x| x.rank == selected_aggregated[0].0).cloned().collect();
                    for card in &scoring {
                        self.score_card(&mut chips, &mut mult, card);
                    }
                },
                3 => {
                    (chips, mult) = *self.game_data.hand_levels.get(&HandType::ThreeOak).expect("unknown hand");
                    let scoring: Vec<_> = self.round_data.selected.iter().filter(|x| x.rank == selected_aggregated[0].0).cloned().collect();
                    for card in &scoring {
                        self.score_card(&mut chips, &mut mult, card);
                    }
                },
                2 => {
                    if selected_aggregated.len() > 1 && selected_aggregated[1].1 == 2 {
                        (chips, mult) = *self.game_data.hand_levels.get(&HandType::TwoPair).expect("unknown hand");
                        let scoring: Vec<_> = self.round_data.selected.iter()
                            .filter(|x| x.rank == selected_aggregated[0].0 || x.rank == selected_aggregated[1].0)
                            .cloned().collect();
                        for card in &scoring {
                            self.score_card(&mut chips, &mut mult, card);
                        }
                    } else {
                        (chips, mult) = *self.game_data.hand_levels.get(&HandType::Pair).expect("unknown hand");
                        let scoring: Vec<_> = self.round_data.selected.iter().filter(|x| x.rank == selected_aggregated[0].0).cloned().collect();
                        for card in &scoring {
                            self.score_card(&mut chips, &mut mult, card);
                        }
                    }
                },
                1 => {
                    let card = *self.round_data.selected.iter()
                        .find(|x| x.enhancements.card_type != CardType::Stone).unwrap();
                    self.score_card(&mut chips, &mut mult, &card);
                },
                _ => unreachable!(),
            }

            let stones: Vec<Card> = self.round_data.selected.iter().filter(|x| x.enhancements.card_type == CardType::Stone).cloned().collect();
            for card in stones {
                self.score_card(&mut chips, &mut mult, &card);
            }
        }
        for i in 0..self.jokers.len() {
            let x = self.jokers[i];
            x.apply(&mut chips, &mut mult, self, hand_type);
        }

        let score = T::score(chips, mult);
        Ok(score.0 * score.1)
    }

    fn determine_hand(&mut self, count: usize, flush: bool, selected_aggregated: &mut Vec<(u8, usize)>) -> HandType {
        if count == 1 {
            return HandType::HighCard;
        }

        if count == 5 && selected_aggregated.iter().all(|x| x.1 == 1) &&
        self.round_data.selected.windows(2).all(|w| 
                (w[0].rank == w[1].rank + 1) || 
                (w[1].rank == 13 && w[0].rank == 1)) 
        {
            let mut hand = HandType::Straight;
            if flush { hand = hand.flush(); }
            return hand;
        }

        match selected_aggregated[0].1 {
            5 => if flush { HandType::FiveOak.flush() } else { HandType::FiveOak },
            4 => if flush { HandType::FourOak.flush() } else { HandType::FourOak },
            3 => {
                if selected_aggregated.len() > 1 && selected_aggregated[1].1 == 2 {
                    if flush { HandType::FullHouse.flush() } else { HandType::FullHouse }
                } else {
                    if flush { HandType::ThreeOak.flush() } else { HandType::ThreeOak }
                }
            },
            2 => {
                if selected_aggregated.len() > 1 && selected_aggregated[1].1 == 2 {
                    if flush { HandType::TwoPair.flush() } else { HandType::TwoPair }
                } else {
                    if flush { HandType::Pair.flush() } else { HandType::Pair }
                }
            },
            1 => HandType::HighCard,
            _ => unreachable!(),
        }
    }

    // Helper to score and update deck
    fn score_card (&mut self, chips: &mut f64, mult: &mut f64, card: &Card) {
        let mut c = card.clone();
        let ci = self.deck.remaining_cards.iter().position(|x| *x == c).unwrap();
        c.score(chips, mult, self);
        self.deck.remaining_cards[ci].count -= 1;
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
    pub hand_levels: HashMap<HandType, (f64, f64)>
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
            current_blind: 0,
            hand_levels: Self::init_hand_levels()
        }
    }
    fn init_hand_levels() -> HashMap<HandType, (f64, f64)> {
        let mut map = HashMap::with_capacity(13);
        map.insert(HandType::HighCard, (5.0, 1.0));
        map.insert(HandType::Pair, (10.0, 2.0));
        map.insert(HandType::TwoPair, (20.0, 2.0));
        map.insert(HandType::ThreeOak, (30.0, 3.0));
        map.insert(HandType::Straight, (30.0, 4.0));
        map.insert(HandType::Flush, (35.0, 4.0));
        map.insert(HandType::FullHouse, (40.0, 4.0));
        map.insert(HandType::FourOak, (60.0, 7.0));
        map.insert(HandType::StraightFlush, (100.0, 8.0));
        map.insert(HandType::FiveOak, (120.0, 12.0));
        map.insert(HandType::FlushHouse, (140.0, 14.0));
        map.insert(HandType::FlushFive, (160.0, 16.0));
        map
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