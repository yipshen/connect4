use std::cmp;

#[derive(Clone, Copy)]
pub enum RulesVariation {
    Classic,
}

#[derive(Clone, Copy)]
pub struct Options {
    pub rules: RulesVariation,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Token {
    Invalid,
    Red,
    Yellow
}

impl Token {
    pub fn iter(&mut self) -> TokenIter {
        TokenIter {
            next: Some(*self),
        }
    }

    pub fn is_valid(&self) -> bool {
        match *self {
            Token::Invalid => false,
            _ => true,
        }
    }
}

pub struct TokenIter {
    next: Option<Token>,
}

impl Iterator for TokenIter {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.next {
            self.next = match next {
                Token::Red => Some(Token::Yellow),
                Token::Yellow => Some(Token::Red),
                _ => None,
            };
        } else {
            self.next = None;
        }
        self.next
    }
}

impl From<&char> for Token {
    fn from(c: &char) -> Self {
        match c {
            'R' => Token::Red,
            'Y' => Token::Yellow,
            _ => Token::Invalid,
        }
    }
}

pub struct Board {
    rules: RulesVariation,
    board: Vec<Vec<Token>>,
    history: Vec<usize>,
    pub cols: usize,
    pub rows: usize,
    pub current_token: Token,
}

#[derive(Debug)]
pub enum BoardError {
    BadFormat,
    InvalidBoard,
}

pub enum GamePlayError {
    InvalidColumn,
    ColumnFull,
}

impl Board {
    pub fn new(rules: RulesVariation) -> Self {
        let cols: usize;
        let rows: usize;

        match rules {
            RulesVariation::Classic => {
                cols = 7;
                rows = 6;
            }
        }

        Self {
            rules: rules,
            board: vec![vec![Token::Invalid; rows]; cols],
            history: Vec::new(),
            cols: cols,
            rows: rows,
            current_token: Token::Red,
        }
    }

    // Load a game from a string (save).
    pub fn from_string(board: &str) -> Result<Board, BoardError> {
        let chars = board.chars().collect::<Vec<char>>();
        if chars.len() < 3 {
            return Err(BoardError::BadFormat);
        }

        let mut iter = chars.iter();

        let mut board = Board::new(RulesVariation::Classic);

        while let Some(char) = iter.next() {
            let col = char.to_digit(10).ok_or(BoardError::InvalidBoard)? as usize;
            if col >= board.cols {
                return Err(BoardError::InvalidBoard);
            }
            board.drop(col).map_err(|_| BoardError::InvalidBoard)?;
        }

        Ok(board)
    }

    // Drop the current token into the board at the column col.
    pub fn drop(&mut self, col: usize) -> Result<(), GamePlayError> {
        if col > self.board.len() {
            return Err(GamePlayError::InvalidColumn);
        }

        for row in 0..=self.rows-1 {
            if !self.board[col][row].is_valid() {
                self.board[col][row] = self.current_token;
                self.history.push(col);

                return Ok(());
            }
        }
        Err(GamePlayError::ColumnFull)
    }

    // Prepare the token for the next player.
    pub fn switch_token(&mut self) {
        self.current_token = self.current_token.iter().next().unwrap();
    }

    // Check whether the last play generates a winning board.
    pub fn check_win(&self) -> bool {
        if self.history.is_empty() {
            return false;
        }

        let col = *self.history.last().unwrap();
        let row = self.find_row_for_col(col).unwrap();

        // Check -
        let mut counter = 0;
        for c in 0..self.cols {
            if self.board[c][row] == self.board[col][row] {
                counter += 1;
                if counter == 4 {
                    return true;
                }
            } else {
                counter = 0;
            }
        }

        // Check |
        let mut counter = 0;
        for r in 0..self.rows {
            if self.board[col][r] == self.board[col][row] {
                counter += 1;
                if counter == 4 {
                    return true;
                }
            } else {
                counter = 0;
            }
        }

        // Check \
        let mut counter = 0;
        let shift = cmp::min(col, self.rows - row - 1);
        let top_left = (col - shift, row + shift);
        let shift = cmp::min(self.cols - col - 1, row);
        let bottom_right = (col + shift, row - shift);
        for (c, r) in (top_left.0..bottom_right.0 + 1).zip((bottom_right.1..top_left.1 + 1).rev()) {
            if self.board[c][r] == self.board[col][row] {
                counter += 1;
                if counter == 4 {
                    return true;
                }
            } else {
                counter = 0;
            }
        }

        // Check /
        let mut counter = 0;
        let shift = cmp::min(self.cols - col - 1, self.rows - row - 1);
        let top_right = (col + shift, row + shift);
        let shift = cmp::min(col, row);
        let bottom_left = (col - shift, row - shift);
        for (c, r) in (bottom_left.0..top_right.0 + 1).zip(bottom_left.1..top_right.1 + 1) {
            if self.board[c][r] == self.board[col][row] {
                counter += 1;
                if counter == 4 {
                    return true;
                }
            } else {
                counter = 0;
            }
        }

        false
    }

    // Find the corresponding row of the last play for the given col.
    pub fn find_row_for_col(&self, col: usize) -> Option<usize> {
        for row in (0..self.rows).rev() {
            if self.board[col][row].is_valid() {
                return Some(row);
            }
        }
        None
    }
}
