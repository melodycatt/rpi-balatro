use serde::{Deserialize, Serialize};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub enhancements: CardEnhancements,
    pub rank: u8,
    pub suit: Suit,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[serde(into = "u16", try_from = "u16")]
#[repr(u16)]
pub enum Suit {
    Spades,
    Clubs,
    Diamonds,
    Hearts
}

#[derive(Serialize, Deserialize, Default)]
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

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
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
    #[default] None
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[serde(into = "u16", try_from = "u16")]
#[repr(u16)]
pub enum CardEdition {
    #[default] None = 0,
    Polychrome =1,
    Holographic = 2,
    Foil = 3,
    Negative = 4,
}
#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[serde(into = "u16", try_from = "u16")]
#[repr(u16)]
pub enum CardSeal {
    Red,
    Blue,
    Purple,
    #[default] None
}
