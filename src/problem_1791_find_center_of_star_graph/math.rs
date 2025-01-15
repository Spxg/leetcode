pub struct Solution;

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let edge_part0 = &edges[0];
        let edge_part1 = &edges[1];
        if edge_part0[0] == edge_part1[0] || edge_part0[0] == edge_part1[1] {
            edge_part0[0]
        } else {
            edge_part0[1]
        }
    }
}

impl super::Solution for Solution {
    fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        Self::find_center(edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
