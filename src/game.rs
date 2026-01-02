use std::io::{Result as IoResult, Write};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn other(self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Player::X => 'X',
            Player::O => 'O',
        }
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    pub cells: [Option<Player>; 9],
}

impl Board {
    pub fn new() -> Self {
        Self { cells: [None; 9] }
    }

    fn idx(row: usize, col: usize) -> usize {
        row * 3 + col
    }

    pub fn place(&mut self, row: usize, col: usize, p: Player) -> bool {
        if row >= 3 || col >= 3 {
            return false;
        }
        let i = Board::idx(row, col);
        if self.cells[i].is_none() {
            self.cells[i] = Some(p);
            true
        } else {
            false
        }
    }

    pub fn available_moves(&self) -> Vec<usize> {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(i, c)| if c.is_none() { Some(i) } else { None })
            .collect()
    }

    pub fn is_full(&self) -> bool {
        self.cells.iter().all(|c| c.is_some())
    }

    pub fn winner(&self) -> Option<Player> {
        let lines = [
            (0, 1, 2),
            (3, 4, 5),
            (6, 7, 8),
            (0, 3, 6),
            (1, 4, 7),
            (2, 5, 8),
            (0, 4, 8),
            (2, 4, 6),
        ];

        for (a, b, c) in lines.iter() {
            if let (Some(p1), Some(p2), Some(p3)) = (
                self.cells[*a],
                self.cells[*b],
                self.cells[*c],
            ) {
                if p1 == p2 && p2 == p3 {
                    return Some(p1);
                }
            }
        }
        None
    }

    pub fn print_to<W: Write>(&self, mut w: W) -> IoResult<()> {
        writeln!(w)?;
        for r in 0..3 {
            for c in 0..3 {
                let ch = match self.cells[Board::idx(r, c)] {
                    Some(p) => p.to_char(),
                    None => '.',
                };
                if c < 2 {
                    write!(w, " {} |", ch)?;
                } else {
                    write!(w, " {}", ch)?;
                }
            }
            writeln!(w)?;
            if r < 2 {
                writeln!(w, "---+---+---")?;
            }
        }
        writeln!(w)?;
        Ok(())
    }
}

/// Minimax returns (score, best_move_index)
fn minimax(board: &Board, current: Player, ai: Player) -> (i32, Option<usize>) {
    if let Some(w) = board.winner() {
        if w == ai {
            return (1, None);
        } else {
            return (-1, None);
        }
    }

    if board.is_full() {
        return (0, None);
    }

    let mut best_score = if current == ai { i32::MIN } else { i32::MAX };
    let mut best_move = None;

    for mv in board.available_moves() {
        let mut next = board.clone();
        next.cells[mv] = Some(current);

        let (score, _) = minimax(&next, current.other(), ai);

        if current == ai {
            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }
        } else {
            if score < best_score {
                best_score = score;
                best_move = Some(mv);
            }
        }
    }

    (best_score, best_move)
}

pub fn find_best_move(board: &Board, ai: Player) -> Option<usize> {
    let (_score, mv) = minimax(board, ai, ai);
    mv
}

pub fn parse_coord(s: &str) -> Option<(usize, usize)> {
    let parts: Vec<&str> = s.trim().split(':').collect();
    if parts.len() != 2 {
        return None;
    }
    let r = parts[0].trim().parse::<usize>().ok()?;
    let c = parts[1].trim().parse::<usize>().ok()?;
    if r < 3 && c < 3 {
        Some((r, c))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn place_and_available() {
        let mut b = Board::new();
        assert!(b.place(0, 0, Player::X));
        assert!(!b.place(0, 0, Player::O)); // occupied
        assert!(!b.place(3, 0, Player::X)); // out of bounds
        let moves = b.available_moves();
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn winner_rows_cols_diags() {
        let mut b = Board::new();
        b.place(0, 0, Player::X);
        b.place(0, 1, Player::X);
        b.place(0, 2, Player::X);
        assert_eq!(b.winner(), Some(Player::X));

        let mut b2 = Board::new();
        b2.place(0, 0, Player::O);
        b2.place(1, 0, Player::O);
        b2.place(2, 0, Player::O);
        assert_eq!(b2.winner(), Some(Player::O));

        let mut b3 = Board::new();
        b3.place(0, 0, Player::X);
        b3.place(1, 1, Player::X);
        b3.place(2, 2, Player::X);
        assert_eq!(b3.winner(), Some(Player::X));
    }

    #[test]
    fn minimax_block_and_win() {
        // ai X can win
        let mut b = Board::new();
        b.place(0, 0, Player::X);
        b.place(0, 1, Player::X);
        b.place(1, 1, Player::O);
        let mv = find_best_move(&b, Player::X);
        assert_eq!(mv, Some(2)); // completes row 0

        // need to block opponent
        let mut b2 = Board::new();
        b2.place(0, 0, Player::X);
        b2.place(0, 1, Player::X);
        b2.place(1, 1, Player::O);
        let mv_block = find_best_move(&b2, Player::O);
        assert_eq!(mv_block, Some(2));
    }

    #[test]
    fn parse_coord_tests() {
        assert_eq!(parse_coord("1:2"), Some((1, 2)));
        assert_eq!(parse_coord(" 0:0 \n"), Some((0, 0)));
        assert_eq!(parse_coord("3:0"), None);
        assert_eq!(parse_coord("a:b"), None);
        assert_eq!(parse_coord("1"), None);
    }

    #[test]
    fn print_to_outputs() {
        let mut b = Board::new();
        b.place(0, 0, Player::X);
        let mut buf = Vec::new();
        b.print_to(&mut buf).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert!(s.contains("X"));
        assert!(s.contains("---+---+---"));
    }
}
