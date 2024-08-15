pub struct Solution;

impl Solution {
    pub fn sum_even_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::with_capacity(queries.len());
        let mut nums = nums;
        let mut sum = nums.iter().filter(|&x| x % 2 == 0).sum::<i32>();
        for query in queries {
            let idx = query[1] as usize;
            let num = query[0];
            match (nums[idx] % 2 == 0, num % 2 == 0) {
                (true, true) => sum += num,
                (true, false) => sum -= nums[idx],
                (false, false) => sum += nums[idx] + num,
                (false, true) => (),
            }
            nums[idx] += num;
            result.push(sum);
        }
        result
    }
}

impl super::Solution for Solution {
    fn sum_even_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        Self::sum_even_after_queries(nums, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
