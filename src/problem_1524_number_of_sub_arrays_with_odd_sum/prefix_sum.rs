pub struct Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut result = 0i64;

        let mut even = 1;
        let mut odd = 0;

        let mut sum = 0;
        for num in arr {
            sum += num;
            if sum % 2 == 0 {
                result += odd;
                even += 1;
            } else {
                result += even;
                odd += 1;
            }
        }
        (result % (10i64.pow(9) + 7)) as _
    }
}

impl super::Solution for Solution {
    fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        Self::num_of_subarrays(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
