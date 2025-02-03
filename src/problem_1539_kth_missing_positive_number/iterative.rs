pub struct Solution;

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut nums = [0; 1001];
        for num in arr {
            nums[num as usize] = 1;
        }
        let mut n = 0;
        for idx in 1..1001 {
            if nums[idx] == 0 {
                n += 1;
                if n == k {
                    return idx as i32;
                }
            }
            if idx == 1000 {
                return idx as i32 + (k - n);
            }
        }
        unreachable!()
    }
}

impl super::Solution for Solution {
    fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        Self::find_kth_positive(arr, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
