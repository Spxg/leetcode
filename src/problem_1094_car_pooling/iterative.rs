pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut capacity = capacity;

        let mut pick: HashMap<i32, i32> = HashMap::with_capacity(trips.len());
        let mut drop: HashMap<i32, i32> = HashMap::with_capacity(trips.len());

        let mut start = i32::MAX;
        let mut end = i32::MIN;

        for trip in trips {
            let (passengers, from, to) = (trip[0], trip[1], trip[2]);
            *pick.entry(from).or_default() += passengers;
            *drop.entry(to).or_default() += passengers;

            start = start.min(from);
            end = end.max(to);
        }

        for nth in start..=end {
            if let Some(passengers) = drop.remove(&nth) {
                capacity += passengers;
            }

            if let Some(passengers) = pick.remove(&nth) {
                capacity -= passengers;
            }

            if capacity < 0 {
                return false;
            }
        }

        true
    }
}

impl super::Solution for Solution {
    fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        Self::car_pooling(trips, capacity)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
