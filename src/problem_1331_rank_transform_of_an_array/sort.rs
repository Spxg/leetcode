pub struct Solution;

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; arr.len()];
        let mut nums: Vec<_> = arr.into_iter().zip(0usize..).collect();
        nums.sort_unstable();
        let mut prev = None;
        let mut count = 1;
        for (num, idx) in nums {
            if let Some(val) = prev {
                if val != num {
                    count += 1;
                }
            }
            result[idx] = count;
            prev = Some(num);
        }
        result
    }
}

impl super::Solution for Solution {
    fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        Self::array_rank_transform(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
