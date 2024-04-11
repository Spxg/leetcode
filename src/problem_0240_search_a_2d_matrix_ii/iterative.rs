pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let (mut x, mut y) = (0, n - 1);
        loop {
            match matrix[x][y].cmp(&target) {
                std::cmp::Ordering::Less if x + 1 < m => x += 1,
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Greater if y > 0 => y -= 1,
                _ => return false,
            }
        }
    }
}

impl super::Solution for Solution {
    fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        Self::search_matrix(matrix, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
