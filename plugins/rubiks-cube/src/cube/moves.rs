use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Move {
    R,
    RPrime,
    R2,
    L,
    LPrime,
    L2,
    U,
    UPrime,
    U2,
    D,
    DPrime,
    D2,
    F,
    FPrime,
    F2,
    B,
    BPrime,
    B2,
}

impl Move {
    pub fn from_notation(s: &str) -> Option<Move> {
        match s.trim() {
            "R" => Some(Move::R),
            "R'" => Some(Move::RPrime),
            "R2" => Some(Move::R2),
            "L" => Some(Move::L),
            "L'" => Some(Move::LPrime),
            "L2" => Some(Move::L2),
            "U" => Some(Move::U),
            "U'" => Some(Move::UPrime),
            "U2" => Some(Move::U2),
            "D" => Some(Move::D),
            "D'" => Some(Move::DPrime),
            "D2" => Some(Move::D2),
            "F" => Some(Move::F),
            "F'" => Some(Move::FPrime),
            "F2" => Some(Move::F2),
            "B" => Some(Move::B),
            "B'" => Some(Move::BPrime),
            "B2" => Some(Move::B2),
            _ => None,
        }
    }
}

pub fn parse_scramble(notation: &str) -> Vec<Move> {
    notation
        .split_whitespace()
        .filter_map(Move::from_notation)
        .collect()
}
