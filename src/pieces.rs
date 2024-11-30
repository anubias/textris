use crate::utils::{self, Direction, Position, Rotation};

const SHAPE_SIZE: usize = 4;
const SHAPE_COUNT: usize = 7;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Cell {
    #[default]
    Black,
    Blue,
    Brown,
    Green,
    Orange,
    Purple,
    Red,
    Yellow,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Cell::Black => '⬛',
            Cell::Blue => '🟦',
            Cell::Brown => '🟫',
            Cell::Green => '🟩',
            Cell::Orange => '🟧',
            Cell::Purple => '🟪',
            Cell::Red => '🟥',
            Cell::Yellow => '🟨',
        };
        write!(f, "{c}")
    }
}

#[derive(Clone)]
pub enum Tetromino {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl From<usize> for Tetromino {
    fn from(value: usize) -> Self {
        match value {
            1 => Self::I,
            2 => Self::J,
            3 => Self::L,
            4 => Self::O,
            5 => Self::S,
            6 => Self::T,
            7 => Self::Z,
            _ => Self::O,
        }
    }
}

impl Tetromino {
    pub fn get_count() -> usize {
        SHAPE_COUNT
    }

    pub fn get_spawn_position(&self) -> Position {
        match self {
            Self::I | Self::L => Position::new(0, 3),
            Self::J => Position::new(0, 4),
            _ => Position::new(-1, 3),
        }
    }

    fn get_shape(&self) -> [[Cell; SHAPE_SIZE]; SHAPE_SIZE] {
        let black = Cell::default();

        match self {
            Tetromino::I => {
                let brown = Cell::Brown;
                [
                    [black, brown, black, black],
                    [black, brown, black, black],
                    [black, brown, black, black],
                    [black, brown, black, black],
                ]
            }
            Tetromino::J => {
                let blue = Cell::Blue;
                [
                    [black, blue, black, black],
                    [black, blue, black, black],
                    [blue, blue, black, black],
                    [black, black, black, black],
                ]
            }
            Tetromino::L => {
                let orange = Cell::Orange;
                [
                    [black, orange, black, black],
                    [black, orange, black, black],
                    [black, orange, orange, black],
                    [black, black, black, black],
                ]
            }
            Tetromino::O => {
                let yellow = Cell::Yellow;
                [
                    [black, black, black, black],
                    [black, yellow, yellow, black],
                    [black, yellow, yellow, black],
                    [black, black, black, black],
                ]
            }
            Tetromino::S => {
                let green = Cell::Green;
                [
                    [black, black, black, black],
                    [black, green, green, black],
                    [green, green, black, black],
                    [black, black, black, black],
                ]
            }
            Tetromino::T => {
                let purple = Cell::Purple;
                [
                    [black, black, black, black],
                    [purple, purple, purple, black],
                    [black, purple, black, black],
                    [black, black, black, black],
                ]
            }
            Tetromino::Z => {
                let red = Cell::Red;
                [
                    [black, black, black, black],
                    [red, red, black, black],
                    [black, red, red, black],
                    [black, black, black, black],
                ]
            }
        }
    }
}

#[derive(Clone)]
pub struct Piece {
    position: Position,
    orientation: Direction,
    shape: [[Cell; SHAPE_SIZE]; SHAPE_SIZE],
    tetromino: Tetromino,
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let shape = self.tetromino.get_shape();
        for row in shape.iter().take(SHAPE_SIZE) {
            let mut line = String::new();
            for cell in row.iter().take(SHAPE_SIZE) {
                line = format!("{line}{}", cell);
            }
            let _ = writeln!(f, "{line}");
        }

        Ok(())
    }
}

impl Piece {
    pub fn new(tetromino: Tetromino, position: Position) -> Self {
        Self {
            position,
            orientation: Direction::Up,
            shape: tetromino.get_shape(),
            tetromino,
        }
    }

    pub fn slide(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.position.row -= 1,
            Direction::Down => self.position.row += 1,
            Direction::Left => self.position.col -= 1,
            Direction::Right => self.position.col += 1,
        }
    }

    pub fn rotate(&mut self, rotation: &Rotation) {
        self.orientation = match rotation {
            Rotation::Clockwise => match self.orientation {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
            Rotation::CounterClockwise => match self.orientation {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
        };

        self.rotate_shape();
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn get_size(&self) -> usize {
        SHAPE_SIZE
    }

    pub fn get_cell_at(&self, row: usize, col: usize) -> &Cell {
        &self.shape[row][col]
    }

    pub fn has_cell_at(&self, row: usize, col: usize) -> bool {
        self.get_cell_at(row, col) != &Cell::Black
    }

    pub fn is_inside(&self, row: usize, col: usize) -> bool {
        let pos = self.get_position();
        let size = self.get_size();

        let (row, col) = utils::to_piece_coord(pos, row, col);
        let i_size = size as isize;

        utils::is_within_bounds(row, 0, i_size) && utils::is_within_bounds(col, 0, i_size)
    }
}

//Private functions
impl Piece {
    #[allow(clippy::needless_range_loop)]
    fn rotate_shape(&mut self) {
        let template = self.tetromino.get_shape();
        let mut new_shape: [[Cell; SHAPE_SIZE]; SHAPE_SIZE] =
            [[Cell::Black; SHAPE_SIZE]; SHAPE_SIZE];

        let piece_size = match self.tetromino {
            Tetromino::I | Tetromino::O => SHAPE_SIZE,
            _ => SHAPE_SIZE - 1,
        };

        self.shape = match self.orientation {
            Direction::Up => template,
            Direction::Down => {
                for row in (0..piece_size).rev() {
                    for col in (0..piece_size).rev() {
                        new_shape[piece_size - 1 - row][piece_size - 1 - col] = template[row][col];
                    }
                }
                new_shape
            }
            Direction::Left => {
                for col in (0..piece_size).rev() {
                    for row in 0..piece_size {
                        new_shape[piece_size - 1 - col][row] = template[row][col];
                    }
                }
                new_shape
            }
            Direction::Right => {
                for col in 0..piece_size {
                    for row in (0..piece_size).rev() {
                        new_shape[col][piece_size - 1 - row] = template[row][col];
                    }
                }
                new_shape
            }
        }
    }
}
