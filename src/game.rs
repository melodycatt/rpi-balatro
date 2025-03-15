use serde::{Deserialize, Serialize};
use serde_binary::{to_vec, from_vec, binary_stream::Endian};
use std::{collections::HashMap, env::{current_dir, current_exe}, fs, iter::Map};

use crate::{cards::Card, jokers::Joker};

#[derive(Serialize, Deserialize)]
struct Game<'a> {
    jokers: Vec<Joker<'a>>,
    cards: Vec<Card>,
    ante: usize,
    round: usize,
    money: usize,
}

static ANTES: HashMap<usize, f64> = HashMap::new();