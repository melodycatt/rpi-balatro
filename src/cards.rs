use rand::Rng;
//serde again, but also num_enum
//this is to avoid problems with serializing a special type called an enum
//it allows you to convert enums into primitive number types
//which is quick and easy to serialize
//whereas (for whatever reason) serde_binary serializes them as strings
//which (for whatever reason) caused deserialization panics
use serde::{Deserialize, Serialize};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{game::{deck::DeckType, Game}, jokers::JokerType};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Card {
    pub enhancements: CardEnhancements,
    pub rank: u8,
    pub suit: CardSuit,
    pub count: usize,
}
impl Card {
    pub fn new_default(suit: CardSuit, rank: u8) -> Self {
        Self {
            enhancements: CardEnhancements::default(),
            rank,
            suit,
            count: 1,
        }
    }
    pub fn new(suit: CardSuit, rank: u8, enhancements: CardEnhancements) -> Self {
        Self {
            enhancements,
            rank,
            suit,
            count: 1,
        }
    }

    pub fn with_count(mut self, count: usize) -> Self {
        self.count = count;
        self
    }
    pub fn score<T: DeckType>(&mut self, chips: &mut f64, mult: &mut f64, game: &mut Game<T>) {
        let i = game.deck.cards.iter().position(|x| *x == *self).unwrap();
        *chips += self.rank as f64;
        let mut rng = rand::rng();
        let mut destroyed = false;
        for j in 0..game.jokers.len() {
            let j = game.jokers[j];
            destroyed |= j.score(chips, mult, self, game, false);
        }
        match self.enhancements.card_type {
            CardType::Bonus => *chips += 30.0,
            CardType::Mult => *mult += 4.0,
            CardType::Glass => { *mult *= 2.0; if rng.random_range(0..4) == 0 { destroyed = true } },
            CardType::Lucky => { if rng.random_range(0..5) == 0 { *mult += 20.0 }; if rng.random_range(0..15) == 0 { game.game_data.money += 20 } },
            CardType::Stone => { *chips -= self.rank as f64; *chips += 50.0 },
            _ => {}
        }
        match self.enhancements.edition {
            CardEdition::Foil => *chips += 50.0,
            CardEdition::Holographic => *mult += 10.0,
            CardEdition::Polychrome => *mult *= 1.5,
            _ => {}
        }
        match self.enhancements.seal {
            CardSeal::Red => { destroyed |= self.retrigger(chips, mult, game);},
            _ => {}
        };
        game.deck.cards[i].count -= 1;
        if !destroyed {
            game.deck.push(*self, false);
        }
    }
    pub fn retrigger<T: DeckType>(&mut self, chips: &mut f64, mult: &mut f64, game: &mut Game<T>) -> bool {
        /*let mut chain = TriggerChain {
            child: None,
            source: RetriggerSource::RedSeal
        };*/
        //trigger_chain_add!(chain, RetriggerSource::RedSeal);
        let mut rng = rand::rng();
        let mut destroyed = false;
        for j in 0..game.jokers.len() {
            let j = game.jokers[j];
            destroyed |= j.score(chips, mult, self, game, true);
        }
        match self.enhancements.card_type {
            CardType::Bonus => *chips += 30.0,
            CardType::Mult => *mult += 4.0,
            CardType::Glass => { *mult *= 2.0 },
            CardType::Lucky => { if rng.random_range(0..5) == 0 { *mult += 20.0 }; if rng.random_range(0..15) == 0 { game.game_data.money += 20 } },
            CardType::Stone => { *chips -= self.rank as f64; *chips += 50.0 },
            _ => {}
        };
        match self.enhancements.edition {
            CardEdition::Foil => *chips += 50.0,
            CardEdition::Holographic => *mult += 10.0,
            CardEdition::Polychrome => *mult *= 1.5,
            _ => {}
        };
        match self.enhancements.seal {
            CardSeal::Gold => game.game_data.money += 3,
            _ => {}
        };
        destroyed
    }
}
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        ((self.suit == other.suit && self.rank == other.rank) && self.enhancements.card_type != CardType::Stone) &&
        self.enhancements == other.enhancements
    }
}
impl PartialEq<CardKey> for Card {
    fn eq(&self, other: &CardKey) -> bool {
        ((self.suit == other.suit && self.rank == other.rank) && self.enhancements.card_type != CardType::Stone) &&
        self.enhancements == other.enhancements
    }
}

#[derive(Hash, Eq, Clone, Copy)]
pub struct CardKey {
    pub rank: u8,
    pub enhancements: CardEnhancements,
    pub suit: CardSuit
}
impl PartialEq for CardKey {
    fn eq(&self, other: &Self) -> bool {
        ((self.suit == other.suit && self.rank == other.rank) && self.enhancements.card_type != CardType::Stone) &&
        self.enhancements == other.enhancements
    }
}
impl PartialEq<Card> for CardKey {
    fn eq(&self, other: &Card) -> bool {
        ((self.suit == other.suit && self.rank == other.rank) && self.enhancements.card_type != CardType::Stone) &&
        self.enhancements == other.enhancements
    }
}

impl From<Card> for CardKey {
    fn from(value: Card) -> Self {
        Self {
            rank: value.rank,
            enhancements: value.enhancements,
            suit: value.suit
        }
    }
}

#[derive(Serialize, Deserialize, Debug,  Default, Clone, Copy, IntoPrimitive, TryFromPrimitive, Hash, PartialEq, Eq)]
//this tells serde to turn the enum into a primitive u16 when serializing and deserialize it from a u16
#[serde(into = "u16", try_from = "u16")]
//repr stands for representation - the enum will be represented as a u16 in memory...
#[repr(u16)]
//...it can do this, because enum stands for enumerator
//you give it some possible values (which can have data attatched, but then you cant repr(int))
//and it enumerates them.
//its useful for grouping values together with names, like below
//in fact, i lied about Result - its not a struct, its an enum
//it looks something like this:
/*
pub enum Result<T, E: Error> {
    //(T) is some data attached to a value of Result::Ok
    //in this case, the value returned by a function when it succeed
    Ok(T),
    Err(E)
}
*/
//all those markers apply to this enum
pub enum CardSuit {
    #[default] Spades,
    Clubs,
    Hearts,
    Diamonds
}

#[derive(Serialize, Deserialize, Clone, Copy, Default, Debug, Hash, PartialEq, Eq)]
pub struct CardEnhancements {
    pub card_type: CardType,
    pub edition: CardEdition,
    pub seal: CardSeal,
    pub chips: usize
} /*impl Default for CardEnhancements {
    fn default() -> Self {
        Self { card_type: CardType::None, edition: CardEdition::None, seal: CardSeal::None, chips: 0}
    }
}*/

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive, Hash, PartialEq, Eq)]
#[serde(into = "u16", try_from = "u16")]
#[repr(u16)]
pub enum CardType {
    Lucky,
    Mult,
    Bonus,
    Gold,
    Steel,
    Glass,
    Stone,
    //another special marker, tells the derive above what the default enum is
    #[default] None
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive, Hash, PartialEq, Eq)]
#[serde(into = "u16", try_from = "u16")]
#[repr(u16)]
pub enum CardEdition {
    #[default] None = 0, //you can also give enums custom values
    Polychrome = 1,
    Holographic = 2,
    Foil = 3,
    Negative = 4,
}
#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive, Hash, PartialEq, Eq)]
#[serde(into = "u16", try_from = "u16")]
#[repr(u16)]
pub enum CardSeal {
    Red,
    Blue,
    Purple,
    Gold,
    #[default] None
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum HandType {
    #[default]
    HighCard,
    Pair,
    TwoPair,
    ThreeOak,
    Flush,
    Straight,
    FullHouse,
    FourOak,
    StraightFlush,
    FiveOak,
    FlushHouse,
    FlushFive
}
impl HandType {
    pub fn flush(self) -> Self {
        match self {
            Self::FiveOak => Self::FlushFive,
            Self::FlushFive => self,
            Self::FullHouse => Self::FlushHouse,
            Self::FlushHouse => self,
            Self::Straight => Self::StraightFlush,
            Self::StraightFlush => self,
            Self::FourOak => self,
            _ => Self::Flush
        }
    }
}