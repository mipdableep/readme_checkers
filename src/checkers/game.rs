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

impl From<(i8, i8)> for Location {
    fn from(value: (i8, i8)) -> Self {
        Self::new(value.0, value.1)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Move {
    pub from: Location,
    pub to: Location,
    pub eat: Option<Location>,
}

impl Move {
    pub fn new(from: Location, to: Location, eat: Option<Location>) -> Self {
        Self { from, to, eat }
    }
}

#[derive(Deref, DerefMut)]
pub struct Board([[Option<PieceType>; 8]; 8]);

impl Board {
    pub fn print_board_indedex_with_loc_vec(&self, locs: &[Location]) {
        assert!(locs.is_sorted());
        let mut ret = String::new();

        ret.push(' ');
        for c in '0'..='7' {
            ret.push_str("   ");
            ret.push(c);
        }

        for (r_idx, row) in self.iter().enumerate() {
            ret.push_str(&format!("\n {}", r_idx));
            for (c_idx, sq) in row.iter().enumerate() {
                ret.push_str(" [");
                match sq {
                    Some(PieceType::Black) => ret.push('b'),
                    Some(PieceType::BlackQueen) => ret.push('B'),
                    Some(PieceType::White) => ret.push('w'),
                    Some(PieceType::WhiteQueen) => ret.push('W'),
                    None => {
                        if locs
                            .binary_search(&((r_idx as i8, c_idx as i8).into()))
                            .is_ok()
                        {
                            ret.push('X');
                        } else {
                            ret.push(' ');
                        }
                    }
                }
                ret.push(']')
            }
            ret.push_str(&format!(" {}", r_idx));
        }

        ret.push('\n');
        ret.push(' ');
        for c in '0'..='7' {
            ret.push_str("   ");
            ret.push(c);
        }

        println!("{ret}");
    }
    pub fn print_board_index(&self) {
        let mut ret = String::new();

        ret.push(' ');
        for c in '0'..='7' {
            ret.push_str("   ");
            ret.push(c);
        }

        for (r_idx, row) in self.iter().enumerate() {
            ret.push_str(&format!("\n {}", r_idx));
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
            ret.push_str(&format!(" {}", r_idx));
        }

        ret.push('\n');
        ret.push(' ');
        for c in '0'..='7' {
            ret.push_str("   ");
            ret.push(c);
        }

        println!("{ret}");
    }

    pub fn print_board_squares(&self) {
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
    pub fn new() -> Self {
        Board([[None; 8]; 8])
    }
}

pub fn get_leagal_moves(board: &Board, loc: Location, last_move: Move) -> Vec<Move> {
    let Some(piece) = board
        .loc_index(&loc)
        .expect("trying to move piece at {loc:?}")
    else {
        panic!(
            "requested moves for empty piece - row {}, col {}",
            loc.row, loc.col
        )
    };

    #[allow(unused_mut)]
    let mut v = match piece {
        PieceType::Black | PieceType::White => get_moves_soldier(board, loc, piece, last_move),
        PieceType::BlackQueen | PieceType::WhiteQueen => get_moves_queen(board, piece, loc),
    };

    #[cfg(test)]
    v.sort();

    v
}

fn get_moves_soldier(board: &Board, loc: Location, piece: PieceType, last_move: Move) -> Vec<Move> {
    match (last_move.to == loc, piece.get_color()) {
        (true, Color::Black) => get_moves_soldier_from_modif_vec(
            board,
            loc,
            piece,
            vec![
                ((-1, -1), true),
                ((-1, 1), true),
                ((1, 1), true),
                ((1, -1), true),
            ],
        ),
        (true, Color::White) => get_moves_soldier_from_modif_vec(
            board,
            loc,
            piece,
            vec![
                ((1, 1), true),
                ((1, -1), true),
                ((-1, -1), true),
                ((-1, 1), true),
            ],
        ),
        (false, Color::Black) => get_moves_soldier_from_modif_vec(
            board,
            loc,
            piece,
            vec![((-1, -1), false), ((-1, 1), false)],
        ),
        (false, Color::White) => get_moves_soldier_from_modif_vec(
            board,
            loc,
            piece,
            vec![((1, 1), false), ((1, -1), false)],
        ),
    }
}

fn get_moves_soldier_from_modif_vec(
    board: &Board,
    loc: Location,
    moving_piece: PieceType,
    // modif + eat
    modifs: Vec<((i8, i8), bool)>,
) -> Vec<Move> {
    let mut ret = Vec::with_capacity(modifs.len());
    for (m, eat) in modifs {
        if eat {
            ret.push(get_possible_eat_soldier(board, loc, moving_piece, m));
        } else {
            ret.push(get_possible_move_or_eat_soldier(
                board,
                loc,
                moving_piece,
                m,
            ));
        }
    }

    ret.iter()
        .filter_map(|loc_option| *loc_option)
        .collect::<Vec<Move>>()
}

fn get_possible_move_or_eat_soldier(
    board: &Board,
    loc: Location,
    moving_piece: PieceType,
    modif: (i8, i8),
) -> Option<Move> {
    let new_loc = loc + modif;
    let Ok(idx) = board.loc_index(&new_loc) else {
        return None;
    };
    let Some(piece) = idx else {
        return Some(Move::new(loc, new_loc, None));
    };
    if piece.get_color() == moving_piece.get_color() {
        None
    } else {
        match board.loc_index(&(new_loc + modif)) {
            Ok(None) => Some(Move::new(loc, new_loc + modif, Some(new_loc))),
            _ => None,
        }
    }
}

fn get_possible_eat_soldier(
    board: &Board,
    loc: Location,
    moving_piece: PieceType,
    modif: (i8, i8),
) -> Option<Move> {
    let new_loc = loc + modif;
    match board.loc_index(&new_loc) {
        Ok(Some(p)) if p.get_color() != moving_piece.get_color() => {
            match board.loc_index(&(new_loc + modif)) {
                Ok(None) => Some(Move::new(loc, new_loc + modif, Some(new_loc))),
                _ => None,
            }
        }
        _ => None,
    }
}

static DIRECTIONS: [(i8, i8); 4] = [(-1, -1), (-1, 1), (1, 1), (1, -1)];

fn get_moves_queen(board: &Board, piece: PieceType, loc: Location) -> Vec<Move> {
    let mut ret = vec![];
    'dir_loop: for modif in DIRECTIONS {
        let mut new_loc = loc;
        loop {
            new_loc = new_loc + modif;
            match board.loc_index(&new_loc) {
                Ok(Some(p)) if p.get_color() != piece.get_color() => {
                    let Ok(None) = board.loc_index(&(new_loc + modif)) else {
                        continue 'dir_loop;
                    };
                    ret.push(Move::new(loc, new_loc + modif, Some(new_loc)));
                    continue 'dir_loop;
                }
                Ok(Some(_)) => continue 'dir_loop,
                Ok(None) => ret.push(Move::new(loc, new_loc, None)),
                Err(_) => continue 'dir_loop,
            }
        }
    }

    ret
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
    fn basic_movment_1() {
        let loc_0 = Location::new(0, 0);
        let loc_22 = Location::new(2, 2);
        let board = Board::new_game_board();
        let mut v = vec![Move::new(Location::new(2, 0), Location::new(3, 1), None)];
        v.sort();
        assert_eq!(
            v,
            get_leagal_moves(&board, Location::new(2, 0), Move::new(loc_0, loc_0, None))
        );
    }

    #[test]
    fn basic_movment_2() {
        let loc_0 = Location::new(0, 0);
        let loc_22 = Location::new(2, 2);
        let board = Board::new_game_board();
        let mut v = vec![
            Move::new(loc_22, loc_22 + (1, -1), None),
            Move::new(loc_22, loc_22 + (1, 1), None),
        ];
        v.sort();
        let moves = get_leagal_moves(&board, loc_22, Move::new(loc_0, loc_0, None));
        assert_eq!(v, moves);
    }

    #[test]
    fn simpble_and_complex_movment_white() {
        let loc_0 = Location::new(0, 0);
        let loc_33 = Location::new(3, 3);
        let move_0 = Move::new(loc_0, loc_0, None);
        let move_to_33 = Move::new(loc_0, loc_33, None);
        let mut board = Board::new();
        board[3][3] = Some(PieceType::White);
        board[4][2] = Some(PieceType::White);
        board[4][4] = Some(PieceType::Black);
        board[2][4] = Some(PieceType::Black);
        let mut v = vec![Move::new(loc_33, loc_33 + (2, 2), Some(loc_33 + (1, 1)))];
        v.sort();
        assert_eq!(v, get_leagal_moves(&board, loc_33, move_0));
        let mut v = vec![
            Move::new(loc_33, loc_33 + (2, 2), Some(loc_33 + (1, 1))),
            Move::new(loc_33, loc_33 + (-2, 2), Some(loc_33 + (-1, 1))),
        ];
        v.sort();
        assert_eq!(v, get_leagal_moves(&board, loc_33, move_to_33));
    }
    #[test]
    fn simpble_and_complex_movment_black() {
        let loc_0 = Location::new(0, 0);
        let loc_33 = Location::new(3, 3);
        let move_0 = Move::new(loc_0, loc_0, None);
        let move_to_33 = Move::new(loc_0, loc_33, None);
        let mut board = Board::new();
        board[3][3] = Some(PieceType::Black);
        board[2][2] = Some(PieceType::Black);
        board[4][4] = Some(PieceType::White);
        board[2][4] = Some(PieceType::White);
        let mut v = vec![Move::new(
            loc_33,
            Location::new(1, 5),
            Some(Location::new(2, 4)),
        )];
        v.sort();
        assert_eq!(v, get_leagal_moves(&board, loc_33, move_0));
        let mut v = vec![
            Move::new(loc_33, loc_33 + (2, 2), Some(loc_33 + (1, 1))),
            Move::new(loc_33, loc_33 + (-2, 2), Some(loc_33 + (-1, 1))),
        ];
        v.sort();
        assert_eq!(v, get_leagal_moves(&board, loc_33, move_to_33));
    }
    #[test]
    fn move_after_eat() {
        let loc_0 = Location::new(0, 0);
        let loc_33 = Location::new(3, 3);
        let move_to_33 = Move::new(loc_0, loc_33, None);
        let mut board = Board::new();
        board[3][3] = Some(PieceType::White);
        let v: Vec<Move> = vec![];
        assert_eq!(v, get_leagal_moves(&board, loc_33, move_to_33));
        board[4][4] = Some(PieceType::Black);
        board[2][2] = Some(PieceType::Black);
        let mut v = vec![
            Move::new(loc_33, (1, 1).into(), Some((2, 2).into())),
            Move::new(loc_33, (5, 5).into(), Some((4, 4).into())),
        ];
        v.sort();
        assert_eq!(v, get_leagal_moves(&board, loc_33, move_to_33));
    }
    #[test]
    fn basic_queen_movment() {
        let loc_0 = Location::new(0, 0);
        let loc_33 = Location::new(3, 3);
        let move_to_33 = Move::new(loc_0, loc_33, None);
        let move_to_0 = Move::new(loc_0, loc_0, None);
        let mut board = Board::new();
        board[3][3] = Some(PieceType::WhiteQueen);
        let v = vec![
            // right to left cross
            (0, 0),
            (1, 1),
            (2, 2),
            (4, 4),
            (5, 5),
            (6, 6),
            (7, 7),
            // left to right cross
            (0, 6),
            (1, 5),
            (2, 4),
            (4, 2),
            (5, 1),
            (6, 0),
        ];
        let mut v: Vec<Move> = v
            .into_iter()
            .map(|l| Move::new(loc_33, l.into(), None))
            .collect();
        v.sort();

        assert_eq!(v, get_leagal_moves(&board, loc_33, move_to_0));
    }
}
