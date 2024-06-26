pub mod bit;
pub mod dp;

pub trait Solution {
    fn gray_code(n: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

        for test_case in test_cases {
            let mut result = S::gray_code(test_case);

            assert_eq!(result[0], 0);
            assert_eq!(result.len(), 1 << test_case);

            for window in result.windows(2) {
                let diff = window[0] ^ window[1];

                assert!(diff > 0 && (diff & (diff - 1) == 0)); // Check diff is power of two.
            }

            result.sort_unstable();

            for window in result.windows(2) {
                assert_eq!(window[1], window[0] + 1);
            }
        }
    }
}
