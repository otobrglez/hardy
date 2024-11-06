use crate::board::{Board, Move, Position};
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

pub struct RandomEngine {
    board: Board,
}

impl GameEngine for RandomEngine {
    fn load_board(board: Board) -> Result<Self, GameEngineError> {
        Ok(RandomEngine { board })
    }

    fn next_move(&mut self, playing: Player) -> Result<Move, GameEngineError> {
        let empty_positions = self.board.empty_positions();
        let random_position = empty_positions.choose(&mut rand::thread_rng());

        if let Some(position) = random_position {
            Ok(Move {
                player: playing,
                position: position.clone(),
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
    fn test_load_board() -> Result<(), Box<dyn Error>> {
        let mut board = Board::new(Size::from_usize(3));
        board.add_move(X, (0, 0))?;
        board.add_move(O, (1, 1))?;

        let mut engine = RandomEngine::load_board(board)?;
        let next_move = engine.next_move(X)?;

        println!("{:?}", next_move);
        Ok(())
    }
}
