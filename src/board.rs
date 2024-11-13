use crate::{
    pieces::{Cell, Piece},
    utils::{self, Direction, Rotation},
};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;
const LINE_CLEAR_POINTS: [u64; 5] = [0, 40, 100, 300, 1200];

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

    pub fn move_piece(&mut self, direction: Direction) -> (bool, u64) {
        let mut points = 0;

        if let Some(p) = self.piece.as_mut() {
            if Self::can_piece_slide(&self.board, p, &direction) {
                p.slide(&direction);
                return (true, 0);
            }
            if direction == Direction::Down {
                points = self.incorporate_piece();
            }
        }

        (false, points)
    }

    pub fn land_piece(&mut self) -> u64 {
        let mut lines_dropped = 0;

        loop {
            let (moved, mut points) = self.move_piece(Direction::Down);
            if moved {
                lines_dropped += 1;
            } else {
                if points > 0 {
                    points = points + lines_dropped + 1;
                }
                return points;
            }
        }
    }

    pub fn rotate_piece(&mut self, rotation: Rotation) -> bool {
        if let Some(p) = self.piece.as_mut() {
            // Check if 'in-place' rotation is allowed, and rotate if true
            if Self::can_piece_rotate(&self.board, p, &rotation) {
                p.rotate(&rotation);
                return true;
            }
            if Self::can_piece_slide(&self.board, p, &Direction::Left) {
                p.slide(&Direction::Left);
                // Try sliding the piece to the left, and attempt a rotation there
                if Self::can_piece_rotate(&self.board, p, &rotation) {
                    p.rotate(&rotation);
                    return true;
                }
                p.slide(&Direction::Right); // undo the slide
            }
            if Self::can_piece_slide(&self.board, p, &Direction::Right) {
                p.slide(&Direction::Right);
                // Try sliding the piece to the right, and attempt a rotation there
                if Self::can_piece_rotate(&self.board, p, &rotation) {
                    p.rotate(&rotation);
                    return true;
                }
                p.slide(&Direction::Left); // undo the slide
            }
        }

        false
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

    fn can_piece_rotate(
        board: &[[Cell; BOARD_WIDTH]; BOARD_HEIGHT],
        piece: &Piece,
        rotation: &Rotation,
    ) -> bool {
        if Self::is_piece_on_the_board(piece) {
            let mut virt_piece = piece.clone();
            virt_piece.rotate(rotation);

            !Self::does_piece_overlap(board, &virt_piece)
        } else {
            false
        }
    }

    fn incorporate_piece(&mut self) -> u64 {
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

            self.collapse_completed_rows()
        } else {
            0
        }
    }

    fn collapse_completed_rows(&mut self) -> u64 {
        let mut cleared_lines = 0;

        loop {
            let mut repeat = false;

            for row in (1..BOARD_HEIGHT).rev() {
                if self.is_row_full(row) {
                    repeat = true;
                    cleared_lines += 1;
                    for pull_row in (1..row).rev() {
                        self.lower_row(pull_row);
                    }
                }
            }

            if !repeat {
                break;
            }
        }

        LINE_CLEAR_POINTS[cleared_lines]
    }

    fn is_row_full(&self, row: usize) -> bool {
        for col in 0..BOARD_WIDTH {
            if self.get_cell_at(row, col) == Cell::Black {
                return false;
            }
        }

        true
    }

    fn lower_row(&mut self, row: usize) {
        for col in 0..BOARD_WIDTH {
            if row == 0 {
                self.board[row][col] = Cell::Black
            } else {
                self.board[row + 1][col] = self.board[row][col];
            }
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
        assert!(board.move_piece(Direction::Down).0);
        assert!(!board.move_piece(Direction::Down).0);
    }

    #[test]
    fn drop_s_piece_on_bottom() {
        let mut board = Board::new();

        let pos = Position { row: 15, col: 0 };
        let piece = Piece::new(crate::pieces::Tetromino::S, pos);

        assert!(board.add_piece(piece));
        assert!(board.move_piece(Direction::Down).0);
        assert!(board.move_piece(Direction::Down).0);
        assert!(!board.move_piece(Direction::Down).0);
    }

    #[test]
    fn stack_pieces_simple() {
        let mut board = Board::new();

        let pos = Position { row: 15, col: 3 };
        let piece_o = Piece::new(crate::pieces::Tetromino::O, pos);
        let pos = Position { row: 14, col: 4 };
        let piece_l = Piece::new(crate::pieces::Tetromino::L, pos);

        assert!(board.add_piece(piece_o));
        assert!(board.move_piece(Direction::Down).0);
        assert!(board.move_piece(Direction::Down).0);
        assert!(!board.move_piece(Direction::Down).0);

        assert!(board.add_piece(piece_l));
        assert!(board.move_piece(Direction::Down).0);
        assert!(!board.move_piece(Direction::Down).0);
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
        assert!(board.move_piece(Direction::Down).0);
        assert!(board.move_piece(Direction::Down).0);
        assert!(board.move_piece(Direction::Down).0);
        assert!(!board.move_piece(Direction::Down).0);
    }

    #[test]
    fn rotate_piece_ok() {
        let mut board = Board::new();

        let pos = Position { row: 5, col: 3 };
        let piece_i = Piece::new(crate::pieces::Tetromino::I, pos);

        assert!(board.add_piece(piece_i));

        assert!(board.rotate_piece(Rotation::Clockwise));
        assert!(board.rotate_piece(Rotation::Clockwise));

        assert!(board.rotate_piece(Rotation::CounterClockwise));
        assert!(board.rotate_piece(Rotation::CounterClockwise));
    }

    #[test]
    fn rotate_piece_ok_with_slide() {
        let mut board = Board::new();

        let pos = Position { row: 6, col: 0 };
        let piece_z = Piece::new(crate::pieces::Tetromino::Z, pos);
        assert!(board.add_piece(piece_z));
        board.incorporate_piece();

        let pos = Position { row: 6, col: 3 };
        let piece_l = Piece::new(crate::pieces::Tetromino::L, pos.clone());
        assert!(board.add_piece(piece_l));

        assert!(board.rotate_piece(Rotation::CounterClockwise));
        let piece = board.piece.unwrap();
        assert_eq!(pos.col + 1, piece.get_position().col);
    }

    #[test]
    fn rotate_piece_fails() {
        let mut board = Board::new();

        let pos = Position { row: 6, col: 6 };
        let piece_s = Piece::new(crate::pieces::Tetromino::S, pos);
        assert!(board.add_piece(piece_s));
        board.incorporate_piece();

        let pos = Position { row: 6, col: 0 };
        let piece_z = Piece::new(crate::pieces::Tetromino::Z, pos);
        assert!(board.add_piece(piece_z));
        board.incorporate_piece();

        let pos = Position { row: 6, col: 3 };
        let piece_l = Piece::new(crate::pieces::Tetromino::L, pos);
        assert!(board.add_piece(piece_l));

        assert!(board.rotate_piece(Rotation::Clockwise));
        assert!(board.rotate_piece(Rotation::Clockwise));
        assert!(!board.rotate_piece(Rotation::Clockwise));
    }

    #[test]
    fn lower_single_row() {
        let mut board = Board::new();

        let pos = Position { row: 17, col: 5 };
        let piece_s = Piece::new(crate::pieces::Tetromino::S, pos);
        assert!(board.add_piece(piece_s));
        board.incorporate_piece();

        let pos = Position { row: 17, col: 1 };
        let piece_z = Piece::new(crate::pieces::Tetromino::Z, pos);
        assert!(board.add_piece(piece_z));
        board.incorporate_piece();

        let pos = Position { row: 17, col: 0 };
        let piece_l = Piece::new(crate::pieces::Tetromino::L, pos);
        assert!(board.add_piece(piece_l));
        board.incorporate_piece();

        let pos = Position { row: 17, col: 6 };
        let piece_j = Piece::new(crate::pieces::Tetromino::J, pos);
        assert!(board.add_piece(piece_j));
        board.incorporate_piece();

        let pos = Position { row: 16, col: -1 };
        let piece_i = Piece::new(crate::pieces::Tetromino::I, pos);
        assert!(board.add_piece(piece_i));
        board.incorporate_piece();

        assert_eq!(Cell::Red, board.get_cell_at(18, 2));
        assert_eq!(Cell::Green, board.get_cell_at(18, 7));
        assert_eq!(Cell::Orange, board.get_cell_at(19, 2));
        assert_eq!(Cell::Blue, board.get_cell_at(19, 7));

        let pos = Position { row: 16, col: 8 };
        let piece_i = Piece::new(crate::pieces::Tetromino::I, pos);
        assert!(board.add_piece(piece_i));
        board.incorporate_piece();

        assert_eq!(Cell::Orange, board.get_cell_at(18, 1));
        assert_eq!(Cell::Black, board.get_cell_at(18, 2));
        assert_eq!(Cell::Black, board.get_cell_at(18, 7));
        assert_eq!(Cell::Red, board.get_cell_at(19, 2));
        assert_eq!(Cell::Green, board.get_cell_at(19, 7));
    }

    #[test]
    fn lower_multiple_rows() {
        let mut board = Board::new();

        let pos = Position { row: 17, col: 5 };
        let piece_s = Piece::new(crate::pieces::Tetromino::S, pos);
        assert!(board.add_piece(piece_s));
        board.incorporate_piece();

        let pos = Position { row: 17, col: 1 };
        let piece_z = Piece::new(crate::pieces::Tetromino::Z, pos);
        assert!(board.add_piece(piece_z));
        board.incorporate_piece();

        let pos = Position { row: 17, col: 0 };
        let piece_l = Piece::new(crate::pieces::Tetromino::L, pos);
        assert!(board.add_piece(piece_l));
        board.incorporate_piece();

        let pos = Position { row: 17, col: 6 };
        let piece_j = Piece::new(crate::pieces::Tetromino::J, pos);
        assert!(board.add_piece(piece_j));
        board.incorporate_piece();

        let pos = Position { row: 16, col: -1 };
        let piece_i = Piece::new(crate::pieces::Tetromino::I, pos);
        assert!(board.add_piece(piece_i));
        board.incorporate_piece();

        let pos = Position { row: 16, col: 3 };
        let piece_o = Piece::new(crate::pieces::Tetromino::O, pos);
        assert!(board.add_piece(piece_o));
        board.incorporate_piece();

        assert_eq!(Cell::Brown, board.get_cell_at(17, 0));
        assert_eq!(Cell::Orange, board.get_cell_at(17, 1));
        assert_eq!(Cell::Yellow, board.get_cell_at(17, 4));
        assert_eq!(Cell::Blue, board.get_cell_at(17, 8));
        assert_eq!(Cell::Red, board.get_cell_at(18, 2));
        assert_eq!(Cell::Green, board.get_cell_at(18, 7));
        assert_eq!(Cell::Orange, board.get_cell_at(19, 2));
        assert_eq!(Cell::Blue, board.get_cell_at(19, 7));

        let pos = Position { row: 16, col: 8 };
        let piece_i = Piece::new(crate::pieces::Tetromino::I, pos);
        assert!(board.add_piece(piece_i));
        board.incorporate_piece();

        assert_eq!(Cell::Brown, board.get_cell_at(19, 0));
        assert_eq!(Cell::Orange, board.get_cell_at(19, 1));
        assert_eq!(Cell::Yellow, board.get_cell_at(19, 4));
        assert_eq!(Cell::Black, board.get_cell_at(19, 7));
        assert_eq!(Cell::Blue, board.get_cell_at(19, 8));
        assert_eq!(Cell::Brown, board.get_cell_at(19, 9));
    }
}
