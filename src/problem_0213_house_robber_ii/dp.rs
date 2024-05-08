pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        fn helper(nums: &[i32]) -> i32 {
            let (mut rob, mut no_rob) = (0, 0);
            for &num in nums {
                let new_no_rob = rob.max(no_rob);
                rob = no_rob + num;
                no_rob = new_no_rob;
            }
            rob.max(no_rob)
        }

        if nums.len() == 1 {
            nums[0]
        } else {
            helper(&nums[1..]).max(helper(&nums[..nums.len() - 1]))
        }
    }
}

impl super::Solution for Solution {
    fn rob(nums: Vec<i32>) -> i32 {
        Self::rob(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
