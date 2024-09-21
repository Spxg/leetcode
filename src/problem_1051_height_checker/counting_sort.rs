pub struct Solution;

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut idx = 0;

        let mut counts = [0; 101];
        heights
            .iter()
            .copied()
            .for_each(|x| counts[x as usize] += 1);

        for height in heights {
            loop {
                let count = &mut counts[idx as usize];
                if *count == 0 {
                    idx += 1;
                } else {
                    *count -= 1;

                    break;
                }
            }
            result += i32::from(height != idx);
        }

        result
    }
}

impl super::Solution for Solution {
    fn height_checker(heights: Vec<i32>) -> i32 {
        Self::height_checker(heights)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
