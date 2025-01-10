pub struct Solution;

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        fn helper(s: &str) -> bool {
            let mut iter = s.chars();
            while let (Some(left), Some(right)) = (iter.next(), iter.next_back()) {
                if left != right {
                    return false;
                }
            }
            true
        }
        words.into_iter().find(|x| helper(x)).unwrap_or_default()
    }
}

impl super::Solution for Solution {
    fn first_palindrome(words: Vec<String>) -> String {
        Self::first_palindrome(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
