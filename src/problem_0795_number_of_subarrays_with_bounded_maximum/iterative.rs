pub struct Solution;

impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        let mut result = 0;

        let mut idx = 1;
        let mut prev = 0;

        // [32, 69]
        // 55 36 5 55 14 9 7
        // 1  2  2 4  4  4 4 = 21
        for num in nums {
            if num > right {
                idx = 1;
                prev = 0;
            } else {
                if num < left {
                    result += prev;
                } else {
                    result += idx;
                    prev = idx;
                }
                idx += 1;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        Self::num_subarray_bounded_max(nums, left, right)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
