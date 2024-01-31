pub struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() {
            return vec![];
        }

        let mut i = 0;
        let mut result = vec![];
        let mut phrase = [0; 26];
        p.as_bytes()
            .iter()
            .for_each(|&x| phrase[(x - b'a') as usize] += 1);

        let (first, rest) = s.as_bytes().split_at(p.len());
        first.iter().for_each(|&x| phrase[(x - b'a') as usize] -= 1);
        if phrase.iter().all(|&x| x == 0) {
            result.push(i);
        }

        for (a, b) in s.bytes().zip(rest) {
            i += 1;
            phrase[(a - b'a') as usize] += 1;
            phrase[(b - b'a') as usize] -= 1;
            if phrase.iter().all(|&x| x == 0) {
                result.push(i);
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn find_anagrams(s: String, p: String) -> Vec<i32> {
        Self::find_anagrams(s, p)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
