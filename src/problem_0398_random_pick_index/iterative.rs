use std::collections::HashMap;

pub struct Solution {
    map: HashMap<i32, (usize, Vec<usize>)>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let mut map: HashMap<i32, (usize, Vec<usize>)> = HashMap::new();
        for (idx, value) in nums.into_iter().enumerate() {
            map.entry(value).or_default().1.push(idx);
        }
        Self { map }
    }

    fn pick(&mut self, target: i32) -> i32 {
        let val = self.map.get_mut(&target).unwrap();
        let idx = val.0 % val.1.len();
        let result = val.1[idx] as i32;
        val.0 += 1;
        result
    }
}

impl super::RandomPickIndex for Solution {
    fn new(val: &[i32]) -> Self {
        Self::new(val.to_vec())
    }

    fn pick(&mut self, target: i32) -> i32 {
        Self::pick(self, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
