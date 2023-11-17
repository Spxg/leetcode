pub struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        fn helper(board: &mut Vec<Vec<char>>, word: &[u8], start: (usize, usize)) -> bool {
            let mut exist = |x, y, ch, rest| {
                let mut result = false;
                if (0..board.len()).contains(&x) && (0..board[0].len()).contains(&y) && board[x][y] as u8 == ch {
                    board[x][y] = '.';
                    result |= helper(board, rest, (x, y));
                    board[x][y] = char::from(ch);
                }
                result
            };
            if let Some((first, rest)) = word.split_first() {
                exist(start.0.wrapping_sub(1), start.1, *first, rest)
                    || exist(start.0 + 1, start.1, *first, rest)
                    || exist(start.0, start.1.wrapping_sub(1), *first, rest)
                    || exist(start.0, start.1 + 1, *first, rest)
            } else {
                true
            }
        }

        let word = word.as_bytes();
        let (first, rest) = word.split_first().unwrap();
        let mut board = board;

        for x in 0..board.len() {
            for y in 0..board[0].len() {
                if board[x][y] as u8 == *first {
                    board[x][y] = '.';
                    if helper(&mut board, rest, (x, y)) {
                        return true;
                    }
                    board[x][y] = char::from(*first);
                }
            }
        }

        false
    }
}

impl super::Solution for Solution {
    fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        Self::exist(board, word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
