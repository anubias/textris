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
    let s = Piece::new(Tetromino::S, Position::new(17, 0));
    board.add_piece(s);
    board.drop_piece_one_row();

    let s = Piece::new(Tetromino::S, Position::new(15, 4));
    board.add_piece(s);
    board.drop_piece_one_row();
    board.drop_piece_one_row();
    board.drop_piece_one_row();

    let mut col = 2;
    loop {
        println!("{board}");
        if !board.drop_piece_one_row() {
            let s = Piece::new(Tetromino::L, Position::new(0, col));
            board.add_piece(s);
            col += 2;
        };

        // add a condition to break the loop and end the game

        std::thread::sleep(Duration::from_millis(100));
    }
}
