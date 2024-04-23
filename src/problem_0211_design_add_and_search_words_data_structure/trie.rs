#[derive(Default)]
struct WordDictionary {
    child: [Option<Box<WordDictionary>>; 26],
    end: bool,
}

impl WordDictionary {
    fn new() -> Self {
        Self::default()
    }

    fn add_word(&mut self, word: String) {
        let mut node = self;
        for &byte in word.as_bytes() {
            let idx = (byte - b'a') as usize;
            node = node.child[idx].get_or_insert(Box::<Self>::default());
        }
        node.end = true;
    }

    fn search(&self, word: String) -> bool {
        fn helper(dict: &WordDictionary, word: &[u8]) -> bool {
            if let Some((&first, rest)) = word.split_first() {
                if first == b'.' {
                    for dict in dict.child.iter().flatten() {
                        if helper(dict, rest) {
                            return true;
                        }
                    }
                    false
                } else {
                    dict.child[(first - b'a') as usize]
                        .as_ref()
                        .map_or(false, |d| helper(d, rest))
                }
            } else {
                dict.end
            }
        }
        helper(self, word.as_bytes())
    }
}

impl super::WordDictionary for WordDictionary {
    fn new() -> Self {
        Self::new()
    }

    fn add_word(&mut self, word: String) {
        self.add_word(word);
    }

    fn search(&self, word: String) -> bool {
        self.search(word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::WordDictionary>();
    }
}
