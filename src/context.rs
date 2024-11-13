use std::io::{Stdout, Write};

use crossterm::{
    cursor::{Hide, MoveTo, MoveToNextLine, Show},
    terminal::Clear,
    ExecutableCommand, QueueableCommand,
};
use rand::{rngs::ThreadRng, Rng};

use crate::{
    pieces::{Piece, Tetromino},
    utils::Position,
};

const PIECE_SPAWN_COLUMN: isize = 3;

pub struct Context {
    next_piece: Option<Piece>,
    rng: ThreadRng,
    score: u64,
    stdout: Stdout,
}

impl Context {
    pub fn new() -> Self {
        Self {
            next_piece: None,
            rng: rand::thread_rng(),
            score: 0,
            stdout: std::io::stdout(),
        }
    }

    pub fn setup(&mut self) -> std::io::Result<()> {
        self.stdout
            .execute(Clear(crossterm::terminal::ClearType::All))?
            .execute(Hide)?;

        Ok(())
    }

    pub fn teardown(&mut self) -> std::io::Result<()> {
        self.stdout.execute(Show)?;

        Ok(())
    }

    pub fn print_game(&mut self, board: String) -> std::io::Result<()> {
        let next_piece = if let Some(p) = self.next_piece.clone() {
            p.to_string()
        } else {
            String::new()
        };
        let next_piece_lines = next_piece.lines().collect::<Vec<&str>>();

        self.stdout.queue(MoveTo(0, 0))?;

        for (i, line) in board.lines().enumerate() {
            write!(self.stdout, "{line}")?;

            match i {
                1 => write!(self.stdout, "     NEXT PIECE:")?,
                2..6 => {
                    let nl = if let Some(next_piece_line) = next_piece_lines.get(i - 2) {
                        *next_piece_line
                    } else {
                        ""
                    };
                    write!(self.stdout, "          {nl}")?
                }
                8 => write!(self.stdout, "     CONTROL KEYS:")?,
                9 => write!(self.stdout, "          MOVE LEFT:     ⬅️")?,
                10 => write!(self.stdout, "          MOVE RIGHT:    ➡️")?,
                11 => write!(self.stdout, "          DROP SOFT:     ⬇️")?,
                13 => write!(self.stdout, "          ROTATE LEFT:   Z")?,
                14 => write!(self.stdout, "          ROTATE RIGHT:  X")?,
                15 => write!(self.stdout, "          HOLD:          C")?,
                16 => write!(self.stdout, "          DROP HARD:     SPACEBAR")?,
                19 => write!(self.stdout, "     SCORE:              {}", self.score)?,
                _ => {}
            }
            self.stdout.queue(MoveToNextLine(1))?;
        }

        self.stdout.queue(MoveToNextLine(1))?;
        self.stdout.flush()
    }

    pub fn get_piece(&mut self) -> Piece {
        let piece = if let Some(p) = self.next_piece.clone() {
            p
        } else {
            self.generate_random_piece()
        };

        self.next_piece = Some(self.generate_random_piece());

        piece
    }

    pub fn increment_score(&mut self, points: u64) {
        self.score += points;
    }

    fn generate_random_piece(&mut self) -> Piece {
        let next = self.rng.gen_range(1..=Tetromino::get_count());
        let tetromino = Tetromino::from(next);
        let position = Position::new(tetromino.get_starting_row(), PIECE_SPAWN_COLUMN);

        Piece::new(tetromino, position)
    }
}
