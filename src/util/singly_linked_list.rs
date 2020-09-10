/// LeetCode 提供的单向列表定义
/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[macro_export]
macro_rules! list {
    () => { None };
    ($($x:expr),*) => {{
        let vec = vec![$($x),*];
        let mut list = None;
        for &v in vec.iter().rev() {
            let mut node = ListNode::new(v);
            node.next = list;
            list = Some(Box::new(node));
        }
        list
    }};
}
