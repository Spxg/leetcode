use crate::data_structures::NestedInteger;

pub struct NestedIterator {
    it: std::iter::Peekable<StackedIterator>,
}

struct StackedIterator {
    stack: Vec<std::vec::IntoIter<NestedInteger>>,
}

impl Iterator for StackedIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(last) = self.stack.last_mut() {
            match last.next() {
                Some(NestedInteger::Int(val)) => return Some(val),
                Some(NestedInteger::List(list)) => self.stack.push(list.into_iter()),
                None => drop(self.stack.pop()),
            }
        }
        None
    }
}

impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        Self {
            it: StackedIterator {
                stack: vec![nested_list.into_iter()],
            }
            .peekable(),
        }
    }

    fn next(&mut self) -> i32 {
        self.it.next().unwrap()
    }

    fn has_next(&mut self) -> bool {
        self.it.peek().is_some()
    }
}

impl super::NestedIterator for NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        Self::new(nested_list)
    }

    fn next(&mut self) -> i32 {
        self.next()
    }

    fn has_next(&mut self) -> bool {
        self.has_next()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::NestedIterator>();
    }
}
