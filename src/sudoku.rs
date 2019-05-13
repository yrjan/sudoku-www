use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Variable(u8),
    Constant(u8),
    Empty,
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Variable(v) | Cell::Constant(v) => write!(f, "{}", v),
            Cell::Empty => write!(f, "_"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Board {
    pub squares: Box<[Cell]>,
    pub n: usize,
}

#[allow(dead_code)]
impl Board {
    pub fn new(n: usize) -> Board {
        Board {
            squares: vec![Cell::Empty; n * n].into_boxed_slice(),
            n: n,
        }
    }

    pub fn from(squares: &[Cell]) -> Board {
        let n = (squares.len() as f64).sqrt() as usize;
        assert_eq!(n * n, squares.len());
        Board {
            // TODO: there must be a nicer way to do this.
            squares: squares.to_vec().into_boxed_slice(),
            n: n,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Cell {
        self.squares[y * self.n + x]
    }

    pub fn set(&self, x: usize, y: usize, v: Cell) -> Board {
        let mut board = Board {
            squares: self.squares.to_vec().into_boxed_slice(),
            n: self.n,
        };
        board.squares[y * self.n + x] = v;
        board
    }

    fn check_row_constraint(&self, y: usize) -> bool {
        let mut set: HashSet<u8> = HashSet::new();
        for x in 0..self.n {
            let value = match self.get(x, y) {
                Cell::Variable(v) | Cell::Constant(v) => v,
                Cell::Empty => continue,
            };
            match set.get(&value) {
                Some(_) => return false,
                None => set.insert(value),
            };
        }
        true
    }

    fn check_col_constraint(&self, x: usize) -> bool {
        let mut set: HashSet<u8> = HashSet::new();
        for y in 0..self.n {
            let value = match self.get(x, y) {
                Cell::Variable(v) | Cell::Constant(v) => v,
                Cell::Empty => continue,
            };
            match set.get(&value) {
                Some(_) => return false,
                None => set.insert(value),
            };
        }
        true
    }

    fn check_box_constraint(&self, x: usize, y: usize) -> bool {
        let mut set: HashSet<u8> = HashSet::new();
        let sqrt_n = (self.n as f64).sqrt() as usize;
        for y_ in (y / sqrt_n * sqrt_n)..((y / sqrt_n + 1) * sqrt_n) {
            for x_ in (x / sqrt_n * sqrt_n)..((x / sqrt_n + 1) * sqrt_n) {
                let value = match self.get(x_, y_) {
                    Cell::Variable(v) | Cell::Constant(v) => v,
                    Cell::Empty => continue,
                };
                match set.get(&value) {
                    Some(_) => return false,
                    None => set.insert(value),
                };
            }
        }
        true
    }

    fn within_constraints(&self, x: usize, y: usize) -> bool {
        self.check_row_constraint(y)
            && self.check_col_constraint(x)
            && self.check_box_constraint(x, y)
    }

    fn solver(&self, x: usize, y: usize) -> Option<Board> {
        let x_next = if x < self.n - 1 { x + 1 } else { 0 };
        let y_next = if x < self.n - 1 { y } else { y + 1 };

        match self.get(x, y) {
            Cell::Constant(_) => {
                if !self.within_constraints(x, y) {
                    return None;
                }
                self.solver(x_next, y_next)
            }
            _ => {
                for v in 1..=self.n {
                    let new_board = self.set(x, y, Cell::Variable(v as u8));

                    if !new_board.within_constraints(x, y) {
                        continue;
                    }

                    if x == self.n - 1 && y == self.n - 1 {
                        // We have finished.
                        return Some(Board {
                            squares: new_board.squares.to_vec().into_boxed_slice(),
                            n: self.n,
                        });
                    }

                    match new_board.solver(x_next, y_next) {
                        Some(board) => return Some(board),
                        _ => (),
                    }
                }
                None
            }
        }
    }

    pub fn solve(&self) -> Option<Board> {
        self.solver(0, 0)
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.n {
            for x in 0..self.n {
                match write!(f, " {:?} ", self.get(x, y)) {
                    Err(e) => return Err(e),
                    _ => (),
                }
            }
            match write!(f, "\n") {
                Err(e) => return Err(e),
                _ => (),
            }
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_valid() {
        let squares = [
            Cell::Constant(2),
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Constant(4),
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Constant(2),
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
        ];
        let correct_squares = [
            Cell::Constant(2),
            Cell::Variable(1),
            Cell::Variable(3),
            Cell::Variable(4),
            Cell::Constant(4),
            Cell::Variable(3),
            Cell::Variable(1),
            Cell::Variable(2),
            Cell::Variable(1),
            Cell::Variable(4),
            Cell::Constant(2),
            Cell::Variable(3),
            Cell::Variable(3),
            Cell::Variable(2),
            Cell::Variable(4),
            Cell::Variable(1),
        ];
        let board = Board::from(&squares);
        let correct_board = Board::from(&correct_squares);
        let solution = board.solve();
        assert_eq!(solution.unwrap(), correct_board);
    }

    #[test]
    fn test_solve_invalid_return_none() {
        let squares = [
            Cell::Constant(2),
            Cell::Empty,
            Cell::Empty,
            Cell::Constant(1),
            Cell::Constant(4),
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Constant(2),
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Constant(1),
        ];
        let board = Board::from(&squares);
        let solution = board.solve();
        assert_eq!(solution, None);
    }
}
