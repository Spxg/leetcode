pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        fn search_inner(nums: &[i32], target: i32) -> i32 {
            let mut left = 0;
            if nums.is_empty() {
                return -1;
            }
            let mut right = nums.len() - 1;

            while left <= right {
                let mid = left + (right - left) / 2;
                match nums[mid].cmp(&target) {
                    std::cmp::Ordering::Less => left = mid + 1,
                    std::cmp::Ordering::Equal => return mid as _,
                    std::cmp::Ordering::Greater => {
                        if mid == 0 {
                            break;
                        }
                        right = mid - 1;
                    }
                }
            }
            -1
        }

        match nums[0].cmp(&target) {
            std::cmp::Ordering::Less => {
                let left = 0;
                let mut right = nums.len() - 1;
                while nums[left] > nums[right] {
                    right -= 1;
                }
                search_inner(&nums[left..=right], target)
            }
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => {
                let mut left = 0;
                let right = nums.len() - 1;
                while nums[left] > nums[right] {
                    left += 1;
                }
                let result = search_inner(&nums[left..=right], target);
                if result == -1 {
                    -1
                } else {
                    left as i32 + result
                }
            }
        }
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
