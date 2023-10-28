pub mod dp;

pub trait Solution {
    fn num_decodings(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("12", 2), ("226", 3), ("0", 0), ("06", 0), ("27", 1)];

        for (s, expected) in test_cases {
            assert_eq!(S::num_decodings(s.to_string()), expected);
        }
    }
}
