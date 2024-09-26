pub struct Solution;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let pos = nums
            .iter()
            .enumerate()
            .filter_map(|(i, &n)| (n % 2 != 0).then_some(i))
            .collect::<Vec<_>>();
        let mut result = 0;
        let k = k as usize;

        for start_idx in 0..pos.len() {
            let end_idx = start_idx + k - 1;
            if end_idx >= pos.len() {
                break;
            }
            let (start, end) = (pos[start_idx], pos[end_idx]);
            let left = if start_idx == 0 {
                start + 1
            } else {
                start - pos[start_idx - 1]
            };
            let right = if end_idx == pos.len() - 1 {
                nums.len() - end
            } else {
                pos[end_idx + 1] - pos[end_idx]
            };
            result += left * right;
        }

        result as i32
    }
}

impl super::Solution for Solution {
    fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        Self::number_of_subarrays(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
