pub struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut chs = [0; 26];
        magazine
            .bytes()
            .for_each(|ch| chs[(ch - b'a') as usize] += 1);
        ransom_note
            .bytes()
            .map(|ch| (ch - b'a') as usize)
            .all(|idx| {
                chs[idx] -= 1;
                chs[idx] >= 0
            })
    }
}

impl super::Solution for Solution {
    fn can_construct(ransom_note: String, magazine: String) -> bool {
        Self::can_construct(ransom_note, magazine)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
