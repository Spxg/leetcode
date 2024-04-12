pub struct Solution;

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for &num in &nums {
            min = min.min(num);
            max = max.max(num);
        }
        if max == min {
            return 0;
        }

        let bucket_size = {
            let n = nums.len() as i32;
            (max - min + n - 2) / (n - 1)
        };
        let mut buckets = vec![(i32::MAX, i32::MIN); nums.len()];
        for num in nums {
            let index = ((num - min) / bucket_size) as usize;
            buckets[index].0 = buckets[index].0.min(num);
            buckets[index].1 = buckets[index].1.max(num);
        }

        let mut result = 0;
        let mut prev_max = buckets[0].1;
        for &(min, max) in &buckets[1..] {
            if min == i32::MAX {
                continue;
            }
            result = result.max(min - prev_max);
            prev_max = max;
        }
        result
    }
}

impl super::Solution for Solution {
    fn maximum_gap(nums: Vec<i32>) -> i32 {
        Self::maximum_gap(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
