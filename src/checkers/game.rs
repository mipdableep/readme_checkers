use color_eyre::{eyre::eyre, Result};
use derive_more::derive::{Deref, DerefMut};
use std::{fmt::Display, ops::Add};

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

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
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

impl Move {
    pub fn new(from: Location, to: Location) -> Self {
        Self { from, to }
    }
}

#[derive(Deref, DerefMut)]
pub struct Board([[Option<PieceType>; 8]; 8]);

impl Board {
    pub fn print_board(&self) {
        let mut ret = String::new();

        ret.push(' ');
        for c in 'A'..='H' {
            ret.push_str("   ");
            ret.push(c);
        }

        for (r_idx, row) in self.iter().enumerate() {
            ret.push_str(&format!("\n {}", r_idx + 1));
            for sq in row.iter() {
                ret.push_str(" [");
                match sq {
                    Some(PieceType::Black) => ret.push('b'),
                    Some(PieceType::BlackQueen) => ret.push('B'),
                    Some(PieceType::White) => ret.push('w'),
                    Some(PieceType::WhiteQueen) => ret.push('W'),
                    None => ret.push(' '),
                }
                ret.push(']')
            }
            ret.push_str(&format!(" {}", r_idx + 1));
        }

        ret.push('\n');
        ret.push(' ');
        for c in 'A'..='H' {
            ret.push_str("   ");
            ret.push(c);
        }

        println!("{ret}");
    }
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
        }

        board
    }
}

pub fn get_leagal_moves(board: &Board, loc: Location, last_move: Move) -> Vec<Location> {
    let Some(piece) = board
        .loc_index(&loc)
        .expect("trying to move piece at {loc:?}")
    else {
        panic!(
            "requested moves for empty piece - row {}, col {}",
            loc.row, loc.col
        )
    };

    match piece {
        PieceType::Black | PieceType::White => get_moves_soldier(board, loc, piece, last_move),
        PieceType::BlackQueen | PieceType::WhiteQueen => get_moves_queen(board, loc),
    }
}

fn get_moves_soldier(
    board: &Board,
    loc: Location,
    piece: PieceType,
    last_move: Move,
) -> Vec<Location> {
    let mut ret = vec![];
    if last_move.to == loc {
        ret.push(get_possible_move_soldier(board, loc, piece, (-1, -1)));
        ret.push(get_possible_move_soldier(board, loc, piece, (-1, 1)));
    }

    ret.push(get_possible_move_soldier(board, loc, piece, (1, 1)));
    ret.push(get_possible_move_soldier(board, loc, piece, (1, -1)));
    #[allow(unused_mut)]
    let mut ret = ret
        .iter()
        .filter_map(|loc_option| *loc_option)
        .collect::<Vec<Location>>();
    #[cfg(test)]
    ret.sort();

    ret
}

fn get_possible_move_soldier(
    board: &Board,
    loc: Location,
    moving_piece: PieceType,
    modif: (i8, i8),
) -> Option<Location> {
    let new_loc = loc + modif;
    let Ok(idx) = board.loc_index(&new_loc) else {
        return None;
    };
    let Some(piece) = idx else {
        return Some(new_loc);
    };
    if piece.get_color() == moving_piece.get_color() {
        None
    } else {
        match board.loc_index(&(new_loc + modif)) {
            Ok(None) => Some(new_loc + modif),
            _ => None,
        }
    }
}

fn get_moves_queen(board: &Board, loc: Location) -> Vec<Location> {
    todo!()
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
    #[test]
    fn basic_movment() {
        let loc_0 = Location::new(0, 0);
        let mut board = Board::new_game_board();
        assert_eq!(
            vec![Location::new(3, 1)],
            get_leagal_moves(&board, Location::new(2, 0), Move::new(loc_0, loc_0))
        );
        let mut v = vec![Location::new(3, 1), Location::new(3, 3)];
        v.sort();
        let moves = get_leagal_moves(&board, Location::new(2, 2), Move::new(loc_0, loc_0));
        assert_eq!(v, moves);
    }
}
