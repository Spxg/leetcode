pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        fn helper(
            x: usize,
            y: usize,
            target: i32,
            m: usize,
            n: usize,
            matrix: &mut Vec<Vec<i32>>,
        ) -> bool {
            let value = matrix[x][y];
            matrix[x][y] = i32::MAX;
            match value.cmp(&target) {
                std::cmp::Ordering::Less => {
                    y + 1 < n
                        && matrix[x][y + 1] != i32::MAX
                        && helper(x, y + 1, target, m, n, matrix)
                        || x + 1 < m
                            && matrix[x + 1][y] != i32::MAX
                            && helper(x + 1, y, target, m, n, matrix)
                }
                std::cmp::Ordering::Equal => true,
                std::cmp::Ordering::Greater => {
                    x != 0 && matrix[x - 1][y] != i32::MAX && helper(x - 1, y, target, m, n, matrix)
                        || y != 0
                            && matrix[x][y - 1] != i32::MAX
                            && helper(x, y - 1, target, m, n, matrix)
                }
            }
        }
        let mut matrix = matrix;
        let m = matrix.len();
        let n = matrix[0].len();
        helper(0, 0, target, m, n, &mut matrix)
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
