use crate::board::{Board, Move};
use crate::engine::GameEngineError::NoMove;
use crate::player::Player;
use rand::seq::SliceRandom;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum GameEngineError {
    LoadingError { message: String },
    NoMove { message: String },
}

impl Display for GameEngineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GameEngineError {}

pub trait GameEngine {
    fn load_board(board: Board) -> Result<Self, GameEngineError>
    where
        Self: Sized;

    fn next_move(&mut self, playing: Player) -> Result<Move, GameEngineError>;
}

pub struct AlmostRandomEngine {
    board: Board,
}

impl GameEngine for AlmostRandomEngine {
    fn load_board(board: Board) -> Result<Self, GameEngineError> {
        Ok(AlmostRandomEngine { board })
    }

    fn next_move(&mut self, playing: Player) -> Result<Move, GameEngineError> {
        let empty_positions = self.board.empty_positions();

        let maybe_move = if self.board.number_of_moves() == 0 {
            let center_position = match self.board.size {
                3 => (1, 1),
                5 => (3, 3),
                7 => (4, 4),
                _ => panic!("Impossible.")
            };

            Some(Move { player: playing, position: center_position })
        } else {
            empty_positions.choose(&mut rand::thread_rng()).map(|&position| Move { player: playing, position })
        };

        if let Some(new_move) = maybe_move {
            Ok(Move {
                player: new_move.player,
                position: new_move.position,
            })
        } else {
            Err(NoMove {
                message: "No valid moves available".to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::Player::*;
    use crate::size::Size;

    #[test]
    fn test_load_board() {
        let mut board = Board::new(Size::from_usize(3));
        board.add_move(X, (0, 0)).expect("Failed to add move");
        board.add_move(O, (1, 1)).expect("Failed to add move");

        let mut engine = AlmostRandomEngine::load_board(board).expect("Failed to load board");
        let next_move = engine.next_move(X);

        assert_eq!(next_move.unwrap().player, X)
    }
}
