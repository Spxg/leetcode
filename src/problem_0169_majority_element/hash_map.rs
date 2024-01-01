pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let times = nums.len() as i32 / 2;

        for num in nums {
            *map.entry(num).or_default() += 1;
            if map[&num] > times {
                return num;
            }
        }

        unreachable!()
    }
}

impl super::Solution for Solution {
    fn majority_element(nums: Vec<i32>) -> i32 {
        Self::majority_element(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
