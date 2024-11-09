mod board;
mod pieces;
mod utils;

use std::{
    io::{Stdout, Write},
    time::{Duration, Instant},
};

use board::Board;
use crossterm::{
    cursor::{Hide, MoveTo, MoveToNextLine, Show},
    event::{poll, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, Clear},
    ExecutableCommand, QueueableCommand,
};
use pieces::{Piece, Tetromino};
use utils::{Direction, Position};

const PIECE_DROP_MILLISECONDS: u128 = 500;

struct Context {
    stdout: Stdout,
}

impl Context {
    fn new() -> Self {
        Self {
            stdout: std::io::stdout(),
        }
    }

    fn setup(&mut self) -> std::io::Result<()> {
        self.stdout
            .execute(Clear(crossterm::terminal::ClearType::All))?
            .execute(Hide)?;

        Ok(())
    }

    fn teardown(&mut self) -> std::io::Result<()> {
        self.stdout.execute(Show)?;

        Ok(())
    }

    fn print_board(&mut self, board: String) -> std::io::Result<()> {
        self.stdout.queue(MoveTo(0, 0))?;

        for line in board.lines() {
            write!(std::io::stdout(), "{line}")?;
            self.stdout.queue(MoveToNextLine(1))?;
        }

        self.stdout.flush()
    }
}

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

    loop {
        context.print_board(format!("{board}"))?;

        if poll(Duration::from_millis(100))? {
            let event = read()?;

            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            } else if event == Event::Key(KeyCode::Left.into()) {
                board.move_piece(Direction::Left);
            } else if event == Event::Key(KeyCode::Right.into()) {
                board.move_piece(Direction::Right);
            } else if event == Event::Key(KeyCode::Down.into()) {
                board.move_piece(Direction::Down);
            } else if event == Event::Key(KeyCode::Char('f').into()) {
                board.rotate_piece(utils::Rotation::CounterClockwise);
            } else if event == Event::Key(KeyCode::Char('g').into()) {
                board.rotate_piece(utils::Rotation::Clockwise);
            } else if event == Event::Key(KeyCode::Char(' ').into()) {
                // land piece all the way down
            }
        }

        if !board.has_piece() {
            let s = Piece::new(Tetromino::L, Position::new(0, 2));
            board.add_piece(s);
        }

        let elapsed = now.elapsed();
        if elapsed.as_millis() >= PIECE_DROP_MILLISECONDS {
            board.move_piece(Direction::Down);
            now = Instant::now();
        }
    }

    disable_raw_mode()
}
