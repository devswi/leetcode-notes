#[derive(Debug, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    pub fn from(vals: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current: Option<Box<ListNode>> = None;
        for &v in vals.iter().rev() {
            let mut node = ListNode::new(v);
            node.next = current;
            current = Some(Box::new(node));
        }
        current
    }
}

#[macro_export]
macro_rules! linked {
    ($($e:expr),*) => {
        ListNode::from(vec![$($e.to_owned()), *])
    };
    ($($e:expr),*) => {
        ListNode::from(vec![$($e.to_owned()), *])
    };
}
