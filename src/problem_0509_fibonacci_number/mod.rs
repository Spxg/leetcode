pub mod dp;

pub trait Solution {
    fn fib(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 2),
            (4, 3),
            (5, 5),
            (6, 8),
            (7, 13),
            (8, 21),
            (9, 34),
            (10, 55),
            (46, 1_836_311_903),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::fib(n), expected);
        }
    }
}
