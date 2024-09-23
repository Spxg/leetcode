pub struct Solution;

use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    paths: HashMap<String, Trie>,
    end: bool,
}

impl Trie {
    fn push(&mut self, path: &str) -> bool {
        let mut trie = self;

        for path in path.split('/').skip(1) {
            trie = trie.paths.entry(path.into()).or_default();
            if trie.end {
                return false;
            }
        }

        trie.end = true;
        true
    }
}

impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut folder = folder;
        folder.sort_unstable();
        let mut trie = Trie::default();
        folder.into_iter().filter(|f| trie.push(f)).collect()
    }
}

impl super::Solution for Solution {
    fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        Self::remove_subfolders(folder)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
