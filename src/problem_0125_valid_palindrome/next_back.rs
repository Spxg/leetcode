pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut iter = s.chars().filter(char::is_ascii_alphanumeric);
        while let Some(a) = iter.next() {
            if let Some(b) = iter.next_back() {
                if !a.eq_ignore_ascii_case(&b) {
                    return false;
                }
            } else {
                break;
            }
        }
        true
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
