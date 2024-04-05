pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}

impl super::Solution for Solution {
    fn reverse_string(s: &mut Vec<char>) {
        Self::reverse_string(s);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
