pub struct Solution;

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut frequency = [0; 201];
        for &num in &nums {
            let idx = if num < 0 { 100 - num } else { num };
            frequency[idx as usize] += 1;
        }
        let mut nums = nums;
        nums.sort_unstable_by_key(|&num| {
            let idx = if num < 0 { 100 - num } else { num };
            (frequency[idx as usize], -num)
        });
        nums
    }
}

impl super::Solution for Solution {
    fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        Self::frequency_sort(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
