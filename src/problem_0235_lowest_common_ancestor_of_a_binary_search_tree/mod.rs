pub mod iterative;

use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Solution {
    fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[
                        Some(6),
                        Some(2),
                        Some(8),
                        Some(0),
                        Some(4),
                        Some(7),
                        Some(9),
                        None,
                        None,
                        Some(3),
                        Some(5),
                    ] as &[_],
                    2,
                    8,
                ),
                6,
            ),
            (
                (
                    &[
                        Some(6),
                        Some(2),
                        Some(8),
                        Some(0),
                        Some(4),
                        Some(7),
                        Some(9),
                        None,
                        None,
                        Some(3),
                        Some(5),
                    ],
                    2,
                    4,
                ),
                2,
            ),
            ((&[Some(2), Some(1)], 2, 1), 2),
            (
                (
                    &[
                        Some(6),
                        Some(2),
                        Some(8),
                        Some(0),
                        Some(4),
                        Some(7),
                        Some(9),
                        None,
                        None,
                        Some(3),
                        Some(5),
                    ],
                    3,
                    5,
                ),
                4,
            ),
        ];

        for ((root, p, q), expected) in test_cases {
            let root = test_utilities::make_tree(root.iter().copied());
            let p = test_utilities::find_node(&root, p);
            let q = test_utilities::find_node(&root, q);
            let expected = test_utilities::find_node(&root, expected);

            assert_eq!(S::lowest_common_ancestor(root, p, q), expected);
        }
    }
}
