use color_eyre::{eyre::eyre, Result};
use derive_more::derive::{Deref, DerefMut};
use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceType {
    Black,
    White,
    BlackQueen,
    WhiteQueen,
}

impl PieceType {
    pub fn get_color(&self) -> Color {
        match self {
            Self::Black | Self::BlackQueen => Color::Black,
            Self::White | Self::WhiteQueen => Color::White,
        }
    }
}

#[derive(PartialEq, Eq)]
enum Color {
    Black,
    White,
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub struct Location {
    row: i8,
    col: i8,
}

impl Location {
    pub fn new(row: i8, col: i8) -> Self {
        Self { row, col }
    }
}

impl Add<(i8, i8)> for Location {
    type Output = Location;

    fn add(self, rhs: (i8, i8)) -> Self::Output {
        Self::new(self.row + rhs.0, self.col + rhs.1)
    }
}

pub struct Move {
    from: Location,
    to: Location,
}

#[derive(Deref, DerefMut)]
pub struct Board([[Option<PieceType>; 8]; 8]);

impl Board {
    pub fn loc_index(&self, loc: &Location) -> Result<Option<PieceType>> {
        match (loc.row, loc.col) {
            (0..8, 0..8) => Ok(self[loc.row as usize][loc.col as usize]),
            _ => Err(eyre!("idexing error [{}][{}]", loc.row, loc.col)),
        }
    }
    pub fn new_game_board() -> Board {
        let mut board: Board = Board([[None; 8]; 8]);
        for (row_num, row) in board.iter_mut().enumerate() {
            for (col_num, square) in row.iter_mut().enumerate() {
                // construct board from A1, white first
                if (row_num + col_num) % 2 == 0 {
                    match row_num {
                        0..=2 => *square = Some(PieceType::White),
                        5..=7 => *square = Some(PieceType::Black),
                        _ => (),
                    };
                };
            }
            println!("{row:?}");
        }

        board
    }
}
#[cfg(test)]
mod game_test {
    use super::*;
    #[test]
    fn loc_add_tuple() {
        assert_eq!(Location::new(2, 2), Location::new(1, 1) + (1, 1));
    }

    #[test]
    fn starting_board() {
        let board = Board::new_game_board();
        for (row_n, row) in board.iter().enumerate() {
            for (col_n, sq) in row.iter().enumerate() {
                if (row_n + col_n) % 2 == 0 {
                    match row_n {
                        0..=2 => assert_eq!(&Some(PieceType::White), sq),
                        5..=7 => assert_eq!(&Some(PieceType::Black), sq),
                        _ => {
                            if sq.is_some() {
                                panic!("should be none - row {row_n} col {col_n}")
                            }
                        }
                    }
                } else if sq.is_some() {
                    panic!("should be none - row {row_n} col {col_n}")
                }
            }
        }
    }
}
