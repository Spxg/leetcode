pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut result = 1;
        let mut replace_idx = vec![];
        nums.windows(2)
            .enumerate()
            .for_each(|(x, elements)| match *elements {
                [lhs, rhs] if lhs != rhs => {
                    replace_idx.push(x + 1);
                    result += 1;
                }
                _ => (),
            });
        for (idx1, idx2) in (1..).zip(replace_idx) {
            nums[idx1] = nums[idx2];
        }
        result
    }
}

impl super::Solution for Solution {
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        Self::remove_duplicates(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
