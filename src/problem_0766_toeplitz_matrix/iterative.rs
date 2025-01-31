pub struct Solution;

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();

        for col in 0..n {
            if !(0..m)
                .zip(col..n)
                .all(|(x, y)| matrix[x][y] == matrix[0][col])
            {
                return false;
            }
        }

        for row in 0..m {
            if !(row..m)
                .zip(0..n)
                .all(|(x, y)| matrix[x][y] == matrix[row][0])
            {
                return false;
            }
        }

        true
    }
}

impl super::Solution for Solution {
    fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        Self::is_toeplitz_matrix(matrix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
