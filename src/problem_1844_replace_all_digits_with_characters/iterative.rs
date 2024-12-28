pub struct Solution;

impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut s = s.into_bytes();
        for idx in (1..s.len()).step_by(2) {
            s[idx] = s[idx - 1] + s[idx] - b'0';
        }
        String::from_utf8(s).unwrap()
    }
}

impl super::Solution for Solution {
    fn replace_digits(s: String) -> String {
        Self::replace_digits(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
