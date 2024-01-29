pub struct Solution;

impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut g = g;
        g.sort_unstable();
        let mut s = s;
        s.sort_unstable();

        let mut iter = s.into_iter();
        'loop1: for x in g {
            for n in iter.by_ref() {
                if n >= x {
                    result += 1;
                    continue 'loop1;
                }
                continue;
            }
            break 'loop1;
        }

        result
    }
}

impl super::Solution for Solution {
    fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        Self::find_content_children(g, s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
