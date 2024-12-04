pub struct Solution;

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut prev = nums[0];
        let mut increasing = None;
        for &num in &nums[1..] {
            match (num.cmp(&prev), increasing) {
                (std::cmp::Ordering::Less, None) => increasing = Some(false),
                (std::cmp::Ordering::Greater, None) => increasing = Some(true),
                (std::cmp::Ordering::Less, Some(true))
                | (std::cmp::Ordering::Greater, Some(false)) => return false,
                _ => (),
            };
            prev = num;
        }
        true
    }
}

impl super::Solution for Solution {
    fn is_monotonic(nums: Vec<i32>) -> bool {
        Self::is_monotonic(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
