pub struct Solution;

impl Solution {
    /// m is moves, n is array's length, x is final number
    ///
    /// 1. `sum + m * (n - 1) = x * n`
    ///
    /// 2. `x = minNum + m`
    ///
    /// 4. `sum - minNum * n = m`
    ///
    /// 5. `m = (n1 - minNum) + (n2 - minNum) + ..`
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let min = nums.iter().min().copied().unwrap();
        nums.into_iter().fold(0, |acc, x| acc + (x - min))
    }
}

impl super::Solution for Solution {
    fn min_moves(nums: Vec<i32>) -> i32 {
        Self::min_moves(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
