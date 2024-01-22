use rand::Rng;
use std::collections::HashMap;

pub struct Solution {
    map: HashMap<i32, Vec<usize>>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        for (idx, value) in nums.into_iter().enumerate() {
            map.entry(value).or_default().push(idx);
        }
        Self { map }
    }

    fn pick(&self, target: i32) -> i32 {
        let val = &self.map[&target];
        val[rand::thread_rng().gen_range(0, val.len())] as i32
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
