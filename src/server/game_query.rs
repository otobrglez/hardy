use crate::board::Move;
use crate::player::Player;
use serde::de::Error;
use serde::{Deserialize as SerdeDeserialize, Deserializer};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

pub type GID = Uuid;

#[derive(Serialize, Debug, Clone)]
pub struct GameQuery {
    pub gid: GID,
    pub size: usize,
    pub playing: Player,
    #[serde(deserialize_with = "deserialize_moves")]
    pub moves: Vec<Move>,
}

impl<'de> SerdeDeserialize<'de> for GameQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct GameQueryInternal {
            gid: Uuid,
            size: Option<usize>,
            playing: Player,
            moves: String,
        }

        let GameQueryInternal {
            gid,
            size,
            playing,
            moves,
        } = GameQueryInternal::deserialize(deserializer)?;

        // If size is something crazy - convert int to default. Default is 3
        let size = size.filter(|&s| s == 3 || s == 5 || s == 7).unwrap_or(3);

        // Loop over the tokens and convert to valid objects and structs
        let partially_parsed_moves: Vec<Option<Move>> = moves
            .splitn(size * size, "_")
            .into_iter()
            .map(|s| {
                let tpl: Vec<String> = s
                    .to_string()
                    .splitn(3, "-")
                    .map(|p| p.to_string())
                    .into_iter()
                    .collect();

                let maybe_player: Option<Player> = tpl
                    .get(0)
                    .map(|t| {
                        if t == "X" {
                            Ok(Player::X)
                        } else if t == "O" {
                            Ok(Player::O)
                        } else {
                            Err("Unknown symbol.".to_string())
                        }
                    })
                    .unwrap()
                    .ok();

                let maybe_row: Option<usize> = tpl.get(1).and_then(|t| t.parse().ok());
                let maybe_col: Option<usize> = tpl.get(2).and_then(|t| t.parse().ok());

                if let (Some(playing), Some(row), Some(col)) = (maybe_player, maybe_row, maybe_col)
                {
                    Some(Move {
                        player: playing,
                        position: (row, col),
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // We need to handle the case when moves are empty.
        if partially_parsed_moves.get(0) == Some(&None) && partially_parsed_moves.len() == 1 {
            return Ok(GameQuery {
                gid,
                size,
                playing,
                moves: vec![],
            });
        }

        // If some of the moves are invalid; raise error.
        if let Some(moves) = partially_parsed_moves
            .into_iter()
            .collect::<Option<Vec<_>>>()
        {
            Ok(GameQuery {
                gid,
                size,
                playing,
                moves,
            })
        } else {
            Err(Error::custom("Invalid move detected."))
        }
    }
}
