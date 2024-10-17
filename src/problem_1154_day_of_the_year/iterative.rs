pub struct Solution;

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let (year, date) = date
            .split_once('-')
            .map(|(x, y)| (x.parse::<i32>().unwrap(), y))
            .unwrap();
        let (month, day) = date
            .split_once('-')
            .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<i32>().unwrap()))
            .unwrap();
        let offset = i32::from(year % 4 == 0 && year % 100 != 0 || year % 400 == 0);
        let map = [31, 28 + offset, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        map[0..month - 1].iter().copied().sum::<i32>() + day
    }
}

impl super::Solution for Solution {
    fn day_of_year(date: String) -> i32 {
        Self::day_of_year(date)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
