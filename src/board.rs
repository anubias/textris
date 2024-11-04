use crate::{
    pieces::{Cell, Piece},
    utils::{self, Position},
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
                    let (i_br, i_bc) = utils::to_board_coord(pos, row, col);

                    if Self::inside_board(i_br, i_bc) {
                        let (u_br, u_bc) = utils::to_usize(i_br, i_bc);
                        self.board[u_br][u_bc] = self.get_cell_at(u_br, u_bc);
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
        let size = piece.get_size();
        let cur_pos = piece.get_position();
        let next_pos = Self::get_new_piece_position(&cur_pos);

        if Self::is_piece_on_the_board(piece) {
            for row in (0..size).rev() {
                for col in 0..size {
                    if piece.has_cell_at(row, col) {
                        let (i_nbr, i_nbc) = utils::to_board_coord(&next_pos, row, col);

                        if Self::inside_board(i_nbr, i_nbc) {
                            let (u_nbr, u_nbc) = utils::to_usize(i_nbr, i_nbc);
                            if board[u_nbr][u_nbc] != Cell::Black {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                }
            }
            true
        } else {
            false
        }
    }

    fn get_cell_at(&self, board_row: usize, board_col: usize) -> Cell {
        if let Some(piece) = self.piece.as_ref() {
            let pos = piece.get_position();

            let result = if piece.is_inside(board_row, board_col) {
                let (i_pr, i_pc) = utils::to_piece_coord(pos, board_row, board_col);
                // we are already inside the piece, so the piece-coordinates should be proper
                let (u_pr, u_pc) = utils::to_usize(i_pr, i_pc);
                let piece_cell = if piece.has_cell_at(u_pr, u_pc) {
                    *piece.get_cell_at(u_pr, u_pc)
                } else {
                    self.board[board_row][board_col]
                };
                piece_cell
            } else {
                self.board[board_row][board_col]
            };

            result
        } else {
            self.board[board_row][board_col]
        }
    }

    fn get_new_piece_position(pos: &Position) -> Position {
        Position {
            row: pos.row + 1,
            col: pos.col,
        }
    }

    fn inside_board(row: isize, col: isize) -> bool {
        let (i_height, i_width) = utils::to_isize(BOARD_HEIGHT, BOARD_WIDTH);

        utils::is_within_bounds(row, 0, i_height) && utils::is_within_bounds(col, 0, i_width)
    }

    fn is_piece_on_the_board(piece: &Piece) -> bool {
        let pos = piece.get_position();
        let size = piece.get_size();

        for row in 0..size {
            for col in 0..size {
                if piece.has_cell_at(row, col) {
                    let (i_br, i_bc) = utils::to_board_coord(pos, row, col);
                    if !Self::inside_board(i_br, i_bc) {
                        return false;
                    }
                }
            }
        }

        true
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
