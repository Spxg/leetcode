pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let mut result = String::new();

        for start in 0..bytes.len() {
            for end in start + 1..=bytes.len() {
                let slice = &bytes[start..end];

                if slice
                    .iter()
                    .zip(slice.iter().rev())
                    .all(|(start_ch, end_ch)| start_ch == end_ch)
                    && slice.len() > result.len()
                {
                    result = String::from_utf8(slice.to_vec()).unwrap();
                }
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn longest_palindrome(s: String) -> String {
        Self::longest_palindrome(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
