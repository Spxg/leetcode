pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        fn helper<T>(s: T) -> impl Iterator<Item = char>
        where
            T: Iterator<Item = char>,
        {
            s.into_iter().filter(char::is_ascii_alphanumeric)
        }

        helper(s.chars())
            .zip(helper(s.chars().rev()))
            .all(|(a, b)| a.eq_ignore_ascii_case(&b))
    }
}

impl super::Solution for Solution {
    fn is_palindrome(s: String) -> bool {
        Self::is_palindrome(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
