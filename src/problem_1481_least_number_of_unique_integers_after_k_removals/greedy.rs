pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        for &num in &arr {
            *map.entry(num).or_insert(0) += 1;
        }
        let mut nums = arr;
        nums.sort_unstable_by_key(|&x| (map[&x], x));
        for num in &nums[0..k as usize] {
            let count = map.get(num).copied().unwrap();
            if count == 1 {
                map.remove(num);
            } else {
                *map.get_mut(num).unwrap() -= 1;
            }
        }
        map.len() as _
    }
}

impl super::Solution for Solution {
    fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        Self::find_least_num_of_unique_ints(arr, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
