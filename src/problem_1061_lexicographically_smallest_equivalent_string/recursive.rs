pub struct Solution;

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        fn helper(a: usize, b: usize, map: &mut [[bool; 26]; 26]) {
            if map[a][b] {
                return;
            }
            map[a][b] = true;
            for idx in 0..26 {
                if map[a][idx] {
                    helper(idx, b, map);
                    helper(b, idx, map);
                }
            }
        }

        let mut result = String::with_capacity(base_str.len());
        let mut map = [[false; 26]; 26];

        for (ch1, ch2) in s1
            .bytes()
            .map(|x| (x - b'a') as usize)
            .zip(s2.bytes().map(|x| (x - b'a') as usize))
        {
            map[ch1][ch1] = true;
            map[ch2][ch2] = true;
            helper(ch1, ch2, &mut map);
            helper(ch2, ch1, &mut map);
        }

        for idx in base_str.bytes().map(|x| (x - b'a') as usize) {
            let pos = map[idx].into_iter().position(|x| x).unwrap_or(idx);
            result.push((pos as u8 + b'a') as char);
        }

        result
    }
}

impl super::Solution for Solution {
    fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        Self::smallest_equivalent_string(s1, s2, base_str)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
