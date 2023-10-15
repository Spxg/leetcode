pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        fn search_helper(nums: &[i32], target: i32) -> bool {
            let mut left = 0;
            let mut right = nums.len() - 1;
            while left <= right {
                let mid = left + (right - left) / 2;
                match nums[mid].cmp(&target) {
                    std::cmp::Ordering::Less => left = mid + 1,
                    std::cmp::Ordering::Equal => return true,
                    std::cmp::Ordering::Greater => {
                        if mid == 0 {
                            break;
                        }
                        right = mid - 1;
                    }
                }
            }
            false
        }

        match nums[0].cmp(&target) {
            std::cmp::Ordering::Less => {
                let left = 0;
                let mut right = nums.len() - 1;
                while nums[left] >= nums[right] {
                    if left + 1 > right {
                        break;
                    }
                    right -= 1;
                }
                search_helper(&nums[0..=right], target)
            }
            std::cmp::Ordering::Equal => true,
            std::cmp::Ordering::Greater => {
                let mut left = 0;
                let right = nums.len() - 1;
                while nums[left] >= nums[right] {
                    if left + 1 > right {
                        break;
                    }
                    left += 1;
                }
                search_helper(&nums[left..], target)
            }
        }
    }
}

impl super::Solution for Solution {
    fn search(nums: Vec<i32>, target: i32) -> bool {
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
