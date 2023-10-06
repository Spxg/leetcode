pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut result = vec![];

        for idx in 0..nums.len() {
            if idx > 0 && nums[idx] == nums[idx - 1] {
                continue;
            }
            let mut pointer1 = idx + 1;
            let mut pointer2 = nums.len() - 1;

            while pointer1 < pointer2 {
                let sum = nums[idx] + nums[pointer1] + nums[pointer2];
                match sum.cmp(&0) {
                    std::cmp::Ordering::Less => pointer1 += 1,
                    std::cmp::Ordering::Equal => {
                        result.push(vec![nums[idx], nums[pointer1], nums[pointer2]]);
                        pointer1 += 1;
                        pointer2 -= 1;
                        while nums[pointer1] == nums[pointer1 - 1] && pointer1 < pointer2 {
                            pointer1 += 1;
                        }
                        while nums[pointer2] == nums[pointer2 + 1] && pointer1 < pointer2 {
                            pointer2 -= 1;
                        }
                    }
                    std::cmp::Ordering::Greater => pointer2 -= 1,
                }
            }
        }

        result.into_iter().collect()
    }
}

impl super::Solution for Solution {
    fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::three_sum(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
