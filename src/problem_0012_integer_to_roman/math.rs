pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let map = HashMap::from([
            (1, "I"),
            (4, "IV"),
            (5, "V"),
            (9, "IX"),
            (10, "X"),
            (40, "XL"),
            (50, "L"),
            (90, "XC"),
            (100, "C"),
            (400, "CD"),
            (500, "D"),
            (900, "CM"),
            (1000, "M"),
        ]);

        let mut num = num;
        let mut result = String::new();

        for i in [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1] {
            for _ in 0..num / i {
                result.push_str(map[&i]);
            }
            num %= i;
        }

        result
    }
}

impl super::Solution for Solution {
    fn int_to_roman(num: i32) -> String {
        Self::int_to_roman(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
