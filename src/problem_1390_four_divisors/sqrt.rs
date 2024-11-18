pub struct Solution;

impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for num in nums {
            let max = f64::from(num).sqrt() as i32;
            if max * max == num {
                continue;
            }

            let mut sum = 0;
            let mut count = 0;
            for val in (1..=max).filter(|&x| (num % x == 0)) {
                sum += val;
                sum += num / val;
                count += 1;
            }
            if count == 2 {
                result += sum;
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        Self::sum_four_divisors(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
