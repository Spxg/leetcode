pub struct Solution;

impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let bytes = s.into_bytes();

        let mut result = vec![];
        let mut prev = (bytes[0], 0, 0);

        for (idx, &ch) in (1..).zip(&bytes[1..]) {
            if ch == prev.0 {
                prev.2 = idx;
            } else {
                if prev.2 - prev.1 > 1 {
                    result.push(vec![prev.1, prev.2]);
                }
                prev = (ch, idx, idx);
            }
        }

        if prev.2 - prev.1 > 1 {
            result.push(vec![prev.1, prev.2]);
        }

        result
    }
}

impl super::Solution for Solution {
    fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        Self::large_group_positions(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
