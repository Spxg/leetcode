use std::collections::HashMap;

#[derive(Default)]
pub struct MapSum {
    map: HashMap<String, i32>,
    dict: Dict,
}

#[derive(Default)]
struct Dict {
    child: [Option<Box<Dict>>; 26],
    value: i32,
}

impl MapSum {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, key: String, val: i32) {
        let sub = self.map.insert(key.clone(), val).unwrap_or(0);
        let mut node = &mut self.dict;
        for ch in key.bytes() {
            node = node.child[(ch - b'a') as usize].get_or_insert(Box::<Dict>::default());
            node.value = node.value - sub + val;
        }
    }

    fn sum(&self, prefix: String) -> i32 {
        let mut node = &self.dict;
        for ch in prefix.bytes() {
            if let Some(n) = node.child[(ch - b'a') as usize].as_ref() {
                node = n;
            } else {
                return 0;
            }
        }
        node.value
    }
}

impl super::MapSum for MapSum {
    fn new() -> Self {
        Self::new()
    }

    fn insert(&mut self, key: String, val: i32) {
        self.insert(key, val);
    }

    fn sum(&self, prefix: String) -> i32 {
        self.sum(prefix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::MapSum>();
    }
}
