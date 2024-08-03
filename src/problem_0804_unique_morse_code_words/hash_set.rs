pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let map = [
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];

        let mut set = HashSet::with_capacity(words.len());

        for word in words {
            set.insert(
                word.bytes()
                    .map(|x| map[(x - b'a') as usize])
                    .collect::<String>(),
            );
        }

        set.len() as _
    }
}

impl super::Solution for Solution {
    fn unique_morse_representations(words: Vec<String>) -> i32 {
        Self::unique_morse_representations(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
