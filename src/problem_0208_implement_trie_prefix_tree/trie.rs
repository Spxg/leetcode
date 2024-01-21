#[derive(Default)]
pub struct Trie {
    trie: [Option<Box<Trie>>; 26],
    end: bool,
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: String) {
        let mut node = self;
        for &ch in word.as_bytes() {
            node = node.trie[(ch - b'a') as usize].get_or_insert_with(Box::<Self>::default);
        }
        node.end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut node = self;
        for &ch in word.as_bytes() {
            if let Some(n) = node.trie[(ch - b'a') as usize].as_ref() {
                node = n;
            } else {
                return false;
            }
        }
        node.end
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = self;
        for &ch in prefix.as_bytes() {
            if let Some(n) = node.trie[(ch - b'a') as usize].as_ref() {
                node = n;
            } else {
                return false;
            }
        }
        true
    }
}

impl super::Trie for Trie {
    fn new() -> Self {
        Self::new()
    }

    fn insert(&mut self, word: String) {
        self.insert(word);
    }

    fn search(&self, word: String) -> bool {
        self.search(word)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.starts_with(prefix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Trie>();
    }
}
