pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut left, mut cur, mut right) = (0, 0, nums.len());

        while cur < right {
            if nums[cur] == 0 {
                nums.swap(left, cur);
                left += 1;
                cur += 1;
            } else if nums[cur] == 2 {
                right -= 1;
                nums.swap(cur, right);
            } else {
                cur += 1;
            }
        }
    }
}

impl super::Solution for Solution {
    fn sort_colors(nums: &mut Vec<i32>) {
        Self::sort_colors(nums);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
