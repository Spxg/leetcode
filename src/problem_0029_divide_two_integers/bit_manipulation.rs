pub struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let negative = (dividend < 0) ^ (divisor < 0);
        let mut dividend = if dividend == i32::MIN {
            u32::from_le_bytes(dividend.to_le_bytes())
        } else {
            dividend.unsigned_abs()
        };
        let mut divisor = divisor.unsigned_abs();
        let max_offset = divisor.leading_zeros();
        let bound = u32::from_le_bytes(i32::MIN.to_le_bytes());

        let mut result = 0;
        let mut count = 0;

        while dividend >= (divisor << 1) && count < max_offset {
            count += 1;
            divisor <<= 1;
        }

        for offset in (0..=count).rev() {
            if dividend >= divisor {
                result += 1 << offset;
                dividend -= divisor;
            }
            divisor >>= 1;
        }

        if result >= bound {
            if negative {
                i32::MIN
            } else {
                i32::MAX
            }
        } else if negative {
            -(result as i32)
        } else {
            result as i32
        }
    }
}

impl super::Solution for Solution {
    fn divide(dividend: i32, divisor: i32) -> i32 {
        Self::divide(dividend, divisor)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
