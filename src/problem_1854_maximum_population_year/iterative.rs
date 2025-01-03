pub struct Solution;

impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut population = 0;
        let mut max_population = 0;
        let mut years = [0; 100];
        for log in logs {
            years[(log[0] - 1950) as usize] += 1;
            years[(log[1] - 1950) as usize] -= 1;
        }
        for (y, p) in years.into_iter().enumerate() {
            population += p;
            if max_population < population {
                result = (1950 + y) as i32;
                max_population = population;
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        Self::maximum_population(logs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
