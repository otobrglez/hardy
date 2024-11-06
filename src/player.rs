use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Player {
    X,
    O,
}
use Player::*;

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            X => write!(f, "X"),
            O => write!(f, "O"),
        }
    }
}

impl FromStr for Player {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(X),
            "O" => Ok(O),
            _ => Err("Sorry. Can't deserialize.".to_string()),
        }
    }
}
