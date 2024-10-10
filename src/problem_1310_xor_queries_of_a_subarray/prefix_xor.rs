pub struct Solution;

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::with_capacity(queries.len());
        let mut xors = Vec::with_capacity(arr.len() + 1);
        xors.push(0);

        for num in arr {
            let xor = xors[xors.len() - 1] ^ num;
            xors.push(xor);
        }
        for query in queries {
            let (start, end) = (query[0], query[1]);
            result.push(xors[start as usize] ^ xors[end as usize + 1]);
        }
        result
    }
}

impl super::Solution for Solution {
    fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        Self::xor_queries(arr, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
