struct OrderedStream {
    ptr: usize,
    value: Vec<Option<String>>,
}

impl OrderedStream {
    fn new(n: i32) -> Self {
        Self {
            ptr: 0,
            value: vec![None; n as usize],
        }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        self.value[id_key as usize - 1] = Some(value);
        let result = self.value[self.ptr..]
            .iter()
            .map_while(Clone::clone)
            .collect();
        while self.ptr < self.value.len() && self.value[self.ptr].is_some() {
            self.ptr += 1;
        }
        result
    }
}

impl super::OrderedStream for OrderedStream {
    fn new(n: i32) -> Self {
        Self::new(n)
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        self.insert(id_key, value)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::OrderedStream>();
    }
}
