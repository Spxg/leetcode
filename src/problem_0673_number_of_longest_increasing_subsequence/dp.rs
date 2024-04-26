pub struct Solution;

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut cache = Vec::<(i32, i32)>::with_capacity(nums.len());
        let mut result = (0, 0);
        for &num in &nums {
            let mut temp = (1, 1);
            for (&(size, count), &x) in cache.iter().zip(&nums) {
                if x < num {
                    match (size + 1).cmp(&temp.0) {
                        std::cmp::Ordering::Less => (),
                        std::cmp::Ordering::Equal => temp.1 += count,
                        std::cmp::Ordering::Greater => {
                            temp.0 = size + 1;
                            temp.1 = count;
                        }
                    }
                }
            }
            cache.push(temp);
        }
        for (size, count) in cache {
            match size.cmp(&result.0) {
                std::cmp::Ordering::Less => (),
                std::cmp::Ordering::Equal => result.1 += count,
                std::cmp::Ordering::Greater => {
                    result.0 = size;
                    result.1 = count;
                }
            }
        }
        result.1
    }
}

impl super::Solution for Solution {
    fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        Self::find_number_of_lis(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
