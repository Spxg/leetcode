pub struct Solution;

impl Solution {
    pub fn filter_restaurants(
        restaurants: Vec<Vec<i32>>,
        vegan_friendly: i32,
        max_price: i32,
        max_distance: i32,
    ) -> Vec<i32> {
        let mut restaurants = restaurants;
        restaurants.sort_unstable_by(|a, b| (b[1], b[0]).cmp(&(a[1], a[0])));
        restaurants
            .into_iter()
            .filter(|x| {
                x[3] <= max_price
                    && x[4] <= max_distance
                    && (vegan_friendly == 0 || vegan_friendly == x[2])
            })
            .map(|x| x[0])
            .collect()
    }
}

impl super::Solution for Solution {
    fn filter_restaurants(
        restaurants: Vec<Vec<i32>>,
        vegan_friendly: i32,
        max_price: i32,
        max_distance: i32,
    ) -> Vec<i32> {
        Self::filter_restaurants(restaurants, vegan_friendly, max_price, max_distance)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
