pub struct Solution;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let mut counts = vec![0; k as usize];
        counts[0] = 1;

        let mut sum = 0;
        for num in nums {
            sum = (sum + num).rem_euclid(k);
            result += counts[sum as usize];
            counts[sum as usize] += 1;
        }

        result
    }
}

impl super::Solution for Solution {
    fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        Self::subarrays_div_by_k(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
