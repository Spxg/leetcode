pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (index1, &num1) in nums.split_last().unwrap().1.iter().enumerate() {
            let next = index1 + 1;
            for (index2, &num2) in (next..).zip(&nums[next..]) {
                if num1 + num2 == target {
                    return vec![index1 as i32, index2 as i32];
                }
            }
        }

        // Description:
        // You may assume that each input would have exactly one solution
        unreachable!();
    }
}

impl super::Solution for Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::two_sum(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
