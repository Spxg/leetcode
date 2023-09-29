pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<_, Vec<String>> = HashMap::new();
        for string in strs {
            let mut sig = [0; 26];
            for ch in string.chars() {
                sig[(ch as u8 - b'a') as usize] += 1;
            }
            map.entry(sig).or_default().push(string);
        }
        map.into_values().collect()
    }
}

impl super::Solution for Solution {
    fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        Self::group_anagrams(strs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
