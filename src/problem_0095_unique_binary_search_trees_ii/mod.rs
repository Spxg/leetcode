pub mod recursive;

use std::{cell::RefCell, rc::Rc};

use crate::data_structures::TreeNode;

pub trait Solution {
    fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [(
            3,
            &[
                &[Some(1), None, Some(2), None, Some(3)],
                &[Some(1), None, Some(3), Some(2)] as &[_],
                &[Some(2), Some(1), Some(3)],
                &[Some(3), Some(1), None, None, Some(2)],
                &[Some(3), Some(2), None, Some(1)],
            ] as &[&[_]],
        )];

        for (n, expected) in test_cases {
            assert_eq!(
                S::generate_trees(n),
                expected
                    .iter()
                    .map(|tree| test_utilities::make_tree(tree.iter().copied()))
                    .collect::<Box<_>>()
                    .as_ref()
            );
        }
    }
}
