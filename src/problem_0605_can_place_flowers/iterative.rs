pub struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut can = 0;
        let mut prev_idx = None;
        for idx in flowerbed
            .into_iter()
            .chain([0, 1])
            .enumerate()
            .filter_map(|(idx, bed)| (bed == 1).then_some(idx as i32))
        {
            if can >= n {
                return true;
            }
            let size = prev_idx.map_or_else(|| idx - 1, |prev| idx - prev - 3);
            if size > 0 {
                can += (size + 1) / 2;
            }
            prev_idx = Some(idx);
        }
        can >= n
    }
}

impl super::Solution for Solution {
    fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        Self::can_place_flowers(flowerbed, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
