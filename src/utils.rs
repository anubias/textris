pub struct Position {
    pub row: isize,
    pub col: isize,
}

impl Position {
    pub fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
}

/// Translates the piece coordinates into board coordinates, by adding to the
/// piece coordinates the position (top-left) of the piece relative to the board.
///
/// Note: Please take care, that the returned coordinates may not necessarily
/// be valid coordinates for the board. You must validate the results before
/// indexing with these coordinates!
pub fn to_board_coord(pos: &Position, piece_row: usize, piece_col: usize) -> (isize, isize) {
    let (i_pr, i_pc) = to_isize(piece_row, piece_col);
    (i_pr + pos.row, i_pc + pos.col)
}

/// Translates the board coordinates into piece coordinates, by subtracting from the
/// board coordinates the position (top-left) of the piece relative to the board.
///
/// Note: Please take care, that the returned coordinates may not necessarily
/// be valid coordinates for the piece.You must validate the results before
/// indexing with these coordinates!
pub fn to_piece_coord(pos: &Position, board_row: usize, board_col: usize) -> (isize, isize) {
    let (i_br, i_bc) = to_isize(board_row, board_col);

    (i_br - pos.row, i_bc - pos.col)
}

pub fn to_isize(col: usize, row: usize) -> (isize, isize) {
    (col as isize, row as isize)
}

pub fn to_usize(col: isize, row: isize) -> (usize, usize) {
    (col as usize, row as usize)
}

pub fn is_within_bounds(val: isize, min: isize, max: isize) -> bool {
    val >= min && val < max
}
