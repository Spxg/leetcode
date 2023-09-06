pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let number_str = x.to_string();
        !number_str
            .chars()
            .zip(number_str.chars().rev())
            .any(|(lhs, rhs)| lhs != rhs)
    }
}

impl super::Solution for Solution {
    fn is_palindrome(x: i32) -> bool {
        Self::is_palindrome(x)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
