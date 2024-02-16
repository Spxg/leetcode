pub struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut mark = (1..=nums.len() as i32).collect::<Vec<i32>>();
        nums.into_iter().for_each(|x| mark[x as usize - 1] = 0);
        mark.into_iter().filter(|&x| x != 0).collect()
    }
}

impl super::Solution for Solution {
    fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        Self::find_disappeared_numbers(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
