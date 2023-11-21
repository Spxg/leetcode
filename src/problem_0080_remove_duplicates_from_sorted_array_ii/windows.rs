pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut remove_idxs = vec![];
        for (idx, element) in nums.windows(3).enumerate() {
            if let [prev, mid, last] = *element {
                if prev == mid && prev == last {
                    remove_idxs.push(idx + 2);
                }
            }
        }

        remove_idxs
            .iter()
            .enumerate()
            .for_each(|(idx, remove_idx)| {
                nums.remove(remove_idx - idx);
            });

        nums.len() as i32
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
