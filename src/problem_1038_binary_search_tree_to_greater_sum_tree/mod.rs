pub mod recursive;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub trait Solution {
    fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    Some(4),
                    Some(1),
                    Some(6),
                    Some(0),
                    Some(2),
                    Some(5),
                    Some(7),
                    None,
                    None,
                    None,
                    Some(3),
                    None,
                    None,
                    None,
                    Some(8),
                ] as &[_],
                &[
                    Some(30),
                    Some(36),
                    Some(21),
                    Some(36),
                    Some(35),
                    Some(26),
                    Some(15),
                    None,
                    None,
                    None,
                    Some(33),
                    None,
                    None,
                    None,
                    Some(8),
                ] as &[_],
            ),
            (&[Some(0), None, Some(1)], &[Some(1), None, Some(1)]),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::bst_to_gst(test_utilities::make_tree(root.iter().copied())),
                test_utilities::make_tree(expected.iter().copied())
            );
        }
    }
}
