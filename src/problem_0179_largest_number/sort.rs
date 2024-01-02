pub struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
        nums.sort_unstable_by(|lhs, rhs| {
            rhs.bytes()
                .chain(lhs.bytes())
                .cmp(lhs.bytes().chain(rhs.bytes()))
        });

        if nums[0] == "0" {
            "0".into()
        } else {
            nums.into_iter().collect()
        }
    }
}

impl super::Solution for Solution {
    fn largest_number(nums: Vec<i32>) -> String {
        Self::largest_number(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
