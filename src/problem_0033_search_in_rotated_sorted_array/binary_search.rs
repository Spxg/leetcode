pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        nums.first()
            .and_then(|first| {
                let pivot = nums.partition_point(|x| x >= first);
                if *first > target {
                    nums[pivot..].binary_search(&target).map(|x| x + pivot)
                } else {
                    nums[0..pivot].binary_search(&target)
                }
                .ok()
            })
            .map_or(-1, |x| x as _)
    }
}

impl super::Solution for Solution {
    fn search(nums: Vec<i32>, target: i32) -> i32 {
        Self::search(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
