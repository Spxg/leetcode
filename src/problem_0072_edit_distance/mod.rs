pub mod dp;

pub trait Solution {
    fn min_distance(word1: String, word2: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("horse", "ros"), 3),
            (("intention", "execution"), 5),
            (("", "a"), 1),
        ];

        for ((word1, word2), expected) in test_cases {
            assert_eq!(
                S::min_distance(word1.to_string(), word2.to_string()),
                expected
            );
        }
    }
}
