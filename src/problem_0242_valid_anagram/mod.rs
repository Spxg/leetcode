pub mod iterative;

pub trait Solution {
    fn is_anagram(s: String, t: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("anagram", "nagaram"), true),
            (("rat", "car"), false),
            (("a", "ab"), false),
        ];

        for ((s, t), expected) in test_cases {
            assert_eq!(S::is_anagram(s.to_string(), t.to_string()), expected);
        }
    }
}
