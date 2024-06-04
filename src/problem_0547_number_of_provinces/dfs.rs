pub struct Solution;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        fn helper(is_connected: &Vec<Vec<i32>>, city: usize, visited: &mut [bool]) {
            for (idx, val) in is_connected[city].iter().enumerate() {
                if !visited[idx] && *val == 1 {
                    visited[idx] = true;
                    helper(is_connected, idx, visited);
                }
            }
        }

        let mut result = 0;
        let mut visited = [false; 200];
        for city in 0..is_connected.len() {
            if !visited[city] {
                result += 1;
                visited[city] = true;
                helper(&is_connected, city, &mut visited);
            }
        }
        result
    }
}

impl super::Solution for Solution {
    fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        Self::find_circle_num(is_connected)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
