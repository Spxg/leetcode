pub struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut mark = [0; 26];
        s.bytes().for_each(|idx| mark[(idx - b'a') as usize] += 1);
        for (idx, val) in s.bytes().enumerate() {
            if mark[(val - b'a') as usize] == 1 {
                return idx as i32;
            }
        }
        -1
    }
}

impl super::Solution for Solution {
    fn first_uniq_char(s: String) -> i32 {
        Self::first_uniq_char(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
