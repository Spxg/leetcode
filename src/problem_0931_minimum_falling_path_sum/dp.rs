pub struct Solution;

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let mut matrix = matrix;

        for row in 1..n {
            for col in 0..n {
                matrix[row][col] += if col == 0 {
                    i32::MAX
                } else {
                    matrix[row - 1][col - 1]
                }
                .min(matrix[row - 1][col])
                .min(if col == n - 1 {
                    i32::MAX
                } else {
                    matrix[row - 1][col + 1]
                });
            }
        }

        matrix[n - 1].iter().min().copied().unwrap()
    }
}

impl super::Solution for Solution {
    fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        Self::min_falling_path_sum(matrix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
