pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let negative = numerator < 0 && denominator > 0 || numerator > 0 && denominator < 0;
        let denominator = i64::from(denominator).abs();
        let mut numerator = i64::from(numerator).abs();
        let mut result = vec![];

        if negative {
            result.push(b'-');
        }
        result.extend((numerator / denominator).to_string().bytes());
        numerator %= denominator;
        if numerator != 0 {
            result.push(b'.');
            numerator *= 10;
        }

        let mut cache = HashMap::new();
        while numerator != 0 {
            if let Some(&idx) = cache.get(&numerator) {
                result.insert(idx, b'(');
                result.push(b')');
                break;
            }
            cache.insert(numerator, result.len());
            result.push(b'0' + (numerator / denominator) as u8);
            numerator = (numerator % denominator) * 10;
        }

        String::from_utf8(result).unwrap()
    }
}

impl super::Solution for Solution {
    fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        Self::fraction_to_decimal(numerator, denominator)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
