struct NumArray {
    inner: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut inner = vec![0; nums.len() + 1];

        for (idx, num) in (1..).zip(nums) {
            inner[idx] = num + inner[idx - 1];
        }

        Self { inner }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.inner[right as usize + 1] - self.inner[left as usize]
    }
}

impl super::NumArray for NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self::new(nums)
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.sum_range(i, j)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::NumArray>();
    }
}
