pub struct Solution;

impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(num.len());
        let mut k = k;
        let mut iter = num.into_iter().rev();
        let mut inc = 0;

        while k != 0 {
            let reminder = k % 10;
            let num = iter
                .next()
                .map_or_else(|| reminder + inc, |num| num + reminder + inc);
            result.push(num % 10);

            k /= 10;
            inc = num / 10;
        }

        for num in iter {
            let num = num + inc;
            result.push(num % 10);
            inc = num / 10;
        }

        if inc == 1 {
            result.push(1);
        }

        result.into_iter().rev().collect()
    }
}

impl super::Solution for Solution {
    fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        Self::add_to_array_form(num, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
