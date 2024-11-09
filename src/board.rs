use crate::{
    pieces::{Cell, Piece},
    utils::{self, Direction, Rotation},
};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

pub struct Board {
    board: [[Cell; BOARD_WIDTH]; BOARD_HEIGHT],
    piece: Option<Piece>,
}

// Public functions
impl Board {
    pub fn new() -> Self {
        Self {
            board: [[Cell::default(); BOARD_WIDTH]; BOARD_HEIGHT],
            piece: None,
        }
    }

    pub fn add_piece(&mut self, piece: Piece) -> bool {
        if self.piece.is_none()
            && Self::is_piece_on_the_board(&piece)
            && !Self::does_piece_overlap(&self.board, &piece)
        {
            self.piece = Some(piece);
            true
        } else {
            false
        }
    }

    pub fn move_piece(&mut self, direction: Direction) -> bool {
        if let Some(piece) = &mut self.piece {
            if Self::can_piece_slide(&self.board, piece, &direction) {
                piece.slide(&direction);
                return true;
            }
            if direction == Direction::Down {
                self.incorporate_piece();
            }
        }

        false
    }

    pub fn rotate_piece(&mut self, rotation: Rotation) {
        if let Some(p) = self.piece.as_mut() {
            p.rotate(rotation);
        }
    }

    pub fn has_piece(&self) -> bool {
        self.piece.is_some()
    }
}

// Private functions
impl Board {
    fn remove_piece(&mut self) {
        self.piece = None;
    }

    fn can_piece_slide(
        board: &[[Cell; BOARD_WIDTH]; BOARD_HEIGHT],
        piece: &Piece,
        direction: &Direction,
    ) -> bool {
        if Self::is_piece_on_the_board(piece) {
            let mut virt_piece = piece.clone();
            virt_piece.slide(direction);

            !Self::does_piece_overlap(board, &virt_piece)
        } else {
            false
        }
    }

    fn incorporate_piece(&mut self) {
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

    fn does_piece_overlap(board: &[[Cell; BOARD_WIDTH]; BOARD_HEIGHT], piece: &Piece) -> bool {
        let pos = piece.get_position();
        let size = piece.get_size();

        for row in 0..size {
            for col in 0..size {
                if piece.has_cell_at(row, col) {
                    let (i_nbr, i_nbc) = utils::to_board_coord(&pos, row, col);

                    if Self::inside_board(i_nbr, i_nbc) {
                        let (u_nbr, u_nbc) = utils::to_usize(i_nbr, i_nbc);
                        if board[u_nbr][u_nbc] != Cell::Black {
                            return true;
                        }
                    } else {
                        return true;
                    }
                }
            }
        }

        false
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

#[cfg(test)]
mod tests {
    use utils::Position;

    use super::*;

    #[test]
    fn piece_addition_too_high() {
        let mut board = Board::new();

        let pos = Position { row: -2, col: 3 };
        let piece = Piece::new(crate::pieces::Tetromino::O, pos);
        assert!(!board.add_piece(piece));
    }

    #[test]
    fn piece_addition_top_edge() {
        let mut board = Board::new();

        let pos = Position { row: -1, col: 3 };
        let piece = Piece::new(crate::pieces::Tetromino::O, pos);
        assert!(board.add_piece(piece));
    }

    #[test]
    fn piece_addition_too_left() {
        let mut board = Board::new();

        let pos = Position { row: 9, col: -2 };
        let piece = Piece::new(crate::pieces::Tetromino::O, pos);
        assert!(!board.add_piece(piece));
    }

    #[test]
    fn piece_addition_left_edge() {
        let mut board = Board::new();

        let pos = Position { row: 9, col: -1 };
        let piece = Piece::new(crate::pieces::Tetromino::O, pos);
        assert!(board.add_piece(piece));
    }

    #[test]
    fn piece_addition_too_right() {
        let mut board = Board::new();

        let pos = Position { row: 9, col: 8 };
        let piece = Piece::new(crate::pieces::Tetromino::O, pos);
        assert!(!board.add_piece(piece));
    }

    #[test]
    fn piece_addition_right_edge() {
        let mut board = Board::new();

        let pos = Position { row: 9, col: 7 };
        let piece = Piece::new(crate::pieces::Tetromino::O, pos);
        assert!(board.add_piece(piece));
    }

    #[test]
    fn piece_addition_overlap() {
        let mut board = Board::new();

        let pos = Position { row: 15, col: 3 };
        let piece_o = Piece::new(crate::pieces::Tetromino::O, pos);
        let pos = Position { row: 14, col: 4 };
        let piece_l = Piece::new(crate::pieces::Tetromino::L, pos);

        assert!(board.add_piece(piece_o));
        board.incorporate_piece();

        assert!(!board.add_piece(piece_l));
    }

    #[test]
    fn drop_i_piece_on_bottom() {
        let mut board = Board::new();

        let pos = Position { row: 15, col: 0 };
        let piece = Piece::new(crate::pieces::Tetromino::I, pos);

        assert!(board.add_piece(piece));
        assert!(board.move_piece(Direction::Down));
        assert!(!board.move_piece(Direction::Down));
    }

    #[test]
    fn drop_s_piece_on_bottom() {
        let mut board = Board::new();

        let pos = Position { row: 15, col: 0 };
        let piece = Piece::new(crate::pieces::Tetromino::S, pos);

        assert!(board.add_piece(piece));
        assert!(board.move_piece(Direction::Down));
        assert!(board.move_piece(Direction::Down));
        assert!(!board.move_piece(Direction::Down));
    }

    #[test]
    fn stack_pieces_simple() {
        let mut board = Board::new();

        let pos = Position { row: 15, col: 3 };
        let piece_o = Piece::new(crate::pieces::Tetromino::O, pos);
        let pos = Position { row: 14, col: 4 };
        let piece_l = Piece::new(crate::pieces::Tetromino::L, pos);

        assert!(board.add_piece(piece_o));
        assert!(board.move_piece(Direction::Down));
        assert!(board.move_piece(Direction::Down));
        assert!(!board.move_piece(Direction::Down));

        assert!(board.add_piece(piece_l));
        assert!(board.move_piece(Direction::Down));
        assert!(!board.move_piece(Direction::Down));
    }

    #[test]
    fn stack_pieces_complex() {
        let mut board = Board::new();

        let pos = Position { row: 16, col: 1 };
        let piece_z = Piece::new(crate::pieces::Tetromino::Z, pos);
        let pos = Position { row: 16, col: 6 };
        let piece_s = Piece::new(crate::pieces::Tetromino::S, pos);
        let pos = Position { row: 13, col: 4 };
        let piece_t = Piece::new(crate::pieces::Tetromino::T, pos);

        assert!(board.add_piece(piece_z));
        board.incorporate_piece();

        assert!(board.add_piece(piece_s));
        board.incorporate_piece();

        assert!(board.add_piece(piece_t));
        assert!(board.move_piece(Direction::Down));
        assert!(board.move_piece(Direction::Down));
        assert!(board.move_piece(Direction::Down));
        assert!(!board.move_piece(Direction::Down));
    }
}
