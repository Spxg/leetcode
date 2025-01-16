pub struct Solution;

impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut box_types = box_types;
        box_types.sort_unstable_by_key(|x| -x[1]);
        let mut result = 0;
        let mut truck_size = truck_size;
        for box_ in box_types {
            if truck_size == 0 {
                break;
            }
            let num = truck_size.min(box_[0]);
            result += num * box_[1];
            truck_size -= num;
        }
        result
    }
}

impl super::Solution for Solution {
    fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        Self::maximum_units(box_types, truck_size)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
