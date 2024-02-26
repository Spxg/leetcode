pub struct Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        s.bytes().chain(t.bytes()).fold(0, |acc, x| acc ^ x) as char
    }
}

impl super::Solution for Solution {
    fn find_the_difference(s: String, t: String) -> char {
        Self::find_the_difference(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
