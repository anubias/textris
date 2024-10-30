use crate::utils::Position;

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

    pub fn drop_one_row(&mut self) {
        self.position.row += 1;
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
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let shape = self.tetromino.get_shape();
        for i in 0..SHAPE_SIZE {
            let mut line = String::new();
            for j in 0..SHAPE_SIZE {
                line = format!("{line}{}", shape[i][j]);
            }
            let _ = writeln!(f, "{line}");
        }

        write!(f, "")
    }
}
