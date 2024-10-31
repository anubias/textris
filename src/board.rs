use crate::{
    pieces::{Cell, Piece},
    utils::Position,
};

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
        if self.piece.is_none() {
            self.piece = Some(piece);
        }
    }

    pub fn drop_piece_one_row(&mut self) -> bool {
        if let Some(piece) = &mut self.piece {
            if Self::can_piece_move(&self.board, piece) {
                piece.drop_one_row();
                return true;
            }
        }

        false
    }

    pub fn incorporate_piece(&mut self) {
        if let Some(piece) = &self.piece {
            let pos = piece.get_position();

            for row in 0..piece.get_size() {
                for col in 0..piece.get_size() {
                    let (board_row, board_col) = Self::to_board_coord(pos, row, col);
                    if Self::are_board_coords_valid(board_row, board_col) {
                        self.board[board_row][board_col] = self.get_cell_at(board_row, board_col);
                    }
                }
            }

            self.remove_piece();
        }
    }
}

// Private functions
impl Board {
    fn remove_piece(&mut self) {
        self.piece = None;
    }

    fn can_piece_move(board: &[[Cell; BOARD_WIDTH]; BOARD_HEIGHT], piece: &Piece) -> bool {
        let cur_pos = piece.get_position();
        let next_pos = Self::get_new_piece_position(&cur_pos);

        for row in 0..piece.get_size() {
            for col in 0..piece.get_size() {
                if piece.has_cell_at(row, col) {
                    if row < piece.get_size() - 1 {
                        if piece.has_cell_at(row + 1, col) {
                            continue;
                        }
                    }
                    let (next_board_row, next_board_col) =
                        Self::to_board_coord(&next_pos, row, col);

                    if next_board_row >= BOARD_HEIGHT
                        || board[next_board_row][next_board_col] != Cell::Black
                    {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn get_cell_at(&self, row: usize, col: usize) -> Cell {
        if let Some(piece) = self.piece.as_ref() {
            let pos = piece.get_position();
            let size = piece.get_size();

            let result = if Self::inside_piece(pos, size, row, col) {
                let (piece_row, piece_col) = Self::to_piece_coord(pos, size, row, col);
                let piece_cell = if piece.has_cell_at(piece_row, piece_col) {
                    *piece.get_cell_at(piece_row, piece_col)
                } else {
                    self.board[row][col]
                };

                piece_cell
            } else {
                self.board[row][col]
            };

            result
        } else {
            self.board[row][col]
        }
    }

    fn get_new_piece_position(pos: &Position) -> Position {
        Position {
            row: pos.row + 1,
            col: pos.col,
        }
    }

    /// Translates the piece coordinates into board coordinates, by adding to the
    /// piece coordinates the position (top-left) of the piece relative to the board.
    ///
    /// Note: Please take care, that the returned coordinates may not necessarily
    /// be valid coordinates for the board. So please validate the results before
    /// indexing with these coordinates.
    fn to_board_coord(pos: &Position, row: usize, col: usize) -> (usize, usize) {
        (row + pos.row, col + pos.col)
    }

    /// Translates the board coordinates into piece coordinates, by subtracting the
    /// board coordinates the position (top-left) of the piece relative to the board.
    ///
    /// Note: Please note that the returned coordinates are bound to valid piece
    /// coordinates.
    fn to_piece_coord(pos: &Position, size: usize, row: usize, col: usize) -> (usize, usize) {
        let row = row.max(pos.row);
        let col = col.max(pos.col);

        ((row - pos.row).min(size), (col - pos.col).min(size))
    }

    fn inside_piece(pos: &Position, size: usize, row: usize, col: usize) -> bool {
        row >= pos.row && row < pos.row + size && col >= pos.col && col < pos.col + size
    }

    fn are_board_coords_valid(row: usize, col: usize) -> bool {
        row < BOARD_HEIGHT && col < BOARD_WIDTH
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..BOARD_HEIGHT {
            let mut line = String::new();
            for col in 0..BOARD_WIDTH {
                let cell = self.get_cell_at(row, col);
                line = format!("{line}{cell}");
            }
            let _ = writeln!(f, "🧱{line}🧱");
        }

        let mut bottom = String::new();
        for _ in 0..BOARD_WIDTH {
            bottom = format!("{bottom}🧱");
        }

        writeln!(f, "🧱{bottom}🧱")
    }
}
