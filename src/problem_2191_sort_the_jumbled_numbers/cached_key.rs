pub struct Solution;

impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        fn helper(mapping: &[i32], num: i32) -> i32 {
            let mut origin_num = num;
            let mut map_num = 0;
            let mut mul = 1;

            if origin_num == 0 {
                return mapping[0];
            }

            while origin_num != 0 {
                map_num += mapping[(origin_num % 10) as usize] * mul;
                origin_num /= 10;
                mul *= 10;
            }

            map_num
        }
        let mut nums = nums;
        nums.sort_by_cached_key(|&num| helper(&mapping, num));
        nums
    }
}

impl super::Solution for Solution {
    fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        Self::sort_jumbled(mapping, nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
