pub struct Solution;

impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let n = n as usize;
        let mut sums = Vec::with_capacity(n * (n + 1) / 2);

        for idx in 0..n {
            let mut sum = 0;
            for &num in &nums[idx..] {
                sum += num;
                sums.push(sum);
            }
        }

        let left = left as usize - 1;
        let right = right as usize - 1;

        sums.select_nth_unstable(left);
        sums[left..].select_nth_unstable(right - left);

        (sums[left..=right]
            .iter()
            .map(|&x| i64::from(x))
            .sum::<i64>()
            % (10i64.pow(9) + 7)) as i32
    }
}

impl super::Solution for Solution {
    fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        Self::range_sum(nums, n, left, right)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
