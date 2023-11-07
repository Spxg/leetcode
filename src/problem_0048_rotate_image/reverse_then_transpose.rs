pub struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        matrix.reverse();
        for idx1 in 1..matrix.len() {
            for idx2 in 0..idx1 {
                let t = matrix[idx1][idx2];
                matrix[idx1][idx2] = matrix[idx2][idx1];
                matrix[idx2][idx1] = t;
            }
        }
    }
}

impl super::Solution for Solution {
    fn rotate(matrix: &mut Vec<Vec<i32>>) {
        Self::rotate(matrix);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
