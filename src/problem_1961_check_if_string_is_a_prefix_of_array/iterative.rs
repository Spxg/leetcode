pub struct Solution;

impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let mut result = String::new();
        for word in words {
            result += &word;
            if result == s {
                return true;
            }
        }
        false
    }
}

impl super::Solution for Solution {
    fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        Self::is_prefix_string(s, words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
