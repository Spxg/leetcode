pub struct Solution;

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        #[derive(Default)]
        struct Dictionary {
            child: [Option<Box<Dictionary>>; 26],
            end: bool,
        }

        impl Dictionary {
            fn add_word(&mut self, word: &str) {
                let mut node = self;
                for byte in word.bytes() {
                    let idx = (byte - b'a') as usize;
                    node = node.child[idx].get_or_insert(Box::<Self>::default());
                }
                node.end = true;
            }

            fn find_root<'a>(&self, sentence: &'a str) -> Option<&'a str> {
                let mut node = self;
                let sentence = sentence.as_bytes();
                for (idx, &byte) in sentence.iter().enumerate() {
                    match &node.child[(byte - b'a') as usize] {
                        Some(child) if child.end => {
                            return Some(std::str::from_utf8(&sentence[0..=idx]).unwrap());
                        }
                        Some(child) => node = child,
                        None => return None,
                    }
                }
                None
            }
        }

        let mut dict = Dictionary::default();
        dictionary.into_iter().for_each(|x| dict.add_word(&x));
        sentence
            .split(' ')
            .map(|x| dict.find_root(x).unwrap_or(x))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl super::Solution for Solution {
    fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        Self::replace_words(dictionary, sentence)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
