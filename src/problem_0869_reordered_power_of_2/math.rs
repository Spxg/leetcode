pub struct Solution;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        fn helper(mut num: u64) -> (u32, [i32; 10]) {
            let mut count = [0; 10];
            let mut size = 0;
            while num != 0 {
                size += 1;
                count[(num % 10) as usize] += 1;
                num /= 10;
            }
            (size, count)
        }

        let (size, count) = helper(n as u64);

        #[allow(clippy::cast_precision_loss)]
        let (min, max) = {
            let min = 10u64.pow(size - 1) as f64;
            let max = 10u64.pow(size) as f64;
            let min = min.log2().trunc() as u32;
            let max = max.log2().ceil() as u32;
            (min, max)
        };

        for base in min..=max {
            let val = 2u64.pow(base);
            let (_, target) = helper(val);
            if count == target {
                return true;
            }
        }
        false
    }
}

impl super::Solution for Solution {
    fn reordered_power_of2(n: i32) -> bool {
        Self::reordered_power_of2(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
