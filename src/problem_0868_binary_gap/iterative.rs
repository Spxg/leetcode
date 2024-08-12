pub struct Solution;

impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut n = n;
        let mut result = 0;
        let mut distance = None;

        while n != 0 {
            distance = if n & 1 == 1 {
                if let Some(dist) = distance {
                    result = result.max(dist);
                }
                Some(1)
            } else {
                distance.map(|x| x + 1)
            };
            n >>= 1;
        }

        result
    }
}

impl super::Solution for Solution {
    fn binary_gap(n: i32) -> i32 {
        Self::binary_gap(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
