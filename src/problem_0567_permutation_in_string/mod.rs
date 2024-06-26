pub mod map_and_deque;

pub trait Solution {
    fn check_inclusion(s1: String, s2: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("ab", "eidbaooo"), true),
            (("ab", "eidboaoo"), false),
            (("ab", "a"), false),
            (("ab", "ba"), true),
            (("abc", "bbbca"), true),
        ];

        for ((s1, s2), expected) in test_cases {
            assert_eq!(S::check_inclusion(s1.to_string(), s2.to_string()), expected);
        }
    }
}
