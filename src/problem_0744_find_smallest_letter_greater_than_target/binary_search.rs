pub struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let pos = letters.partition_point(|&ch| ch <= target);
        if pos == letters.len() {
            letters[0]
        } else {
            letters[pos]
        }
    }
}

impl super::Solution for Solution {
    fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        Self::next_greatest_letter(letters, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
