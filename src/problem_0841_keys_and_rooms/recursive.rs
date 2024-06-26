pub struct Solution;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        fn helper(rooms: &[Vec<i32>], keys: &[i32], visited: &mut [bool]) {
            for key in keys.iter().map(|&x| x as usize) {
                if visited[key] {
                    continue;
                }
                visited[key] = true;
                helper(rooms, &rooms[key], visited);
            }
        }
        let mut rooms_visited = vec![false; rooms.len()];
        rooms_visited[0] = true;
        helper(&rooms, &rooms[0], &mut rooms_visited);
        rooms_visited.into_iter().all(|x| x)
    }
}

impl super::Solution for Solution {
    fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        Self::can_visit_all_rooms(rooms)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
