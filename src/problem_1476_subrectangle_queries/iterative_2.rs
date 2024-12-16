struct SubrectangleQueries {
    rectangle: Vec<Vec<i32>>,
    operations: Vec<(i32, i32, i32, i32, i32)>,
}

impl SubrectangleQueries {
    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        Self {
            rectangle,
            operations: vec![],
        }
    }

    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        self.operations.push((row1, col1, row2, col2, new_value));
    }

    fn get_value(&self, row: i32, col: i32) -> i32 {
        for &(row1, col1, row2, col2, val) in self.operations.iter().rev() {
            if row >= row1 && row <= row2 && col >= col1 && col <= col2 {
                return val;
            }
        }
        self.rectangle[row as usize][col as usize]
    }
}

impl super::SubrectangleQueries for SubrectangleQueries {
    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        Self::new(rectangle)
    }

    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        self.update_subrectangle(row1, col1, row2, col2, new_value);
    }

    fn get_value(&self, row: i32, col: i32) -> i32 {
        self.get_value(row, col)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::SubrectangleQueries>();
    }
}
