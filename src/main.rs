mod checkers;

use checkers::game;

fn main() {
    let board = game::Board::new_game_board();
    board.print_board();
}
