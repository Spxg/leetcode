pub mod iterative;
pub mod rng;

pub trait RandomPickIndex {
    fn new(val: &[i32]) -> Self;
    fn pick(&mut self, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::RandomPickIndex;

    pub fn run<S: RandomPickIndex>() {
        let val = &[1, 2, 3, 3, 3];
        let mut s = S::new(val);
        let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();
        for (idx, val) in val.iter().enumerate() {
            map.entry(*val).or_default().insert(idx as i32);
        }
        assert!(val.iter().all(|&x| map[&x].contains(&s.pick(x))));
        assert!(val.iter().all(|&x| map[&x].contains(&s.pick(x))));
        assert!(val.iter().all(|&x| map[&x].contains(&s.pick(x))));
        assert!(val.iter().all(|&x| map[&x].contains(&s.pick(x))));
        assert!(val.iter().all(|&x| map[&x].contains(&s.pick(x))));
    }
}
