pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut result = 0;

        while left < right {
            let width = (right - left) as i32;
            let h = height[left].min(height[right]);
            result = result.max(width * h);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn max_area(height: Vec<i32>) -> i32 {
        Self::max_area(height)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
