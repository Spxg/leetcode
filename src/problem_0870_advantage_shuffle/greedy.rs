pub struct Solution;

impl Solution {
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums1.len());
        let mut used = vec![false; nums1.len()];
        let mut nums1 = nums1;
        nums1.sort_unstable();

        for num in nums2 {
            let point = nums1.partition_point(|&x| x < num + 1);
            let idx = (point..nums1.len())
                .chain(0..point)
                .find(|&x| !used[x])
                .unwrap();
            result.push(nums1[idx]);
            used[idx] = true;
        }

        result
    }
}

impl super::Solution for Solution {
    fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        Self::advantage_count(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
