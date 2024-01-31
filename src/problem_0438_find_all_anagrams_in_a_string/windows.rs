pub struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let s_len = s.len();
        let p_len = p.len();
        if s_len < p_len {
            return vec![];
        }

        let mut result = vec![];
        let mut phrase = [0; 26];
        p.as_bytes()
            .iter()
            .for_each(|&x| phrase[(x - b'a') as usize] += 1);

        let mut prev;
        let mut pointer = 0;

        let s = s.as_bytes();

        while pointer <= s_len - p_len {
            let slice = &s[pointer..pointer + p_len];
            let mut bitmap = [0; 26];
            slice.iter().for_each(|&x| bitmap[(x - b'a') as usize] += 1);
            if bitmap == phrase {
                result.push(pointer as _);
            }
            prev = pointer;
            pointer += 1;
            while bitmap == phrase && pointer <= s_len - p_len && s[pointer + p_len - 1] == s[prev]
            {
                result.push(pointer as _);
                prev = pointer;
                pointer += 1;
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
