pub struct Solution;

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut bytes = s.into_bytes();
        let mut left = 0;
        let mut right = bytes.len() - 1;
        while left < right {
            if !bytes[left].is_ascii_alphabetic() {
                left += 1;
                continue;
            }
            if !bytes[right].is_ascii_alphabetic() {
                right -= 1;
                continue;
            }
            bytes.swap(left, right);
            left += 1;
            right -= 1;
        }
        String::from_utf8(bytes).unwrap()
    }
}

impl super::Solution for Solution {
    fn reverse_only_letters(s: String) -> String {
        Self::reverse_only_letters(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
