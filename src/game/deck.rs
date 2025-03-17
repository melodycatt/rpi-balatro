use serde::{Deserialize, Serialize};

use crate::cards::Card;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Deck<T: DeckType> {
    deck_type: T,
    cards: Vec<Card>
}

pub trait DeckType {

}