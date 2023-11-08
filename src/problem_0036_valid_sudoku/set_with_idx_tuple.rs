pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = HashSet::new();
        let mut cols = HashSet::new();
        let mut squares = HashSet::new();

        for (row, board) in board.iter().enumerate() {
            for (col, ele) in board.iter().enumerate() {
                if ele.is_numeric() {
                    if !rows.insert((row, ele)) {
                        return false;
                    }
                    if !cols.insert((col, ele)) {
                        return false;
                    }
                    if !squares.insert((row / 3, col / 3, ele)) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl super::Solution for Solution {
    fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        Self::is_valid_sudoku(board)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
