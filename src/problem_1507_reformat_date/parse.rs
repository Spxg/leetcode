pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn reformat_date(date: String) -> String {
        let months = HashMap::from([
            ("Jan", "01"),
            ("Feb", "02"),
            ("Mar", "03"),
            ("Apr", "04"),
            ("May", "05"),
            ("Jun", "06"),
            ("Jul", "07"),
            ("Aug", "08"),
            ("Sep", "09"),
            ("Oct", "10"),
            ("Nov", "11"),
            ("Dec", "12"),
        ]);
        let (day, rest) = date.split_once(' ').unwrap();
        let (month, year) = rest.split_once(' ').unwrap();
        let day = day[0..day.len() - 2].parse::<i32>().unwrap();

        format!("{}-{}-{:02}", year, months[&month], day)
    }
}

impl super::Solution for Solution {
    fn reformat_date(date: String) -> String {
        Self::reformat_date(date)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
