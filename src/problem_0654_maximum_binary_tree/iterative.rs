use crate::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = Vec::<Rc<RefCell<TreeNode>>>::with_capacity(nums.len());

        for num in nums {
            let mut left = None;
            let node = loop {
                if let Some(last) = stack.last() {
                    let mut last = last.borrow_mut();

                    if last.val < num {
                        drop(last);
                        left = stack.pop();
                    } else {
                        let node = Rc::new(RefCell::new(TreeNode {
                            val: num,
                            left,
                            right: None,
                        }));
                        last.right = Some(Rc::clone(&node));
                        break node;
                    }
                } else {
                    break Rc::new(RefCell::new(TreeNode {
                        val: num,
                        left,
                        right: None,
                    }));
                }
            };
            stack.push(node);
        }

        stack.into_iter().next()
    }
}

impl super::Solution for Solution {
    fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::construct_maximum_binary_tree(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
