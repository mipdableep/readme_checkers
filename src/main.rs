mod checkers;

use checkers::game::{get_leagal_moves, Board, Location, Move, PieceType};

fn main() {
    let loc_0 = Location::new(0, 0);
    let loc_33 = Location::new(3, 3);
    let move_0 = Move::new(loc_0, loc_0);
    let move_to_33 = Move::new(loc_0, loc_33);
    let mut board = Board::new();
    board[3][3] = Some(PieceType::White);
    board[4][2] = Some(PieceType::White);
    board[4][4] = Some(PieceType::Black);
    board[2][4] = Some(PieceType::Black);
    let mut v = vec![Location::new(5, 5)];
    v.sort();
    board.print_board_index();
    assert_eq!(v, get_leagal_moves(&board, loc_33, move_0));
    let mut v = vec![Location::new(5, 5), Location::new(5, 2)];
    v.sort();
    assert_eq!(v, get_leagal_moves(&board, loc_33, move_to_33));
}
