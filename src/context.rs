use std::io::{Stdout, Write};

use crossterm::{
    cursor::{Hide, MoveTo, MoveToNextLine, Show},
    terminal::{disable_raw_mode, enable_raw_mode, Clear},
    ExecutableCommand, QueueableCommand,
};
use kira::{
    manager::{AudioManager, AudioManagerSettings, DefaultBackend},
    sound::static_sound::{StaticSoundData, StaticSoundHandle},
    tween::Tween,
};
use rand::{rngs::ThreadRng, Rng};

use crate::{
    pieces::{Piece, Tetromino},
    utils::Position,
};

const PIECE_SPAWN_COLUMN: isize = 3;

pub struct Context {
    audio_manager: Option<AudioManager>,
    muted: bool,
    next_piece: Option<Piece>,
    rng: ThreadRng,
    score: u64,
    song: Option<StaticSoundHandle>,
    song_index: u8,
    stdout: Stdout,
    volume: f64,
}

impl Context {
    pub fn new() -> Self {
        Self {
            audio_manager: None,
            muted: false,
            next_piece: None,
            rng: rand::thread_rng(),
            score: 0,
            song: None,
            song_index: 0,
            stdout: std::io::stdout(),
            volume: 0.5,
        }
    }

    pub fn setup(&mut self) -> std::io::Result<()> {
        enable_raw_mode()?;

        self.stdout
            .execute(Clear(crossterm::terminal::ClearType::All))?
            .execute(Hide)?;

        if let Ok(manager) = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()) {
            self.audio_manager = Some(manager);
            self.change_song();
            self.update_volume();
        }

        Ok(())
    }

    pub fn teardown(&mut self) -> std::io::Result<()> {
        self.stdout.execute(Show)?;

        disable_raw_mode()
    }

    #[allow(non_contiguous_range_endpoints)]
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
                0 => write!(self.stdout, "     NEXT PIECE:")?,
                1..5 => {
                    let nl = if let Some(next_piece_line) = next_piece_lines.get(i - 1) {
                        *next_piece_line
                    } else {
                        ""
                    };
                    write!(self.stdout, "          {nl}")?
                }
                6 => write!(self.stdout, "     CONTROL KEYS:")?,
                7 => write!(self.stdout, "          MOVE LEFT:     ⬅️")?,
                8 => write!(self.stdout, "          MOVE RIGHT:    ➡️")?,
                9 => write!(self.stdout, "          DROP SOFT:     ⬇️")?,
                11 => write!(self.stdout, "          ROTATE LEFT:   Z")?,
                12 => write!(self.stdout, "          ROTATE RIGHT:  X")?,
                13 => write!(self.stdout, "          HOLD:          C")?,
                14 => write!(self.stdout, "          DROP HARD:     SPACEBAR")?,
                16 => write!(self.stdout, "          VOLUME:        + / -")?,
                17 => write!(self.stdout, "          MUTE TOGGLE:   M")?,
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

    pub fn mute_toggle(&mut self) {
        if let Some(song) = self.song.as_mut() {
            if self.muted {
                song.resume(Tween::default());
                self.muted = false;
            } else {
                song.pause(Tween::default());
                self.muted = true;
            }
        }
    }

    pub fn volume_down(&mut self) {
        self.volume = 0.0f64.max(self.volume - 0.05);
        self.update_volume();
    }

    pub fn volume_up(&mut self) {
        self.volume = 1.0f64.min(self.volume + 0.05);
        self.update_volume();
    }

    fn generate_random_piece(&mut self) -> Piece {
        let next = self.rng.gen_range(1..=Tetromino::get_count());
        let tetromino = Tetromino::from(next);
        let position = Position::new(tetromino.get_starting_row(), PIECE_SPAWN_COLUMN);

        Piece::new(tetromino, position)
    }

    fn change_song(&mut self) {
        if let Some(manager) = self.audio_manager.as_mut() {
            let path = format!("assets/theme-{}.mp3", self.song_index);

            if let Ok(sound_data) = StaticSoundData::from_file(path) {
                if let Some(old_song) = self.song.as_mut() {
                    old_song.stop(Tween::default());
                }

                if let Ok(mut song) = manager.play(sound_data) {
                    song.set_loop_region(0.0..);

                    self.song = Some(song);
                    self.song_index += 1;
                    self.song_index %= 3;
                }
            }
        }
    }

    fn update_volume(&mut self) {
        if let Some(song) = self.song.as_mut() {
            song.set_volume(self.volume, Tween::default());
        }
    }
}
