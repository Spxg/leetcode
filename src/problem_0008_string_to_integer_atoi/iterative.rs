pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut result = 0i32;
        let mut negative = false;
        let mut fuck = false;

        for ch in s.chars() {
            if ch.is_ascii_whitespace() {
                if fuck {
                    break;
                }
                continue;
            }

            if ['+', '-'].contains(&ch) {
                if fuck {
                    break;
                }
                fuck = true;
                if ch == '-' {
                    negative = true;
                }
            } else if ch.is_ascii_digit() {
                fuck = true;
                let digit = ch.to_digit(10).unwrap() as i32;
                result =
                    result
                        .saturating_mul(10)
                        .saturating_add(if negative { -digit } else { digit });
            } else {
                break;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn my_atoi(str: String) -> i32 {
        Self::my_atoi(str)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
