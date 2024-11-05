mod board;
mod pieces;
mod utils;

use std::time::Duration;

use board::Board;
use pieces::{Piece, Tetromino};
use utils::{Direction, Position};

fn main() {
    main_loop();
}

fn main_loop() {
    let mut board = Board::new();
    let s = Piece::new(Tetromino::S, Position::new(17, 0));
    board.add_piece(s);
    board.move_piece(Direction::Down);

    let s = Piece::new(Tetromino::S, Position::new(15, 4));
    board.add_piece(s);
    board.move_piece(Direction::Down);
    board.move_piece(Direction::Down);
    board.move_piece(Direction::Down);

    loop {
        println!("{board}");
        if !board.has_piece() {
            let s = Piece::new(Tetromino::L, Position::new(0, 2));
            board.add_piece(s);
            board.rotate_piece(utils::Rotation::CounterClockwise);
            board.rotate_piece(utils::Rotation::CounterClockwise);
        } else {
            if !board.move_piece(Direction::Left) {
                board.move_piece(Direction::Down);
            }
        };

        // add a condition to break the loop and end the game

        std::thread::sleep(Duration::from_millis(100));
    }
}
