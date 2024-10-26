pub struct Solution;

impl Solution {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut vec = Vec::with_capacity((hi - lo) as usize);
        for num in lo..=hi {
            let mut n = num;
            let mut step = 0;
            while n != 1 {
                step += 1;
                n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
            }
            vec.push((step, num));
        }
        vec.select_nth_unstable(k as usize - 1).1 .1
    }
}

impl super::Solution for Solution {
    fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        Self::get_kth(lo, hi, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
