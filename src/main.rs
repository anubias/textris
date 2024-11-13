mod board;
mod context;
mod pieces;
mod utils;

use std::time::{Duration, Instant};

use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use board::Board;
use context::Context;
use utils::Direction;

const PIECE_DROP_MILLISECONDS: u128 = 500;

fn main() -> std::io::Result<()> {
    let mut context = Context::new();
    context.setup()?;
    game_loop(&mut context)?;
    context.teardown()
}

fn game_loop(context: &mut Context) -> std::io::Result<()> {
    enable_raw_mode()?;

    let mut board = Board::new();
    let mut now = Instant::now();
    let mut paused = false;

    loop {
        if !board.has_piece() {
            board.add_piece(context.get_piece());
        }

        context.print_game(format!("{board}"))?;

        if poll(Duration::from_millis(100))? {
            let event = read()?;

            let points = if event == Event::Key(KeyCode::Esc.into()) {
                break;
            } else if event == Event::Key(KeyCode::Left.into()) {
                board.move_piece(Direction::Left).1
            } else if event == Event::Key(KeyCode::Right.into()) {
                board.move_piece(Direction::Right).1
            } else if event == Event::Key(KeyCode::Down.into()) {
                board.move_piece(Direction::Down).1
            } else if event == Event::Key(KeyCode::Char('z').into())
                || event == Event::Key(KeyCode::Char('Z').into())
            {
                board.rotate_piece(utils::Rotation::CounterClockwise);
                0
            } else if event == Event::Key(KeyCode::Char('x').into())
                || event == Event::Key(KeyCode::Char('X').into())
            {
                board.rotate_piece(utils::Rotation::Clockwise);
                0
            } else if event == Event::Key(KeyCode::Char('c').into())
                || event == Event::Key(KeyCode::Char('C').into())
            {
                paused = !paused;
                0
            } else if event == Event::Key(KeyCode::Char(' ').into()) {
                board.land_piece()
            } else {
                0
            };

            context.increment_score(points);
        }

        if paused {
            continue;
        }

        let elapsed = now.elapsed();
        if elapsed.as_millis() >= PIECE_DROP_MILLISECONDS {
            context.increment_score(board.move_piece(Direction::Down).1);
            now = Instant::now();
        }
    }

    disable_raw_mode()
}
