pub struct Solution;

impl Solution {
    pub fn longest_mountain(arr: Vec<i32>) -> i32 {
        let mut up = 0;
        let mut down = 0;
        let mut result = 0;
        for nums in arr.windows(2) {
            match nums[0].cmp(&nums[1]) {
                std::cmp::Ordering::Less if down == 0 => up += 1,
                std::cmp::Ordering::Greater if up != 0 => down += 1,
                std::cmp::Ordering::Less if down != 0 => {
                    result = result.max(up + down + 1);
                    up = 1;
                    down = 0;
                }
                std::cmp::Ordering::Equal if down != 0 => {
                    result = result.max(up + down + 1);
                    up = 0;
                    down = 0;
                }
                _ => {
                    up = 0;
                    down = 0;
                }
            }
        }
        if down != 0 {
            result = result.max(up + down + 1);
        }
        result
    }
}

impl super::Solution for Solution {
    fn longest_mountain(arr: Vec<i32>) -> i32 {
        Self::longest_mountain(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
