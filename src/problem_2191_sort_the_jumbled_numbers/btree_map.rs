pub struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let mut map = BTreeMap::new();

        for (idx, &num) in nums.iter().enumerate() {
            let mut origin_num = num;
            let mut map_num = 0;
            let mut mul = 1;

            if origin_num == 0 {
                map.insert((mapping[0], idx), num);
                continue;
            }

            while origin_num != 0 {
                map_num += mapping[(origin_num % 10) as usize] * mul;
                origin_num /= 10;
                mul *= 10;
            }
            map.insert((map_num, idx), num);
        }

        map.values().copied().collect()
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
