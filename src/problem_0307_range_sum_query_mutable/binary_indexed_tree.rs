struct BinaryIndexedTree(Vec<i32>);

impl BinaryIndexedTree {
    fn new(capacity: usize) -> Self {
        Self(vec![0; capacity])
    }

    fn sum(&self, index: usize) -> i32 {
        let mut idx = index as i32;
        let mut sum = 0;
        while idx > 0 {
            sum += self.0[idx as usize];
            idx -= idx & (-idx);
        }
        sum
    }

    fn add(&mut self, index: usize, val: i32) {
        let mut idx = index as i32;
        while (idx as usize) < self.0.len() {
            self.0[idx as usize] += val;
            idx += idx & (-idx);
        }
    }
}

struct NumArray {
    tree: BinaryIndexedTree,
    nums: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut tree = BinaryIndexedTree::new(nums.len() + 1);
        for (idx, &num) in (1..).zip(&nums) {
            tree.add(idx, num);
        }
        Self { tree, nums }
    }

    fn update(&mut self, index: i32, val: i32) {
        let idx = index as usize;
        let diff = val - self.nums[idx];
        self.nums[idx] = val;
        self.tree.add(index as usize + 1, diff);
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.tree.sum(right as usize + 1) - self.tree.sum(left as usize)
    }
}

impl super::NumArray for NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self::new(nums)
    }

    fn update(&mut self, i: i32, val: i32) {
        self.update(i, val);
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
