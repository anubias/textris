use std::{
    io::{Stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo, MoveToNextLine, Show},
    terminal::{disable_raw_mode, enable_raw_mode, Clear},
    ExecutableCommand, QueueableCommand,
};
use kira::{
    sound::static_sound::{StaticSoundData, StaticSoundHandle},
    AudioManager, AudioManagerSettings, Decibels, DefaultBackend, Semitones, Tween,
};
use rand::{rngs::ThreadRng, Rng};

use crate::{
    pieces::{Piece, Tetromino},
    utils::Score,
};

const LEVEL_INC_LINES: u32 = 5;
const MUSIC_INC_LEVEL: u32 = 6;
const MUSIC_INC_SPEED: u64 = 5;

const VOLUME_MIN: f32 = Decibels::SILENCE.0;
const VOLUME_MAX: f32 = 10.0;
const VOLUME_INC_STEP: f32 = 1.00;

const SONGS_COUNT: u8 = 3;

pub struct Context {
    audio_manager: Option<AudioManager>,
    level: u32,
    muted: bool,
    next_piece: Option<Piece>,
    random_bag: Vec<Tetromino>,
    rng: ThreadRng,
    score: Score,
    song: Option<StaticSoundHandle>,
    song_index: u8,
    stdout: Stdout,
    volume: f32,
}

impl Context {
    pub fn new() -> Self {
        Self {
            audio_manager: None,
            level: 0,
            muted: false,
            next_piece: None,
            random_bag: Vec::new(),
            rng: rand::rng(),
            score: Score::default(),
            song: None,
            song_index: 0,
            stdout: std::io::stdout(),
            volume: 1.0,
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
                0 => {
                    let nl = if let Some(next_piece_line) = next_piece_lines.get(i) {
                        *next_piece_line
                    } else {
                        ""
                    };
                    write!(self.stdout, "     NEXT PIECE:    {nl}")?;
                }
                1..4 => {
                    let nl = if let Some(next_piece_line) = next_piece_lines.get(i) {
                        *next_piece_line
                    } else {
                        ""
                    };

                    write!(self.stdout, "                    {nl}")?
                }
                5 => write!(self.stdout, "     MOVE LEFT:     ⬅️")?,
                6 => write!(self.stdout, "     MOVE RIGHT:    ➡️")?,
                7 => write!(self.stdout, "     DROP SOFT:     ⬇️")?,
                9 => write!(self.stdout, "     ROTATE LEFT:   Z")?,
                10 => write!(self.stdout, "     ROTATE RIGHT:  X")?,
                11 => write!(self.stdout, "     HOLD:          C")?,
                12 => write!(self.stdout, "     DROP HARD:     SPACEBAR")?,
                14 => write!(self.stdout, "     VOLUME:        + / -")?,
                15 => write!(self.stdout, "     MUTE TOGGLE:   M")?,
                17 => write!(self.stdout, "     LEVEL:         {}", self.level + 1)?,
                18 => write!(
                    self.stdout,
                    "     LINES:         {}",
                    self.score.lines_destroyed
                )?,
                19 => write!(self.stdout, "     POINTS:        {}", self.score.points)?,
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
            self.take_from_random_bag()
        };

        self.next_piece = Some(self.take_from_random_bag());

        piece
    }

    pub fn increment_score(&mut self, score: Score) {
        let prev_level = self.level;

        self.score.increment(score);
        self.level = self.score.lines_destroyed as u32 / LEVEL_INC_LINES;

        if self.level > prev_level {
            if self.level.is_multiple_of(MUSIC_INC_LEVEL) {
                self.change_song();
            } else {
                self.update_playback_rate(self.level % MUSIC_INC_LEVEL + 1);
            }
        }
    }

    pub fn get_game_speed_micros(&self) -> f64 {
        let float_level = self.level as f64;

        (0.8 - (float_level * 0.007)).powf(float_level)
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
        self.volume = VOLUME_MIN.max(self.volume - VOLUME_INC_STEP);
        self.update_volume();
    }

    pub fn volume_up(&mut self) {
        self.volume = VOLUME_MAX.min(self.volume + VOLUME_INC_STEP);
        self.update_volume();
    }

    fn take_from_random_bag(&mut self) -> Piece {
        if self.random_bag.is_empty() {
            self.refill_random_bag();
        }

        let tetromino = self.random_bag.pop().unwrap_or(Tetromino::O);
        let position = tetromino.get_spawn_position();

        Piece::new(tetromino, position)
    }

    fn refill_random_bag(&mut self) {
        let mut bag = vec![
            Tetromino::I,
            Tetromino::J,
            Tetromino::L,
            Tetromino::O,
            Tetromino::S,
            Tetromino::T,
            Tetromino::Z,
        ];

        self.random_bag = Vec::new();
        while !bag.is_empty() {
            let index = self.rng.random_range(0..bag.len());
            let tetromino = bag.remove(index);
            self.random_bag.push(tetromino);
        }
    }

    fn change_song(&mut self) {
        if let Some(manager) = self.audio_manager.as_mut() {
            let path = format!("assets/theme-{}.mp3", self.song_index);

            if let Ok(sound_data) = StaticSoundData::from_file(path) {
                if let Some(old_song) = self.song.as_mut() {
                    old_song.stop(Tween::default());
                }

                if let Ok(mut song) = manager.play(sound_data) {
                    if self.song_index == 0 {
                        // this song has a 3 second pause at the beginning
                        song.seek_by(3.0);
                        song.set_loop_region(3.0..);
                    } else {
                        song.set_loop_region(0.0..);
                    }
                    self.song = Some(song);
                    self.update_volume();

                    self.song_index += 1;
                    self.song_index %= SONGS_COUNT;
                }
            }
        }
    }

    fn update_volume(&mut self) {
        if let Some(song) = self.song.as_mut() {
            song.set_volume(self.volume, Tween::default());
        }
    }

    fn update_playback_rate(&mut self, rate: u32) {
        if let Some(song) = self.song.as_mut() {
            song.set_playback_rate(
                Semitones(rate as f64),
                Tween {
                    duration: Duration::from_secs(MUSIC_INC_SPEED),
                    ..Default::default()
                },
            );
        }
    }
}
