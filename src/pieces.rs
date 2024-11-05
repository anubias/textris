use crate::utils::{self, Direction, Position};

const SHAPE_SIZE: usize = 4;

#[derive(Clone, Copy, Default, PartialEq)]
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

#[derive(Clone, Copy)]
pub enum Tetromino {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl Tetromino {
    fn get_shape(&self) -> [[Cell; SHAPE_SIZE]; SHAPE_SIZE] {
        match self {
            Tetromino::I => {
                let cell = Cell::Brown;
                [
                    [Cell::default(), cell, Cell::default(), Cell::default()],
                    [Cell::default(), cell, Cell::default(), Cell::default()],
                    [Cell::default(), cell, Cell::default(), Cell::default()],
                    [Cell::default(), cell, Cell::default(), Cell::default()],
                ]
            }
            Tetromino::J => {
                let cell = Cell::Blue;
                [
                    [Cell::default(), Cell::default(), cell, Cell::default()],
                    [Cell::default(), Cell::default(), cell, Cell::default()],
                    [Cell::default(), cell, cell, Cell::default()],
                    [
                        Cell::default(),
                        Cell::default(),
                        Cell::default(),
                        Cell::default(),
                    ],
                ]
            }
            Tetromino::L => {
                let cell = Cell::Orange;
                [
                    [Cell::default(), cell, Cell::default(), Cell::default()],
                    [Cell::default(), cell, Cell::default(), Cell::default()],
                    [Cell::default(), cell, cell, Cell::default()],
                    [
                        Cell::default(),
                        Cell::default(),
                        Cell::default(),
                        Cell::default(),
                    ],
                ]
            }
            Tetromino::O => {
                let cell = Cell::Yellow;
                [
                    [
                        Cell::default(),
                        Cell::default(),
                        Cell::default(),
                        Cell::default(),
                    ],
                    [Cell::default(), cell, cell, Cell::default()],
                    [Cell::default(), cell, cell, Cell::default()],
                    [
                        Cell::default(),
                        Cell::default(),
                        Cell::default(),
                        Cell::default(),
                    ],
                ]
            }
            Tetromino::S => {
                let cell = Cell::Green;
                [
                    [
                        Cell::default(),
                        Cell::default(),
                        Cell::default(),
                        Cell::default(),
                    ],
                    [Cell::default(), cell, cell, Cell::default()],
                    [cell, cell, Cell::default(), Cell::default()],
                    [
                        Cell::default(),
                        Cell::default(),
                        Cell::default(),
                        Cell::default(),
                    ],
                ]
            }
            Tetromino::T => {
                let cell = Cell::Purple;
                [
                    [
                        Cell::default(),
                        Cell::default(),
                        Cell::default(),
                        Cell::default(),
                    ],
                    [cell, cell, cell, Cell::default()],
                    [Cell::default(), cell, Cell::default(), Cell::default()],
                    [
                        Cell::default(),
                        Cell::default(),
                        Cell::default(),
                        Cell::default(),
                    ],
                ]
            }
            Tetromino::Z => {
                let cell = Cell::Red;
                [
                    [
                        Cell::default(),
                        Cell::default(),
                        Cell::default(),
                        Cell::default(),
                    ],
                    [Cell::default(), cell, cell, Cell::default()],
                    [Cell::default(), Cell::default(), cell, cell],
                    [
                        Cell::default(),
                        Cell::default(),
                        Cell::default(),
                        Cell::default(),
                    ],
                ]
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Piece {
    position: Position,
    shape: [[Cell; SHAPE_SIZE]; SHAPE_SIZE],
    tetromino: Tetromino,
}

impl Piece {
    pub fn new(tetromino: Tetromino, position: Position) -> Self {
        Self {
            position,
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
