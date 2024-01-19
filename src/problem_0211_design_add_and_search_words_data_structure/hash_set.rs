use std::collections::HashSet;

pub struct WordDictionary {
    words: HashSet<String>,
}

impl WordDictionary {
    fn new() -> Self {
        Self {
            words: HashSet::new(),
        }
    }

    fn add_word(&mut self, word: String) {
        self.words.insert(word);
    }

    fn search(&self, word: String) -> bool {
        if word.contains('.') {
            self.words.iter().any(|w| {
                w.len() == word.len()
                    && w.chars().zip(word.chars()).all(|(x, y)| x == y || y == '.')
            })
        } else {
            self.words.contains(&word)
        }
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
