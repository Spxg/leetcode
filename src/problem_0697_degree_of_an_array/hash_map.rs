pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut result = (1, 1);
        let mut map: HashMap<i32, (usize, i32)> = HashMap::with_capacity(nums.len());
        for (idx, num) in nums.into_iter().enumerate() {
            if let Some((pos, count)) = map.get_mut(&num) {
                *count += 1;
                match result.1.cmp(count) {
                    std::cmp::Ordering::Less => result.0 = idx - *pos + 1,
                    std::cmp::Ordering::Equal => result.0 = (idx - *pos + 1).min(result.0),
                    std::cmp::Ordering::Greater => continue,
                }
                result.1 = *count;
            } else {
                map.insert(num, (idx, 1));
            }
        }
        result.0 as i32
    }
}

impl super::Solution for Solution {
    fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        Self::find_shortest_sub_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
