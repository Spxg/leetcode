pub mod math;

pub trait Solution {
    fn set_zeroes(matrix: &mut Vec<Vec<i32>>);
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[1, 1, 1], [1, 0, 1], [1, 1, 1]] as &dyn Matrix<_>,
                &[[1, 0, 1], [0, 0, 0], [1, 0, 1]] as &dyn Matrix<_>,
            ),
            (
                &[[0, 1, 2, 0], [3, 4, 5, 2], [1, 3, 1, 5]],
                &[[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]],
            ),
            (
                &[
                    [0, 0, 0, 5],
                    [4, 3, 1, 4],
                    [0, 1, 1, 4],
                    [1, 2, 1, 3],
                    [0, 0, 1, 1],
                ],
                &[
                    [0, 0, 0, 0],
                    [0, 0, 0, 4],
                    [0, 0, 0, 0],
                    [0, 0, 0, 3],
                    [0, 0, 0, 0],
                ],
            ),
        ];

        for (matrix, expected) in test_cases {
            let mut matrix = matrix.to_vec();

            S::set_zeroes(&mut matrix);

            assert_eq!(matrix, expected);
        }
    }
}
