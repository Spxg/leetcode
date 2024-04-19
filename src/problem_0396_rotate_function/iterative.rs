pub struct Solution;

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut result = 0;
        for (idx, &num) in (0..).zip(nums.iter()) {
            sum += num;
            result += idx * num;
        }
        let len = nums.len();
        let mut temp = result;
        for num in nums.into_iter().rev().take(len - 1) {
            temp = temp - (len as i32) * num + sum;
            result = result.max(temp);
        }
        result
    }
}

impl super::Solution for Solution {
    fn max_rotate_function(a: Vec<i32>) -> i32 {
        Self::max_rotate_function(a)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
