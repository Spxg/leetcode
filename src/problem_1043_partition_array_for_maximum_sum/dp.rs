pub struct Solution;

impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![0; arr.len() + 1];
        for idx1 in 1..=arr.len() {
            let mut sum = 0;
            let mut current_max = 0;
            for idx2 in 1..=k {
                if idx2 > idx1 {
                    break;
                }
                current_max = current_max.max(arr[idx1 - idx2]);
                sum = (dp[idx1 - idx2] + idx2 as i32 * current_max).max(sum);
            }
            dp[idx1] = sum;
        }
        dp[arr.len()]
    }
}

impl super::Solution for Solution {
    fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        Self::max_sum_after_partitioning(arr, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
