/*
 * @lc app=leetcode id=23 lang=rust
 *
 * [23] Merge k Sorted Lists
 */

use crate::util::singly_linked_list::*;

pub struct Solution;

// @lc code=start
impl Solution {
    fn merge(list1: Box<ListNode>, list2: Box<ListNode>) -> Box<ListNode> {
        let (mut n1, mut n2) = (Some(&list1), Some(&list2));
        let mut list = Box::new(ListNode::new(0));
        let mut node = &mut list;

        loop {
            match (n1, n2) {
                (Some(l1), Some(l2)) => {
                    if l1.val < l2.val {
                        node.next = Some(Box::new(ListNode::new(l1.val)));
                        node = node.next.as_mut().unwrap();
                        n1 = l1.next.as_ref();
                    } else {
                        node.next = Some(Box::new(ListNode::new(l2.val)));
                        node = node.next.as_mut().unwrap();
                        n2 = l2.next.as_ref();
                    }
                }
                (Some(l1), None) => {
                    node.next = Some(Box::new(ListNode::new(l1.val)));
                    node = node.next.as_mut().unwrap();
                    n1 = l1.next.as_ref();
                }
                (None, Some(l2)) => {
                    node.next = Some(Box::new(ListNode::new(l2.val)));
                    node = node.next.as_mut().unwrap();
                    n2 = l2.next.as_ref();
                }
                (None, None) => break,
            }
        }

        list.next.unwrap()
    }

    fn merge_all(mut lists: Vec<Box<ListNode>>) -> Option<Box<ListNode>> {
        if lists.len() < 2 {
            return lists.into_iter().next();
        }

        let (right, left) = (lists.split_off(lists.len() / 2), lists);
        let left_node = Solution::merge_all(left);
        let right_node = Solution::merge_all(right);

        match (left_node, right_node) {
            (Some(l), Some(r)) => Some(Solution::merge(l, r)),
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (None, None) => None,
        }
    }

    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // 对输入进行过滤，过滤掉空的链表
        let filtered_lists: Vec<Box<ListNode>> = lists
            .into_iter()
            .filter(|val| val.is_some())
            .map(|val| val.unwrap())
            .collect();

        Solution::merge_all(filtered_lists)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::list;

    #[test]
    fn test() {
        assert_eq!(
            Solution::merge_k_lists(vec![list![1, 4, 5], list![1, 3, 4], list![2, 6]]),
            list![1, 1, 2, 3, 4, 4, 5, 6]
        );
        assert_eq!(Solution::merge_k_lists(vec![]), list![]);
        assert_eq!(Solution::merge_k_lists(vec![list![]]), list![]);
    }
}
