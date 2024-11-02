pub struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut result = Vec::with_capacity(2 * n);
        for (x, y) in (0..n).zip(n..) {
            result.push(nums[x]);
            result.push(nums[y]);
        }
        result
    }
}

impl super::Solution for Solution {
    fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        Self::shuffle(nums, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
