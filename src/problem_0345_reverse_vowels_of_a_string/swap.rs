pub struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut s = s.into_bytes();
        let chars = [b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U'];

        let mut left = 0;
        let mut right = s.len() - 1;

        while left < right {
            if !chars.contains(&s[left]) {
                left += 1;
            } else if !chars.contains(&s[right]) {
                right -= 1;
            } else {
                s.swap(left, right);
                left += 1;
                right -= 1;
            }
        }

        String::from_utf8(s).unwrap()
    }
}

impl super::Solution for Solution {
    fn reverse_vowels(s: String) -> String {
        Self::reverse_vowels(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
