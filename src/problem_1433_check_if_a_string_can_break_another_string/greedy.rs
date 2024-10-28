pub struct Solution;

impl Solution {
    pub fn check_if_can_break(s1: String, s2: String) -> bool {
        let mut s1 = s1.into_bytes();
        let mut s2 = s2.into_bytes();
        s1.sort_unstable();
        s2.sort_unstable();

        let mut mode = None;
        for (x, y) in s1.into_iter().zip(s2) {
            if let Some(mode) = mode {
                if mode && x > y || (!mode) && x < y {
                    return false;
                }
            } else if x < y {
                mode = Some(true);
            } else if x > y {
                mode = Some(false);
            }
        }

        true
    }
}

impl super::Solution for Solution {
    fn check_if_can_break(s1: String, s2: String) -> bool {
        Self::check_if_can_break(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
