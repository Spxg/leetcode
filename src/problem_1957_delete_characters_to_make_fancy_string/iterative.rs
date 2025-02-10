pub struct Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut bytes = vec![];
        let mut repeat = b'A';
        let mut count = 0;
        for ch in s.bytes() {
            if repeat == ch {
                count += 1;
                if count >= 2 {
                    count = 2;
                }
            } else {
                bytes.extend(std::iter::repeat(repeat).take(count));
                repeat = ch;
                count = 1;
            }
        }
        bytes.extend(std::iter::repeat(repeat).take(count));
        String::from_utf8(bytes).unwrap()
    }
}

impl super::Solution for Solution {
    fn make_fancy_string(s: String) -> String {
        Self::make_fancy_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
