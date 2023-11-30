pub struct Solution;

use crate::data_structures::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(postorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            postorder.split_last().map(|(&root, rest)| {
                let pos = inorder.iter().position(|&x| root == x).unwrap();
                Rc::new(RefCell::new(TreeNode {
                    val: root,
                    left: helper(&rest[0..pos], &inorder[0..pos]),
                    right: helper(&rest[pos..], &inorder[pos + 1..]),
                }))
            })
        }
        helper(&postorder, &inorder)
    }
}

impl super::Solution for Solution {
    fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree(inorder, postorder)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
