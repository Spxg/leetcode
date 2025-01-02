pub struct Solution;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut pangram = [false; 26];
        for b in sentence.bytes() {
            pangram[(b - b'a') as usize] = true;
        }
        pangram.into_iter().all(|x| x)
    }
}

impl super::Solution for Solution {
    fn check_if_pangram(sentence: String) -> bool {
        Self::check_if_pangram(sentence)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
