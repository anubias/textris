mod board;
mod context;
mod pieces;
mod utils;

use std::time::{Duration, Instant};

use crossterm::event::{poll, read, Event, KeyCode};

use board::Board;
use context::Context;
use utils::{Direction, Score};

const PIECE_DROP_MICROSECONDS: f64 = 1_000_000.0;

fn main() -> std::io::Result<()> {
    let mut context = Context::new();
    context.setup()?;
    game_loop(&mut context)?;
    context.teardown()
}

fn game_loop(context: &mut Context) -> std::io::Result<()> {
    let mut board = Board::new();
    let mut paused = false;
    let mut speed_micros = context.get_game_speed_micros();
    let mut now = Instant::now();

    loop {
        if !board.has_piece() && !board.add_piece(context.get_piece()) {
            break;
        }

        context.print_game(format!("{board}"))?;

        if poll(Duration::from_millis(1))? {
            let event = read()?;

            let score = if event == Event::Key(KeyCode::Esc.into()) {
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
                Score::default()
            } else if event == Event::Key(KeyCode::Char('x').into())
                || event == Event::Key(KeyCode::Char('X').into())
            {
                board.rotate_piece(utils::Rotation::Clockwise);
                Score::default()
            } else if event == Event::Key(KeyCode::Char('c').into())
                || event == Event::Key(KeyCode::Char('C').into())
            {
                paused = !paused;
                Score::default()
            } else if event == Event::Key(KeyCode::Char('m').into())
                || event == Event::Key(KeyCode::Char('M').into())
            {
                context.mute_toggle();
                Score::default()
            } else if event == Event::Key(KeyCode::Char(' ').into()) {
                board.land_piece()
            } else if event == Event::Key(KeyCode::Char('-').into()) {
                context.volume_down();
                Score::default()
            } else if event == Event::Key(KeyCode::Char('+').into()) {
                context.volume_up();
                Score::default()
            } else {
                Score::default()
            };

            context.increment_score(score);
        }

        if paused {
            continue;
        }

        if (now.elapsed().as_micros() as f64) >= (PIECE_DROP_MICROSECONDS * speed_micros) {
            context.increment_score(board.move_piece(Direction::Down).1);
            speed_micros = context.get_game_speed_micros();
            now = Instant::now();
        }
    }

    Ok(())
}
