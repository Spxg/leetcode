pub struct Solution;

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut bytes = word.into_bytes();
        if let Some(pos) = bytes.iter().position(|&x| x == ch as u8) {
            bytes[0..=pos].reverse();
        }
        String::from_utf8(bytes).unwrap()
    }
}

impl super::Solution for Solution {
    fn reverse_prefix(word: String, ch: char) -> String {
        Self::reverse_prefix(word, ch)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
