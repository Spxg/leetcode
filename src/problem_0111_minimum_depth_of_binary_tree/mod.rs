pub mod recursive;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub trait Solution {
    fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)] as &[_],
                2,
            ),
            (
                &[
                    Some(2),
                    None,
                    Some(3),
                    None,
                    Some(4),
                    None,
                    Some(5),
                    None,
                    Some(6),
                ],
                5,
            ),
            (&[Some(0)], 1),
            (&[], 0),
            (
                &[Some(1), Some(2), Some(3), Some(4), None, None, Some(5)],
                3,
            ),
            (&[Some(1), Some(2), Some(3), Some(4), Some(5)], 2),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::min_depth(test_utilities::make_tree(root.iter().copied())),
                expected
            );
        }
    }
}
