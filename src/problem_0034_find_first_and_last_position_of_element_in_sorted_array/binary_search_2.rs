pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] == target {
                right = mid;
            } else if nums[mid] < target {
                left = mid + 1;
            } else if let Some(x) = mid.checked_sub(1) {
                right = x;
            } else {
                break;
            }
        }

        if nums[right] == target {
            let start = right;
            let nums = &nums[right..];
            let mut left = 0;
            let mut right = nums.len() - 1;

            while left < right {
                let mid = left + (right - left) / 2;
                if nums[mid] <= target {
                    left = mid + 1;
                } else if let Some(x) = mid.checked_sub(1) {
                    right = x;
                } else {
                    break;
                }
            }

            if nums[left] == target {
                vec![start as i32, (start + left) as i32]
            } else {
                vec![start as i32, (start + left - 1) as i32]
            }
        } else {
            vec![-1, -1]
        }
    }
}

impl super::Solution for Solution {
    fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::search_range(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
