pub struct Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn helpr(nums: &mut [i32]) -> Option<Vec<i32>> {
            for idx1 in (0..nums.len()).rev() {
                let slice = &mut nums[idx1..];
                for last in (0..slice.len()).rev() {
                    for idx2 in 0..last {
                        if slice[idx2] < slice[last] {
                            slice.swap(idx2, last);
                            slice[idx2 + 1..].reverse();
                            return Some(nums.to_vec());
                        }
                    }
                }
            }
            None
        }

        let mut nums = nums;
        nums.sort_unstable();
        let mut result = vec![nums.clone()];
        while let Some(ele) = helpr(&mut nums) {
            result.push(ele);
        }
        result
    }
}

impl super::Solution for Solution {
    fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::permute_unique(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
