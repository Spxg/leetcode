pub struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = vec![];

        if nums.is_empty() {
            return result;
        }

        let mut f = |start: i32, end: i32| {
            result.push(if start == end {
                format!("{start}")
            } else {
                format!("{start}->{end}")
            });
        };

        let mut start = nums[0];
        for window in nums.windows(2) {
            match *window {
                [a, b] if a + 1 != b => {
                    f(start, a);
                    start = b;
                }
                _ => continue,
            }
        }

        let last = nums.last().copied().unwrap();
        f(start, last);

        result
    }
}

impl super::Solution for Solution {
    fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        Self::summary_ranges(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
