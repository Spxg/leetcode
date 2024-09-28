pub struct Solution;

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut result = 0;
        let mut r_count = 0;
        let mut l_count = 0;
        for ch in s.chars() {
            if ch == 'R' {
                r_count += 1;
            } else {
                l_count += 1;
            }
            if r_count == l_count {
                result += 1;
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn balanced_string_split(s: String) -> i32 {
        Self::balanced_string_split(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
