pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut nums = Vec::new();
        let mut map = HashMap::new();
        let mut num = num;
        while num != 0 {
            let digit = num % 10;
            map.entry(digit).or_insert_with(|| nums.len());
            nums.push(digit);
            num /= 10;
        }

        let mut nums_sorted = nums.clone();
        nums_sorted.sort_unstable();

        for idx in (0..nums.len()).rev() {
            if nums[idx] < nums_sorted[idx] {
                nums.swap(idx, map[&nums_sorted[idx]]);
                break;
            }
        }

        nums.into_iter().rev().fold(0, |acc, x| acc * 10 + x)
    }
}

impl super::Solution for Solution {
    fn maximum_swap(num: i32) -> i32 {
        Self::maximum_swap(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
