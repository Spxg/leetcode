use crate::data_structures::TreeNode;

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

struct Info {
    depth: i32,
    left: bool,
}

impl Info {
    fn new(depth: i32, left: bool) -> Self {
        Self { depth, left }
    }
}

struct CBTInserter {
    root: Option<Rc<RefCell<TreeNode>>>,
    queue: VecDeque<(Info, Rc<RefCell<TreeNode>>)>,
}

impl CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        fn helper(
            root: &Rc<RefCell<TreeNode>>,
            queue: &mut VecDeque<(Info, Rc<RefCell<TreeNode>>)>,
            depth: i32,
        ) {
            match (&root.borrow().left, &root.borrow().right) {
                (None, None) => {
                    queue.push_back((Info::new(depth, true), Rc::clone(root)));
                    queue.push_back((Info::new(depth, false), Rc::clone(root)));
                }
                (Some(left), None) => {
                    helper(left, queue, depth + 1);
                    queue.push_back((Info::new(depth, false), Rc::clone(root)));
                }
                (Some(left), Some(right)) => {
                    helper(left, queue, depth + 1);
                    helper(right, queue, depth + 1);
                }
                (None, Some(_)) => unreachable!(),
            }
        }

        let mut result = VecDeque::new();
        helper(root.as_ref().unwrap(), &mut result, 0);

        let pos = (0..result.len())
            .find(|&idx| result[idx].0.depth < result[0].0.depth)
            .unwrap_or(result.len());
        let mut queue = result.split_off(pos);
        queue.extend(result);

        Self { root, queue }
    }

    fn insert(&mut self, val: i32) -> i32 {
        let (info, node) = self.queue.pop_front().unwrap();
        let result = node.borrow().val;

        let new = Rc::new(RefCell::new(TreeNode::new(val)));
        self.queue
            .push_back((Info::new(info.depth + 1, true), Rc::clone(&new)));
        self.queue
            .push_back((Info::new(info.depth + 1, false), Rc::clone(&new)));

        if info.left {
            node.borrow_mut().left = Some(new);
        } else {
            node.borrow_mut().right = Some(new);
        }

        result
    }

    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.root.clone()
    }
}

impl super::CBTInserter for CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self::new(root)
    }

    fn insert(&mut self, val: i32) -> i32 {
        self.insert(val)
    }

    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.get_root()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::CBTInserter>();
    }
}
