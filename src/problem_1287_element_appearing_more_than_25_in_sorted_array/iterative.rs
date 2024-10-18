pub struct Solution;

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let min_count = arr.len() / 4 + 1;
        let (mut cur, mut count) = (arr[0], 0);
        for num in arr {
            if num == cur {
                count += 1;
                if count >= min_count {
                    return cur;
                }
            } else {
                cur = num;
                count = 1;
            }
        }
        unreachable!()
    }
}

impl super::Solution for Solution {
    fn find_special_integer(arr: Vec<i32>) -> i32 {
        Self::find_special_integer(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
