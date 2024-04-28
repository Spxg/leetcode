pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; temperatures.len()];
        'loop1: for idx in (0..temperatures.len() - 1).rev() {
            if temperatures[idx] < temperatures[idx + 1] {
                result[idx] = 1;
            } else {
                let mut offset = result[idx + 1] as usize + 1;
                while temperatures[idx + offset] <= temperatures[idx] {
                    if result[idx + offset] == 0 {
                        result[idx] = 0;
                        continue 'loop1;
                    }
                    offset += result[idx + offset] as usize;
                }
                result[idx] = offset as i32;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        Self::daily_temperatures(t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
