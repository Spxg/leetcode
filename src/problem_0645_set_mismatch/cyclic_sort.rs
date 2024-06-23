pub struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut result = vec![0, 0];
        for idx1 in 0..nums.len() {
            let mut num = nums[idx1];
            let expected = idx1 as i32 + 1;
            while num != expected {
                let idx2 = num as usize - 1;
                let target = nums[idx2];
                if target == num {
                    break;
                }
                nums.swap(idx1, idx2);
                num = target;
            }
        }
        for (expected, num) in nums
            .into_iter()
            .enumerate()
            .map(|(idx, n)| (idx as i32 + 1, n))
        {
            if expected != num {
                result[0] = num;
                result[1] = expected;
                break;
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        Self::find_error_nums(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
