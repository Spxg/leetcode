pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        fn helper(s: &[u8]) -> bool {
            let mut left = 0;
            let mut right = s.len() - 1;
            while left < right {
                if s[left] != s[right] {
                    return false;
                }
                left += 1;
                right -= 1;
            }
            true
        }

        let mut result = 0;
        let bytes = s.as_bytes();
        for size in 1..=s.len() {
            result += bytes
                .windows(size)
                .fold(0, |acc, x| acc + i32::from(helper(x)));
        }
        result
    }
}

impl super::Solution for Solution {
    fn count_substrings(s: String) -> i32 {
        Self::count_substrings(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
