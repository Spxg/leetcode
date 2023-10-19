pub mod split;

pub trait Solution {
    fn generate_matrix(n: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, &[&[1] as &[_]] as &[&[_]]),
            (2, &[&[1, 2], &[4, 3]]),
            (3, &[&[1, 2, 3], &[8, 9, 4], &[7, 6, 5]]),
            (3, &[&[1, 2, 3], &[8, 9, 4], &[7, 6, 5]]),
            (4, &[&[1, 2, 3, 4], &[12, 13, 14, 5], &[11, 16, 15, 6], &[10, 9, 8, 7]]),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::generate_matrix(n), expected);
        }
    }
}
