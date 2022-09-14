use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T>
where
    T: Copy,
{
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from(vals: &[Option<T>]) -> Option<Self> {
        let (first, rest) = vals.split_first().unwrap();
        if let Some(val) = first {
            let mut root: TreeNode<T> = TreeNode::new(*val);
            return Some(root);
        }
        None
    }
}
