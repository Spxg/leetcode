pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        #[allow(clippy::ref_option)]
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut i32, k: i32, count: &mut i32) {
            if let Some(node) = node {
                helper(&node.borrow().left, result, k, count);
                *count += 1;
                match (*count).cmp(&k) {
                    std::cmp::Ordering::Less => helper(&node.borrow().right, result, k, count),
                    std::cmp::Ordering::Equal => *result = node.borrow().val,
                    std::cmp::Ordering::Greater => (),
                }
            }
        }

        let mut result = 0;
        helper(&root, &mut result, k, &mut 0);
        result
    }
}

impl super::Solution for Solution {
    fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        Self::kth_smallest(root, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
