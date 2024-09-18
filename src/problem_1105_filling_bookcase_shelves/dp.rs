pub struct Solution;

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let mut dp = vec![i32::MAX; books.len() + 1];
        dp[0] = 0;
        for idx1 in 0..books.len() {
            let (mut width, mut max_height) = (0, 0);
            for idx2 in (0..=idx1).rev() {
                width += books[idx2][0];
                if width > shelf_width {
                    break;
                }
                max_height = max_height.max(books[idx2][1]);
                dp[idx1 + 1] = dp[idx1 + 1].min(dp[idx2] + max_height);
            }
        }
        dp[books.len()]
    }
}

impl super::Solution for Solution {
    fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        Self::min_height_shelves(books, shelf_width)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
