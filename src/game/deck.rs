use std::{any::Any, marker::PhantomData};

use blind_macros::{derive_trait_getters, generate_trait_getters};
use const_default::ConstDefault;
use num_enum::TryFromPrimitive;
use rand::random_range;
use serde::{Deserialize, Serialize};
use crate::{cards::{Card, CardSuit}, vouchers::{VoucherLevel, Vouchers}};

use super::RoundData;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Deck<T: DeckType + Any> {
    cards: Vec<Card>,
    deck_type_phantom: PhantomData<T>
}

impl<T: DeckType + Any> Deck<T> {
    pub fn initialise_cards(&mut self) { T::initialise_cards(&mut self.cards) }
    pub fn score(&self, chips: f64, mult: f64) -> (f64, f64) { T::score(chips, mult) }
    pub fn interest(&self, round_data: RoundData, multiplier: usize, max: usize) -> usize { T::interest(round_data, multiplier, max) }

    pub fn new() -> Deck<T> {
        Deck::<T> {
            cards: Vec::new(),
            deck_type_phantom: PhantomData
        }
    }
}

//#[generate_trait_getters(config, DeckConfig)]
pub trait DeckType {
    const CONFIG: &'static DeckConfig;

    fn initialise_cards(cards: &mut Vec<Card>) {
        for i in 0..4 {
            for j in 1..=14 {
                cards.push(Card::new_default(CardSuit::try_from_primitive(i).expect("card suit in default fucked up"), j));
            }
        }
    }

    fn score(chips: f64, mult: f64) -> (f64, f64) {
        (chips, mult)
    }
    fn interest(round_data: RoundData, multiplier: usize, max: usize) -> usize {
        (num::Integer::div_floor(&round_data.money, &5).min(max)) * multiplier
    }
}

pub struct RedDeck;
impl DeckType for RedDeck {
    const CONFIG: &'static DeckConfig = &DeckConfig {
        hands: 0,
        discards: 1,
        money: 0,
        hand_size: 0,
        joker_slots: 0,
        consumable_slots: 0,
        vouchers: Vouchers::DEFAULT,
        consumables: Vec::new(),
    };
}
pub struct BlueDeck;
impl DeckType for BlueDeck {
    const CONFIG: &'static DeckConfig = &DeckConfig {
        hands: 1,
        discards: 0,
        money: 0,
        hand_size: 0,
        joker_slots: 0,
        consumable_slots: 0,
        vouchers: Vouchers::DEFAULT,
        consumables: Vec::new(),
    };
}
pub struct YellowDeck;
impl DeckType for YellowDeck {
    const CONFIG: &'static DeckConfig = &DeckConfig {
        hands: 0,
        discards: 0,
        money: 10,
        hand_size: 0,
        joker_slots: 0,
        consumable_slots: 0,
        vouchers: Vouchers::DEFAULT,
        consumables: Vec::new(),
    };
}
pub struct GreenDeck;
impl DeckType for GreenDeck {
    const CONFIG: &'static DeckConfig = &DeckConfig {
        hands: 0,
        discards: 0,
        money: 0,
        hand_size: 0,
        joker_slots: 0,
        consumable_slots: 0,
        vouchers: Vouchers::DEFAULT,
        consumables: Vec::new(),
    };

    fn interest(round_data: RoundData, _multiplier: usize, _max: usize) -> usize {
        2 * round_data.hands + round_data.discards
    }
}
pub struct BlackDeck;
impl DeckType for BlackDeck {
    const CONFIG: &'static DeckConfig = &DeckConfig {
        hands: -1,
        discards: 0,
        money: 0,
        hand_size: 0,
        joker_slots: 1,
        consumable_slots: 0,
        vouchers: Vouchers::DEFAULT,
        consumables: Vec::new(),
    };
}
pub struct MagicDeck;
impl DeckType for MagicDeck {
    const CONFIG: &'static DeckConfig = &DeckConfig {
        hands: 0,
        discards: 0,
        money: 0,
        hand_size: 0,
        joker_slots: 0,
        consumable_slots: 0,
        vouchers: Vouchers::DEFAULT.with_crystal_ball(VoucherLevel::Base),
        consumables: Vec::new(), //TODO: add 2 fools here
    };
}
pub struct NebulaDeck;
impl DeckType for NebulaDeck {
    const CONFIG: &'static DeckConfig = &DeckConfig {
        hands: 0,
        discards: 0,
        money: 0,
        hand_size: 0,
        joker_slots: 0,
        consumable_slots: -1,
        vouchers: Vouchers::DEFAULT.with_telescope(VoucherLevel::Base),
        consumables: Vec::new(),
    };
}
pub struct AbandondedDeck;
impl DeckType for AbandondedDeck {
    const CONFIG: &'static DeckConfig = &DeckConfig {
        hands: 0,
        discards: 0,
        money: 0,
        hand_size: 0,
        joker_slots: 0,
        consumable_slots: 0,
        vouchers: Vouchers::DEFAULT,
        consumables: Vec::new(),
    };

    fn initialise_cards(cards: &mut Vec<Card>) {
        for i in 0..4 {
            for j in 1..=10 {
                cards.push(Card::new_default(CardSuit::try_from_primitive(i).expect("card suit in default fucked up"), j));
            }
        }
        for i in 0..4 {
            cards.push(Card::new_default(CardSuit::try_from_primitive(i).expect("card suit in default fucked up"), 14));
        }
    }
}
pub struct CheckeredDeck;
impl DeckType for CheckeredDeck {
    const CONFIG: &'static DeckConfig = &DeckConfig {
        hands: 0,
        discards: 0,
        money: 0,
        hand_size: 0,
        joker_slots: 0,
        consumable_slots: 0,
        vouchers: Vouchers::DEFAULT,
        consumables: Vec::new(),
    };
    fn initialise_cards(cards: &mut Vec<Card>) {
        for i in [0,0,2,2] {
            for j in 1..=14 {
                cards.push(Card::new_default(CardSuit::try_from_primitive(i).expect("card suit in default fucked up"), j));
            }
        }
    }
}
pub struct ZodiacDeck;
impl DeckType for ZodiacDeck {
    const CONFIG: &'static DeckConfig = &DeckConfig {
        hands: 0,
        discards: 0,
        money: 0,
        hand_size: 0,
        joker_slots: 0,
        consumable_slots: 0,
        vouchers: Vouchers::DEFAULT.with_tarot_merch(VoucherLevel::Base).with_planet_merch(VoucherLevel::Base).with_overstock(VoucherLevel::Base),
        consumables: Vec::new(),
    };
}
pub struct PaintedDeck;
impl DeckType for PaintedDeck {
    const CONFIG: &'static DeckConfig = &DeckConfig {
        hands: 0,
        discards: 0,
        money: 0,
        hand_size: 2,
        joker_slots: -1,
        consumable_slots: 0,
        vouchers: Vouchers::DEFAULT,
        consumables: Vec::new(),
    };
}
pub struct AnaglyphDeck;
impl DeckType for AnaglyphDeck {
    const CONFIG: &'static DeckConfig = &DeckConfig {
        hands: 0,
        discards: 1,
        money: 0,
        hand_size: 0,
        joker_slots: 0,
        consumable_slots: 0,
        vouchers: Vouchers::DEFAULT,
        consumables: Vec::new(),
    };
}
pub struct PlasmaDeck;
impl DeckType for PlasmaDeck {
    const CONFIG: &'static DeckConfig = &DeckConfig {
        hands: 0,
        discards: 0,
        money: 0,
        hand_size: 0,
        joker_slots: 0,
        consumable_slots: 0,
        vouchers: Vouchers::DEFAULT,
        consumables: Vec::new(),
    };
    fn score(chips: f64, mult: f64) -> (f64, f64) {
        ((chips * mult / 2.0), (chips * mult / 2.0))
    }
}
pub struct ErraticDeck;
impl DeckType for ErraticDeck {
    const CONFIG: &'static DeckConfig = &DeckConfig {
        hands: 0,
        discards: 0,
        money: 0,
        hand_size: 0,
        joker_slots: 0,
        consumable_slots: 0,
        vouchers: Vouchers::DEFAULT,
        consumables: Vec::new(),
    };
    fn initialise_cards(cards: &mut Vec<Card>) {
        for _ in 0..52 {
            cards.push(Card::new_default(CardSuit::try_from_primitive(random_range(0..4u16)).expect("plasma init_cards try_into failed"), random_range(1..=14)));
        }
    }
}

pub struct DeckConfig {
    pub hands: isize,
    pub discards: isize,
    pub money: isize,
    pub hand_size: isize,
    pub joker_slots: isize,
    pub consumable_slots: isize,
    pub vouchers: Vouchers,
    pub consumables: Vec<()>,
}