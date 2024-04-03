pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut chars = [0; 26];
        for (x, y) in s.bytes().zip(t.bytes()) {
            chars[(x - b'a') as usize] += 1;
            chars[(y - b'a') as usize] -= 1;
        }
        chars.into_iter().all(|x| x == 0)
    }
}

impl super::Solution for Solution {
    fn is_anagram(s: String, t: String) -> bool {
        Self::is_anagram(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
