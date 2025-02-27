pub mod queue;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub trait Solution {
    fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    Some(1),
                    Some(10),
                    Some(4),
                    Some(3),
                    None,
                    Some(7),
                    Some(9),
                    Some(12),
                    Some(8),
                    Some(6),
                    None,
                    None,
                    Some(2),
                ] as &[_],
                true,
            ),
            (
                &[Some(5), Some(4), Some(2), Some(3), Some(3), Some(7)],
                false,
            ),
            (
                &[Some(5), Some(9), Some(1), Some(3), Some(5), Some(7)],
                false,
            ),
            (&[Some(2)], false),
            (
                &[
                    Some(1),
                    Some(4),
                    Some(2),
                    Some(3),
                    Some(5),
                    Some(7),
                    Some(9),
                ],
                true,
            ),
        ];

        for (root, expected) in test_cases {
            assert_eq!(
                S::is_even_odd_tree(test_utilities::make_tree(root.iter().copied())),
                expected,
            );
        }
    }
}
