pub mod recursive;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub trait Solution {
    fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[Some(3), Some(1), Some(4), Some(3), None, Some(1), Some(5)] as &[_],
                4,
            ),
            (&[Some(3), Some(3), None, Some(4), Some(2)], 3),
            (&[Some(1)], 1),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::good_nodes(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
