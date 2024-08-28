pub struct Solution;

impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut result: Option<String> = None;

        let mut license = [0; 26];
        license_plate
            .bytes()
            .filter(|&x| x.is_ascii_alphabetic())
            .map(|x| x.to_ascii_lowercase())
            .for_each(|x| license[(x - b'a') as usize] += 1);

        for word in words {
            let mut chs = [0; 26];
            word.bytes().for_each(|x| chs[(x - b'a') as usize] += 1);
            if chs.iter().zip(license.iter()).all(|(a, b)| a >= b) {
                match result
                    .as_ref()
                    .map_or(usize::MAX, String::len)
                    .cmp(&word.len())
                {
                    std::cmp::Ordering::Less | std::cmp::Ordering::Equal => continue,
                    std::cmp::Ordering::Greater => result = Some(word),
                }
            }
        }

        result.unwrap()
    }
}

impl super::Solution for Solution {
    fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        Self::shortest_completing_word(license_plate, words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
