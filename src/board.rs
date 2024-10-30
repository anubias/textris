use crate::pieces::{Cell, Piece};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

pub struct Board {
    board: [[Cell; BOARD_WIDTH]; BOARD_HEIGHT],
    piece: Option<Piece>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [[Cell::default(); BOARD_WIDTH]; BOARD_HEIGHT],
            piece: None,
        }
    }

    pub fn add_piece(&mut self, piece: Piece) {
        self.piece = Some(piece);
    }

    pub fn drop_piece_one_row(&mut self) {
        if let Some(p) = &mut self.piece {
            p.drop_one_row();
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..BOARD_HEIGHT {
            let mut line = String::new();
            for col in 0..BOARD_WIDTH {
                let print_cell = if let Some(p) = &self.piece {
                    let piece_position = p.get_position();
                    let size = p.get_size();

                    let cell = if row >= piece_position.row
                        && row < piece_position.row + size
                        && col >= piece_position.col
                        && col < piece_position.col + size
                    {
                        let piece_row = row - piece_position.row;
                        let piece_col = col - piece_position.col;
                        let piece_cell = p.get_cell_at(piece_row, piece_col);
                        let piece_cell = if piece_cell == &Cell::Black {
                            &self.board[row][col]
                        } else {
                            piece_cell
                        };

                        piece_cell
                    } else {
                        &self.board[row][col]
                    };

                    cell
                } else {
                    &self.board[row][col]
                };

                line = format!("{line}{print_cell}");
            }
            let _ = writeln!(f, "🧱{line}🧱");
        }

        let mut bottom = String::new();
        for _ in 0..BOARD_WIDTH + 2 {
            bottom = format!("{bottom}🧱");
        }

        writeln!(f, "{bottom}")
    }
}
