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
    grid: [[bool; MAX_ROWS]; MAX_COLS],
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


        let grid = [[true; MAX_ROWS]; MAX_COLS];

        Ok(Self {
            grid,
            nrows,
            ncols,
        })
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
            _ => false
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
            (x, y) if x >= self.nrows || y >= self.ncols => Err(ChompError::InvalidMove {row, col}),
            (x, y) if !self.grid[x][y] => Err(ChompError::InvalidMove {row, col}),
            (x, y) if self.grid[x][y] => todo!("Change game board state to reflect move, call func maybe"),
            _ => Err(ChompError::InvalidMove {row, col}),
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
        None
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
