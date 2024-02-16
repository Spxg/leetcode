pub struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;

        for idx in 0..nums.len() {
            let mut target = nums[idx] as usize;
            while target != idx + 1 && nums[idx] != nums[target - 1] {
                nums.swap(target - 1, idx);
                target = nums[idx] as usize;
            }
        }

        (1..)
            .zip(nums)
            .filter_map(|(a, b)| (a != b).then_some(a))
            .collect()
    }
}

impl super::Solution for Solution {
    fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        Self::find_disappeared_numbers(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
