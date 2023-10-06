pub struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut result = vec![];

        for head in 0..nums.len() - 1 {
            if head > 0 && nums[head] == nums[head - 1] {
                continue;
            }
            let first = nums[head];
            let new_target = i64::from(target - first);

            let nums = &nums[head + 1..];
            for idx in 0..nums.len() {
                if idx > 0 && nums[idx] == nums[idx - 1] {
                    continue;
                }
                let mut pointer1 = idx + 1;
                let mut pointer2 = nums.len() - 1;

                while pointer1 < pointer2 {
                    let sum = i64::from(nums[idx]) + i64::from(nums[pointer1]) + i64::from(nums[pointer2]);
                    match sum.cmp(&new_target) {
                        std::cmp::Ordering::Less => pointer1 += 1,
                        std::cmp::Ordering::Equal => {
                            result.push(vec![first, nums[idx], nums[pointer1], nums[pointer2]]);
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
        }

        result.into_iter().collect()
    }
}

impl super::Solution for Solution {
    fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        Self::four_sum(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
