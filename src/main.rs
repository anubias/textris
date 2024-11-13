mod board;
mod pieces;
mod utils;

use std::{
    io::{Stdout, Write},
    time::{Duration, Instant},
};

use crossterm::{
    cursor::{Hide, MoveTo, MoveToNextLine, Show},
    event::{poll, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, Clear},
    ExecutableCommand, QueueableCommand,
};
use rand::{rngs::ThreadRng, Rng};

use board::Board;
use pieces::{Piece, Tetromino};
use utils::{Direction, Position};

const PIECE_DROP_MILLISECONDS: u128 = 500;
const PIECE_SPAWN_COLUMN: isize = 3;

struct Context {
    rng: ThreadRng,
    score: u64,
    stdout: Stdout,
}

impl Context {
    fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            score: 0,
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

    fn print_game(&mut self, board: String) -> std::io::Result<()> {
        self.stdout.queue(MoveTo(0, 0))?;

        for (i, line) in board.lines().enumerate() {
            write!(self.stdout, "{line}")?;

            match i {
                18 => write!(self.stdout, "          SCORE: {}", self.score)?,
                _ => {}
            }
            self.stdout.queue(MoveToNextLine(1))?;
        }

        self.stdout.flush()
    }

    fn get_next_piece(&mut self) -> Piece {
        let next = self.rng.gen_range(1..Tetromino::get_count() + 1);
        let tetromino = Tetromino::from(next);
        let position = Position::new(tetromino.get_starting_row(), PIECE_SPAWN_COLUMN);

        Piece::new(tetromino, position)
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
    let mut paused = false;

    loop {
        context.print_game(format!("{board}"))?;

        if poll(Duration::from_millis(100))? {
            let event = read()?;

            context.score += if event == Event::Key(KeyCode::Esc.into()) {
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
            }
        }

        if paused {
            continue;
        }

        if !board.has_piece() {
            let piece = context.get_next_piece();
            board.add_piece(piece);
        }

        let elapsed = now.elapsed();
        if elapsed.as_millis() >= PIECE_DROP_MILLISECONDS {
            context.score += board.move_piece(Direction::Down).1;
            now = Instant::now();
        }
    }

    disable_raw_mode()
}
