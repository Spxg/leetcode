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
            let mut point1 = idx + 1;
            let mut point2 = nums.len() - 1;

            while point1 < point2 {
                let sum = nums[idx] + nums[point1] + nums[point2];
                match sum.cmp(&0) {
                    std::cmp::Ordering::Less => point1 += 1,
                    std::cmp::Ordering::Equal => {
                        result.push(vec![nums[idx], nums[point1], nums[point2]]);
                        point2 -= 1;
                        point1 += 1;
                        while nums[point1] == nums[point1 - 1] && point1 < point2 {
                            point1 += 1;
                        }
                    }
                    std::cmp::Ordering::Greater => point2 -= 1,
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
