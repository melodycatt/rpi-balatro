//serde again, but also num_enum
//this is to avoid problems with serializing a special type called an enum
//it allows you to convert enums into primitive number types
//which is quick and easy to serialize
//whereas (for whatever reason) serde_binary serializes them as strings
//which (for whatever reason) caused deserialization panics
use serde::{Deserialize, Serialize};
use num_enum::{Default, IntoPrimitive, TryFromPrimitive};

#[derive(Serialize, Deserialize, Clone, Copy, Default)]
pub struct Card {
    pub enhancements: CardEnhancements,
    pub rank: u8,
    pub suit: Suit,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
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
pub enum Suit {
    Spades,
    Clubs,
    Diamonds,
    Hearts
}

#[derive(Serialize, Deserialize, Default, Clone, Copy, Default)]
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
    //another special marker, tells the derive above what the default enum is
    #[default] None
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[serde(into = "u16", try_from = "u16")]
#[repr(u16)]
pub enum CardEdition {
    #[default] None = 0, //you can also give enums custom values
    Polychrome = 1,
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
