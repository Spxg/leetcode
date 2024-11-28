use std::collections::HashMap;

struct UndergroundSystem {
    map: HashMap<i32, (String, i32)>,
    average: HashMap<(String, String), (i32, i32)>,
}

impl UndergroundSystem {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            average: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.map.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let (start_station, start) = self.map.remove(&id).unwrap();
        self.average
            .entry((start_station, station_name))
            .and_modify(|x| *x = (x.0 + 1, x.1 + t - start))
            .or_insert((1, t - start));
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        self.average
            .get(&(start_station, end_station))
            .copied()
            .map(|(count, total)| f64::from(total) / f64::from(count))
            .unwrap()
    }
}

impl super::UndergroundSystem for UndergroundSystem {
    fn new() -> Self {
        Self::new()
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.check_in(id, station_name, t);
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        self.check_out(id, station_name, t);
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        self.get_average_time(start_station, end_station)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::UndergroundSystem>();
    }
}
