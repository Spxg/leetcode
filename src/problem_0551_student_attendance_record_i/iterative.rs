pub struct Solution;

impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut late = 0;
        let mut absent = 0;
        for ch in s.chars() {
            if ch == 'A' {
                absent += 1;
                late = 0;
            } else if ch == 'L' {
                late += 1;
            } else if ch == 'P' {
                late = 0;
            }
            if absent == 2 || late == 3 {
                return false;
            }
        }
        true
    }
}

impl super::Solution for Solution {
    fn check_record(s: String) -> bool {
        Self::check_record(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
