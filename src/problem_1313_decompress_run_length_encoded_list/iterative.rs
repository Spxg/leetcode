pub struct Solution;

impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];

        for chunk in nums.chunks(2) {
            result.extend(std::iter::repeat(chunk[1]).take(chunk[0] as _));
        }

        result
    }
}

impl super::Solution for Solution {
    fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        Self::decompress_rl_elist(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
