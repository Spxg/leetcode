pub struct Solution;

impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(barcodes.len());

        let mut nums = [0; 10001];
        let mut max_count = 0;
        for &barcode in &barcodes {
            nums[barcode as usize] += 1;
            max_count = max_count.max(nums[barcode as usize]);
        }

        let mut barcodes = barcodes;
        barcodes.sort_unstable_by(|&num1, &num2| {
            (nums[num2 as usize], num2).cmp(&(nums[num1 as usize], num1))
        });

        for idx in 0..max_count {
            for nums in barcodes.chunks(max_count) {
                if nums.len() > idx {
                    result.push(nums[idx]);
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn rearrange_barcodes(arr: Vec<i32>) -> Vec<i32> {
        Self::rearrange_barcodes(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
