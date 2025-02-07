pub struct Solution;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut broken = [false; 26];
        for byte in broken_letters.bytes() {
            broken[(byte - b'a') as usize] = true;
        }
        text.split(' ')
            .filter(|s| !s.bytes().any(|x| broken[(x - b'a') as usize]))
            .count() as _
    }
}

impl super::Solution for Solution {
    fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        Self::can_be_typed_words(text, broken_letters)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
