use serde::{Deserialize, Serialize};
use serde_binary::{to_vec, from_vec, binary_stream::Endian};
use std::{collections::HashMap, env::{current_dir, current_exe}, fs, iter::Map};
use crate::{cards::Card, jokers::Joker};
mod blinds;

#[derive(Serialize, Deserialize)]
struct Game<'a> {
    jokers: Vec<Joker<'a>>,
    cards: Vec<Card>,
    ante: usize,
    round: usize,
    money: usize,
}