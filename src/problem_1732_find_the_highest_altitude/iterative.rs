pub struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut sum = 0;
        gain.into_iter()
            .map(|x| {
                sum += x;
                sum
            })
            .max()
            .unwrap()
            .max(0)
    }
}

impl super::Solution for Solution {
    fn largest_altitude(gain: Vec<i32>) -> i32 {
        Self::largest_altitude(gain)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
