pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        fn helper(nums: &[i32], sum: i32, set: &mut HashSet<(i32, usize)>) -> bool {
            if !set.insert((sum, nums.len())) {
                return false;
            }

            if let Some((&first, rest)) = nums.split_first() {
                if match first.cmp(&sum) {
                    std::cmp::Ordering::Less => {
                        helper(rest, sum - first, set) || helper(rest, sum, set)
                    }
                    std::cmp::Ordering::Equal => true,
                    std::cmp::Ordering::Greater => helper(rest, sum, set),
                } {
                    return true;
                }
            }
            false
        }

        let totol_sum: i32 = nums.iter().sum();
        totol_sum % 2 == 0 && helper(&nums, totol_sum / 2, &mut HashSet::new())
    }
}

impl super::Solution for Solution {
    fn can_partition(nums: Vec<i32>) -> bool {
        Self::can_partition(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
