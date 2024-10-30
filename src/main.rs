mod board;
mod pieces;

use board::Board;

fn main() {
    let board = Board::new();
    println!("{board}");
}
