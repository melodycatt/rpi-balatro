pub const BASE_SCORES: [f64; 9] = [
    0.0,
    300.0,
    800.0,
    2000.0,
    5000.0,
    11000.0,
    20000.0,
    35000.0,
    50000.0
];
pub const GREEN_SCORES: [f64; 9] = [
    0.0,
    300.0,
    900.0,
    2600.0,
    8000.0,
    20000.0,
    36000.0,
    60000.0,
    100000.0
];
pub const PURPLE_SCORES: [f64; 9] = [
    0.0,
    100.0,
    1000.0,
    3200.0,
    9000.0,
    25000.0,
    60000.0,
    110000.0,
    200000.0
];

use serde::{Serialize, Deserialize};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[serde(into = "u16", try_from = "u16")]
#[repr(u16)]
pub enum Scaling {
    #[default] Base,
    Green,
    Purple,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[serde(into = "u16", try_from = "u16")]
#[repr(u16)]
pub enum BlindType {
    Small,
    Big,
    Wall,
}

impl BlindType {
    pub fn score_requirement(&self, ante_score: f64) -> f64 {
        return match self {
            BlindType::Small => 1.0,
            BlindType::Big => 1.5,
            BlindType::Wall => 4.0
        } * ante_score
    }
}