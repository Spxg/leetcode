pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let len = s.len();
        let mut result = 0;

        for idx in 0..len {
            for (left, right) in (0..=idx).rev().zip(idx..len) {
                if s[left] == s[right] {
                    result += 1;
                } else {
                    break;
                }
            }

            for (left, right) in (0..=idx).rev().zip(idx + 1..len) {
                if s[left] == s[right] {
                    result += 1;
                } else {
                    break;
                }
            }
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
