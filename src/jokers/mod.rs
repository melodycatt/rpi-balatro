mod load;
pub use load::{JokerLoader, save};

use load::JokerApplyFunction;
use libloading::Symbol;
use serde::{Serialize, Deserialize};
use crate::cards::CardEdition;

#[derive(Debug)]
pub struct Joker<'a> {
    pub data: JokerData,
    apply: Symbol<'a, JokerApplyFunction>
}
impl<'a> Joker<'a> {
    pub fn apply(&self, x: f64, y: f64) -> f64 {
        (self.apply)(x, y)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JokerData {
    pub name: String,
    pub id: String,
    pub enhancements: JokerEnhancements,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct JokerEnhancements {
    modifiers: JokerModifiers,
    edition: CardEdition,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct JokerModifiers {
    pub eternal: bool,
    pub perishable: bool,
    pub rental: bool,
}