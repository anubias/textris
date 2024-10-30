const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

pub struct Board {
    board: [[bool; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [[false; BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..BOARD_HEIGHT {
            let mut line = String::new();
            for j in 0..BOARD_WIDTH {
                if self.board[i][j] {
                    todo!();
                } else {
                    line = format!("{line}⬛");
                }
            }
            let _ = writeln!(f, "🧱{}🧱", line);
        }

        let mut bottom = String::new();
        for _ in 0..BOARD_WIDTH + 2 {
            bottom = format!("{bottom}🧱");
        }

        writeln!(f, "{bottom}")
    }
}
