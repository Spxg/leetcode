pub struct Solution;

impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut sum = 0;
        let mut prev = 0;
        let size = customers.len() as i32;
        for customer in customers {
            let (arrival, time) = (i64::from(customer[0]), i64::from(customer[1]));
            if arrival >= prev {
                sum += time;
                prev = arrival + time;
            } else {
                sum += prev - arrival + time;
                prev += time;
            };
        }
        #[allow(clippy::pedantic)]
        let sum = sum as f64;
        sum / f64::from(size)
    }
}

impl super::Solution for Solution {
    fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        Self::average_waiting_time(customers)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
