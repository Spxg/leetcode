pub struct Solution;

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut map = [0; 101];
        let mut sum = 0;
        for num in nums {
            if map[num as usize] == -1 {
                continue;
            }
            if map[num as usize] == 0 {
                map[num as usize] = 1;
            } else {
                map[num as usize] = -1;
            }
        }
        for (idx, &val) in map.iter().enumerate().skip(1) {
            if val == 1 {
                sum += idx as i32;
            }
        }
        sum
    }
}

impl super::Solution for Solution {
    fn sum_of_unique(nums: Vec<i32>) -> i32 {
        Self::sum_of_unique(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
