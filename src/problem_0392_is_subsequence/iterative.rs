pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut t = t.bytes();
        for val in s.bytes() {
            if !t.any(|x| x == val) {
                return false;
            }
        }
        true
    }
}

impl super::Solution for Solution {
    fn is_subsequence(s: String, t: String) -> bool {
        Self::is_subsequence(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
