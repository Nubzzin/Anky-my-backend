use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum CardState {
    NEW,
    REVIEW,
    DUE,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum StruggleLevel {
    AGAIN,
    HARD,
    GOOD,
    EASY,
}
