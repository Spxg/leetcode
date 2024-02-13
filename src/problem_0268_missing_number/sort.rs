pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.push(-1);

        for target in 0..nums.len() {
            let mut num = nums[target];
            while num != target as i32 && num != -1 {
                nums.swap(num as usize, target);
                num = nums[target];
            }
        }

        nums.iter().position(|&x| x == -1).unwrap() as _
    }
}

impl super::Solution for Solution {
    fn missing_number(nums: Vec<i32>) -> i32 {
        Self::missing_number(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
