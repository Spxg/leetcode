pub struct Solution;

impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        let mut result = vec![label];

        let mut sum = 0;
        for level in 0.. {
            if sum + 2i32.pow(level) >= label {
                let mut target = label;
                for nth in (0..level).rev() {
                    let group = (target - sum + 1) / 2;
                    target = sum - group + 1;
                    result.push(target);
                    sum -= 2i32.pow(nth);
                }
                break;
            }
            sum += 2i32.pow(level);
        }

        result.into_iter().rev().collect()
    }
}

impl super::Solution for Solution {
    fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        Self::path_in_zig_zag_tree(label)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
