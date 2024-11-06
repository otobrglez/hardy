#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Player {
    X,
    O,
}

use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

impl FromStr for Player {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "X" {
            Ok(Player::X)
        } else if s == "O" {
            Ok(Player::O)
        } else {
            Err("Sorry. Can't deserialize.".to_string())
        }
    }
}
