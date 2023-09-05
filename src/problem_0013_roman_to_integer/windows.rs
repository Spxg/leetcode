pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let map = HashMap::from([
            (b'I', 1),
            (b'V', 5),
            (b'X', 10),
            (b'L', 50),
            (b'C', 100),
            (b'D', 500),
            (b'M', 1000),
        ]);

        let bytes = s.as_bytes();
        let last = map[bytes.last().unwrap()];
        bytes.windows(2).rev().fold(last, |acc, x| {
            let lhs = map[&x[0]];
            let rhs = map[&x[1]];
            acc + if lhs < rhs { -lhs } else { lhs }
        })
    }
}

impl super::Solution for Solution {
    fn roman_to_int(s: String) -> i32 {
        Self::roman_to_int(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
