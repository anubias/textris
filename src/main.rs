mod board;
mod pieces;
mod utils;

use std::time::Duration;

use board::Board;
use pieces::{Piece, Tetromino};
use utils::Position;

fn main() {
    main_loop();
}

fn main_loop() {
    let mut board = Board::new();
    let s = Piece::new(Tetromino::S, Position::new(0, 0));
    board.add_piece(s);

    loop {
        std::thread::sleep(Duration::from_secs(1));
        println!("{board}");

        // add a condition to break the loop and end the game
    }
}
