//! Chomp AI
//! Gustavo Carrillo Castro and Bart Massey 2023

/// Maximum number of rows the AI can handle.
const MAX_ROWS: usize = 4;
/// Maximum number of columns the AI can handle.
const MAX_COLS: usize = 5;

/// An invalid board size or move.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChompError {
    /// Board dimensions are zero or exceed the supported limits.
    InvalidSize {
        /// The requested number of rows.
        nrows: usize,
        /// The requested number of columns.
        ncols: usize,
    },
    /// A move is outside the board or selects an eaten square.
    InvalidMove {
        /// The requested row, starting from zero.
        row: usize,
        /// The requested column, starting from zero.
        col: usize,
    },
}

impl std::fmt::Display for ChompError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSize { nrows, ncols } => {
                write!(f, "invalid board size: {nrows} rows, {ncols} columns")
            }
            Self::InvalidMove { row, col } => {
                write!(
                    f,
                    "invalid move at ({row}, {col}): expected an uneaten square"
                )
            }
        }
    }
}

impl std::error::Error for ChompError {}

/// A Chomp board.
///
/// Replace this unit struct with a struct containing the fields
/// needed for your chosen board representation.
/// Clones must have the same dimensions and squares, with independent
/// state: making a move on a clone must not change the original.
#[derive(Debug, Clone)]
pub struct Chomp {
    grid: [[bool; MAX_COLS]; MAX_ROWS],
    nrows: usize,
    ncols: usize,
}

impl Chomp {
    /// Make a new Chomp board with the given size for this game.
    /// All squares within the requested dimensions start uneaten.
    ///
    /// # Errors
    /// Returns [`ChompError::InvalidSize`] unless `nrows` is in
    /// `1..=4` and `ncols` is in `1..=5`. Invalid sizes must not panic.
    pub fn new(nrows: usize, ncols: usize) -> Result<Self, ChompError> {
        if nrows == 0 || ncols == 0 || nrows > MAX_ROWS || ncols > MAX_COLS {
            return Err(ChompError::InvalidSize { nrows, ncols });
        };

        let mut grid = [[false; MAX_COLS]; MAX_ROWS];
        for row in grid.iter_mut().take(nrows) {
            for cell in row.iter_mut().take(ncols) {
                *cell = true;
            }
        }

        Ok(Self { grid, nrows, ncols })
    }

    /// Return the logical board dimensions as `(nrows, ncols)`.
    /// These are the original dimensions, even after squares are eaten.
    /// This method must not change the board.
    pub fn dimensions(&self) -> (usize, usize) {
        (self.nrows, self.ncols)
    }

    /// Return whether the square at `(row, col)` is uneaten.
    /// Coordinates start from zero, with `(0, 0)` at the upper left.
    /// Return `false` for eaten squares and for coordinates outside the
    /// logical dimensions, including arbitrarily large coordinates.
    /// This method must not change the board or panic.
    pub fn is_present(&self, row: usize, col: usize) -> bool {
        match (row, col) {
            (x, y) if x >= self.nrows || y >= self.ncols => false,
            (x, y) if !self.grid[x][y] => false,
            (x, y) if self.grid[x][y] => true,
            _ => false,
        }
    }

    /// Make a move on the current board, "eating" all cells
    /// below `row` and to the right of `col` inclusive.
    /// That is, remove each square whose row is at least `row` and whose
    /// column is at least `col`. Leave all other squares unchanged.
    /// The logical dimensions do not change.
    /// Returns `Ok(())` after a valid move. Eating the poison square
    /// at `(0, 0)` is a valid move that ends the game.
    ///
    /// # Errors
    /// Returns [`ChompError::InvalidMove`] if the coordinates are
    /// outside the logical board or the selected square is already
    /// eaten. This includes every move after the poison is eaten.
    /// Invalid moves must leave the board unchanged and must not panic.
    pub fn make_move(&mut self, row: usize, col: usize) -> Result<(), ChompError> {
        match (row, col) {
            (x, y) if x >= self.nrows || y >= self.ncols => {
                Err(ChompError::InvalidMove { row, col })
            }
            (x, y) if !self.grid[x][y] => Err(ChompError::InvalidMove { row, col }),
            (x, y) if self.grid[x][y] => {
                self.apply_move_changes(x, y);
                Ok(())
            }
            _ => Err(ChompError::InvalidMove { row, col }),
        }
    }

    /// Helper method that directly modifies the board grid to apply a move.
    ///
    /// Sets all squares starting from `row` downwards and `col` rightwards
    /// (inclusive) to `false` (eaten). This assumes that bounds checking
    /// and validation have already been handled by `make_move`.
    pub fn apply_move_changes(&mut self, row: usize, col: usize) {
        for row in self.grid[row..].iter_mut() {
            row[col..].fill(false);
        }
    }

    /// Returns `Some` winning move for this position as `(row, col)`.
    /// Returns `None` if there is no winning move in this position.
    /// This includes losing positions, a board containing only the
    /// poison square, and any board where the poison is already eaten.
    /// Any winning move is acceptable when several exist. A winning
    /// move selects an uneaten square other than `(0, 0)` and leaves
    /// the opponent without a winning move.
    /// This method must not change the board.
    ///
    /// # Strategy
    ///
    /// ```text
    /// winning-move(posn):                               // first function call, current game board state
    ///     for each remaining row r                      // for each row
    ///         for each remaining column c in r          // for each col
    ///             if r = 0 and c = 0                    // we start at the poison but skip
    ///                 continue
    ///             p ← copy of posn                      // make a copy of the board
    ///             chomp r, c from p                     // attempt a move on current coords
    ///             m ← winning-move(p)                   // M is the result, either a move was made or none
    ///             if no winning move is returned        // If none, return current move
    ///                 return the move r, c
    ///    return no winning move
    /// ```
    pub fn winning_move(&self) -> Option<(usize, usize)> {
        for (row_i, row) in self.grid.iter().enumerate() {
            if row.first() == Some(&true) {
                for (col_i, col) in row.iter().enumerate() {
                    if *col && !(row_i == 0 && col_i == 0) {
                        let mut grid_copy = self.clone();
                        let _ = grid_copy.make_move(row_i, col_i);

                        if grid_copy.winning_move().is_none() {
                            return Some((row_i, col_i));
                        }
                    }
                }
            }
        }
        None
    }

    /// Prints the current state of the board to standard output.
    ///
    /// Uneaten squares are represented by a `#`, and eaten squares
    /// are represented by a `.`.
    pub fn print_board(&self) {
        for row in 0..self.nrows {
            for col in 0..self.ncols {
                print!("{} ", if self.is_present(row, col) { "#" } else { "." });
            }
            println!();
        }
    }

    /// Performs and returns a fallback move when no winning move is found.
    ///
    /// This strategy searches from the bottom-right corner of the board upwards
    /// and leftwards to find the first available (uneaten) square, "eats" it,
    /// and returns its coordinates as `Some((row, col))`.
    /// Returns `None` if no valid squares remain.
    pub fn fallback_move(&mut self) -> Option<(usize, usize)> {
        // Find lowest row, rightmost square, and eat
        for (row_i, row) in self.grid.iter().enumerate().rev() {
            for (col_i, col) in row.iter().enumerate().rev() {
                if *col {
                    let _ = self.make_move(row_i, col_i);
                    return Some((row_i, col_i));
                }
            }
        }
        None
    }

    /// Returns false if the poison (top left element) is still uneatean
    /// and true otherwise. Used to quickly check if the game should
    /// continue or if someone lost.
    pub fn poison_eaten(&self) -> bool {
        match self.grid[0][0] {
            true => false,
            false => true,
        }
    }
}

#[test]
fn test_winning_move() -> Result<(), ChompError> {
    let mut c = Chomp::new(2, 2)?;
    assert!(c.winning_move().is_some());
    c.make_move(1, 1)?;
    assert!(c.winning_move().is_none());
    Ok(())
}

#[test]
fn test_invalid_sizes() {
    for (nrows, ncols) in [(0, 1), (1, 0), (0, 0), (5, 1), (1, 6), (usize::MAX, 1)] {
        assert!(matches!(
            Chomp::new(nrows, ncols),
            Err(ChompError::InvalidSize { .. })
        ));
    }
}

#[test]
fn test_new_valid_board() {
    let board = Chomp::new(3, 4).expect("Failed to create valid board");
    assert_eq!(board.dimensions(), (3, 4));

    // Check that all valid squares are initially present
    for row in 0..3 {
        for col in 0..4 {
            assert!(board.is_present(row, col));
        }
    }
}

#[test]
fn test_new_invalid_size() {
    // Zero dimensions
    assert!(matches!(
        Chomp::new(0, 3),
        Err(ChompError::InvalidSize { nrows: 0, ncols: 3 })
    ));
    assert!(matches!(
        Chomp::new(3, 0),
        Err(ChompError::InvalidSize { nrows: 3, ncols: 0 })
    ));

    // Exceeding MAX_ROWS / MAX_COLS
    assert!(matches!(
        Chomp::new(MAX_ROWS + 1, 3),
        Err(ChompError::InvalidSize { nrows: 5, ncols: 3 })
    ));
    assert!(matches!(
        Chomp::new(2, MAX_COLS + 1),
        Err(ChompError::InvalidSize { nrows: 2, ncols: 6 })
    ));
}

#[test]
fn test_is_present_out_of_bounds() {
    let board = Chomp::new(2, 2).unwrap();
    assert!(!board.is_present(2, 2));
    assert!(!board.is_present(10, 10));
}

#[test]
fn test_make_move_valid() {
    let mut board = Chomp::new(3, 3).unwrap();

    // Eat from (1, 1)
    assert!(board.make_move(1, 1).is_ok());

    // Top left should still be present
    assert!(board.is_present(0, 0));
    assert!(board.is_present(0, 1));
    assert!(board.is_present(1, 0));

    // (1,1) and everything below/right should be eaten
    assert!(!board.is_present(1, 1));
    assert!(!board.is_present(1, 2));
    assert!(!board.is_present(2, 1));
    assert!(!board.is_present(2, 2));
}

#[test]
fn test_make_move_invalid() {
    let mut board = Chomp::new(3, 3).unwrap();

    // Move outside logical dimensions
    assert!(matches!(
        board.make_move(3, 3),
        Err(ChompError::InvalidMove { row: 3, col: 3 })
    ));

    // Make a valid move
    board.make_move(1, 1).unwrap();

    // Try to move on an already eaten square
    assert!(matches!(
        board.make_move(2, 2),
        Err(ChompError::InvalidMove { row: 2, col: 2 })
    ));
}

#[test]
fn test_poison_eaten() {
    let mut board = Chomp::new(2, 2).unwrap();
    assert!(!board.poison_eaten());

    board.make_move(1, 1).unwrap();
    assert!(!board.poison_eaten()); // Poison still there

    board.make_move(0, 0).unwrap(); // Eat the poison
    assert!(board.poison_eaten());
}

#[test]
fn test_fallback_move() {
    let mut board = Chomp::new(2, 2).unwrap();

    // Fallback should pick the bottom-right most available square
    let mov = board.fallback_move();
    assert_eq!(mov, Some((1, 1)));

    let mov2 = board.fallback_move();
    // Depending on iteration order (rev over rows, then rev over cols),
    // it should pick (1, 0) or (0, 1). Given the nested loops, it checks
    // row 1 entirely before row 0, so it will pick (1, 0).
    assert_eq!(mov2, Some((1, 0)));
}

#[test]
fn test_winning_move_1x2() {
    // On a 1x2 board, eating (0, 1) is a forced win, leaving the opponent
    // with only the poison square (0, 0).
    let board = Chomp::new(1, 2).unwrap();

    let win = board.winning_move();
    assert_eq!(win, Some((0, 1)));
}

#[test]
fn test_winning_move_no_win() {
    // If it's a 1x1 board, there's only the poison square left.
    // A winning move must be an uneaten square *other* than (0,0).
    let board = Chomp::new(1, 1).unwrap();

    let win = board.winning_move();
    assert_eq!(win, None);
}

#[test]
fn test_winning_move_square() {
    // On a square board > 1x1, eating (1,1) is always a mathematically
    // proven winning opening move in Chomp.
    let board = Chomp::new(2, 2).unwrap();

    let win = board.winning_move();
    assert_eq!(win, Some((1, 1)));
}
