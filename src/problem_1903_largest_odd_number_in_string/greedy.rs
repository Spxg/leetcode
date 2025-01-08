pub struct Solution;

impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let mut bytes = num.into_bytes();
        loop {
            let Some(digit) = bytes.last().copied() else {
                return String::new();
            };
            if (digit - b'0') % 2 == 0 {
                bytes.pop();
            } else {
                return String::from_utf8(bytes).unwrap();
            }
        }
    }
}

impl super::Solution for Solution {
    fn largest_odd_number(num: String) -> String {
        Self::largest_odd_number(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
