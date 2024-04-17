pub struct NumMatrix {
    inner: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let col = matrix[0].len() + 1;
        let row = matrix.len() + 1;
        let mut inner = vec![vec![0; col]; row];
        for idx1 in 1..row {
            for idx2 in 1..col {
                inner[idx1][idx2] +=
                    matrix[idx1 - 1][idx2 - 1] + inner[idx1 - 1][idx2] + inner[idx1][idx2 - 1]
                        - inner[idx1 - 1][idx2 - 1];
            }
        }
        Self { inner }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = row1 as usize;
        let col1 = col1 as usize;
        let row2 = row2 as usize;
        let col2 = col2 as usize;

        let matrix = &self.inner;
        matrix[row2 + 1][col2 + 1] - matrix[row1][col2 + 1] - matrix[row2 + 1][col1]
            + matrix[row1][col1]
    }
}

impl super::NumMatrix for NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        Self::new(matrix)
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        self.sum_region(row1, col1, row2, col2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::NumMatrix>();
    }
}
