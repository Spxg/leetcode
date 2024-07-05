pub struct Solution;

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort_unstable();
        let mut left = 0;
        let mut right = people.len() - 1;
        let mut boats = 0;

        while left < right {
            if people[left] + people[right] <= limit {
                left += 1;
            }
            right -= 1;
            boats += 1;
        }
        if left == right {
            boats += 1;
        }

        boats
    }
}

impl super::Solution for Solution {
    fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        Self::num_rescue_boats(people, limit)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
