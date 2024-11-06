#![allow(dead_code)]
use crate::player::Player;
use crate::server::game_query::GameQuery;
use crate::size::Size;
use serde_derive::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug)]
pub struct Board {
    cells: Vec<Option<Player>>,
    size: usize,
}

pub type Position = (usize, usize);

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct Move {
    pub player: Player,
    pub position: Position,
}

impl Board {
    pub(crate) fn new(size: Size) -> Board {
        Board {
            cells: vec![None; size.as_usize() * size.as_usize()],
            size: size.as_usize(),
        }
    }

    fn is_empty(&self) -> bool {
        self.cells.iter().all(|&cell| cell.is_none())
    }

    pub fn empty_positions(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        for row in 0..self.size {
            for col in 0..self.size {
                let idx = row * self.size + col;
                if self.cells[idx].is_none() {
                    positions.push((row, col));
                }
            }
        }
        positions
    }

    pub fn add_move(&mut self, player: Player, position: Position) -> Result<(), &'static str> {
        let (row, col) = position;
        if row >= self.size || col >= self.size {
            return Err("Position out of bounds");
        }
        let idx = row * self.size + col;
        if self.cells[idx].is_some() {
            return Err("Position already occupied");
        }
        self.cells[idx] = Some(player);
        Ok(())
    }

    pub fn add_a_move(&mut self, a_move: Move) -> Result<(), &'static str> {
        self.add_move(a_move.player, a_move.position)
    }

    const EMPTY_SYMBOL: &'static str = ".";

    fn display(&self) -> () {
        for row in 0..self.size {
            for col in 0..self.size {
                let symbol = match self.cells[row * self.size + col] {
                    Some(x) => x.to_string(),
                    None => Self::EMPTY_SYMBOL.to_string(),
                };
                print!("{}", symbol);
            }
            println!();
        }
        println!();
    }
}

impl TryFrom<GameQuery> for Board {
    type Error = ();

    fn try_from(game_query: GameQuery) -> Result<Self, Self::Error> {
        let mut board: Board = Board::new(Size::from_usize(3));
        game_query.moves.iter().for_each(|&m| {
            board.add_a_move(m).expect("Failed adding a move.");
        });

        Ok(board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::game_query::GameQuery;
    use crate::size::Size;
    use uuid::Uuid;
    use Player::{O, X};

    #[test]
    fn test_boards_creation() {
        let board1 = Board::new(Size::from_usize(3));
        let board2 = Board::new(Size::from_usize(5));
        let board3 = Board::new(Size::from_usize(7));

        assert_eq!(board1.size, 3usize);
        assert_eq!(board2.size, 5usize);
        assert_eq!(board3.size, 7usize);
    }

    #[test]
    fn test_if_empty() {
        let mut board1 = Board::new(Size::from_usize(3));
        assert_eq!(board1.is_empty(), true);

        board1.add_move(X, (1, 1)).unwrap();
        assert_eq!(board1.is_empty(), false);
    }

    #[test]
    fn test_display() {
        let mut board1 = Board::new(Size::from_usize(3));
        board1.add_move(X, (0, 0)).unwrap();
        board1.add_move(O, (1, 1)).unwrap();
        board1.add_move(X, (0, 1)).unwrap();
        board1.add_move(O, (0, 2)).unwrap();
        board1
            .add_a_move(Move {
                player: X,
                position: (2, 0),
            })
            .unwrap();
        board1.display();
    }

    #[test]
    fn test_from_game_query() {
        let query_1 = GameQuery {
            gid: Uuid::new_v4(),
            size: 3,
            playing: O,
            moves: vec![
                Move {
                    player: X,
                    position: (1, 1),
                },
                Move {
                    player: O,
                    position: (0, 1),
                },
            ],
        };

        let board = Board::try_from(query_1).unwrap();
        board.display();
    }
}
