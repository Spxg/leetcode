pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut flag = 1;
        let mut result = vec![0, 0];

        let xor = nums.iter().fold(0, |acc, x| acc ^ x);
        while xor & flag == 0 {
            flag <<= 1;
        }

        for num in nums {
            if num & flag == 0 {
                result[0] ^= num;
            } else {
                result[1] ^= num;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn single_number(nums: Vec<i32>) -> Vec<i32> {
        Self::single_number(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
