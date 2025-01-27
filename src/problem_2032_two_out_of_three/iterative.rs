pub struct Solution;

impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(100);
        let mut map = [[false; 100]; 3];
        for num in nums1 {
            map[0][num as usize - 1] = true;
        }
        for num in nums2 {
            map[1][num as usize - 1] = true;
        }
        for num in nums3 {
            map[2][num as usize - 1] = true;
        }
        for idx in 0..100 {
            if i32::from(map[0][idx]) + i32::from(map[1][idx]) + i32::from(map[2][idx]) > 1 {
                result.push(idx as i32 + 1);
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        Self::two_out_of_three(nums1, nums2, nums3)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
