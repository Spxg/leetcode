pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn helper(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
            if start == end {
                return vec![None];
            };

            let mut result = Vec::new();
            for val in start..end {
                let left_tree = helper(start, val);
                let right_tree = helper(val + 1, end);
                for left in &left_tree {
                    for right in &right_tree {
                        result.push(Some(Rc::new(RefCell::new(TreeNode {
                            val,
                            left: left.clone(),
                            right: right.clone(),
                        }))));
                    }
                }
            }
            result
        }

        helper(1, n + 1)
    }
}

impl super::Solution for Solution {
    fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Self::generate_trees(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
