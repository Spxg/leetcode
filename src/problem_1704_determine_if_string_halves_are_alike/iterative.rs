pub struct Solution;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let mid = (s.len() - 1) / 2;

        let mut total = 0;
        let mut left = 0;
        for (idx, ch) in s.chars().enumerate() {
            let is_vowel = i32::from(matches!(
                ch,
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U'
            ));
            if idx <= mid {
                left += is_vowel;
            }
            total += is_vowel;
        }
        left == total - left
    }
}

impl super::Solution for Solution {
    fn halves_are_alike(s: String) -> bool {
        Self::halves_are_alike(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
