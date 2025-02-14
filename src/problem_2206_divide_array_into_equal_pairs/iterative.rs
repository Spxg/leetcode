pub struct Solution;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        nums.sort_unstable();
        for chunk in nums.chunks(2) {
            let num = chunk[0];
            if chunk.iter().any(|&x| num != x) {
                return false;
            }
        }
        true
    }
}

impl super::Solution for Solution {
    fn divide_array(nums: Vec<i32>) -> bool {
        Self::divide_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
