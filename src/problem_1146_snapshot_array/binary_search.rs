use std::collections::HashMap;

struct SnapshotArray {
    changed: HashMap<i32, i32>,
    snap: HashMap<i32, Vec<(i32, i32)>>,
    snap_id: i32,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        let _ = length;
        Self {
            changed: HashMap::new(),
            snap: HashMap::new(),
            snap_id: 0,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.changed.insert(index, val);
    }

    fn snap(&mut self) -> i32 {
        let snap_id = self.snap_id;
        for (idx, value) in std::mem::take(&mut self.changed) {
            self.snap.entry(idx).or_default().push((snap_id, value));
        }
        self.snap_id += 1;
        snap_id
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        self.snap.get(&index).map_or(0, |snap| {
            match snap.binary_search_by(|&(id, _)| id.cmp(&snap_id)) {
                Ok(pos) => snap[pos].1,
                Err(pos) => {
                    if pos == 0 {
                        return 0;
                    }
                    snap[pos - 1].1
                }
            }
        })
    }
}

impl super::SnapshotArray for SnapshotArray {
    fn new(length: i32) -> Self {
        Self::new(length)
    }

    fn set(&mut self, index: i32, val: i32) {
        self.set(index, val);
    }

    fn snap(&mut self) -> i32 {
        self.snap()
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        self.get(index, snap_id)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::SnapshotArray>();
    }
}
