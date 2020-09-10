//! 因为两链表的第一个都是个位数，依此类推，所以可以从左到右每个数字依次相加，
//! 将相加的结果放入链表中，并记录进位信息（下一次相加时加 1）
//!
//! - [循环版](https://leetcode.com/submissions/detail/393740556/)
//! - [递归版](https://leetcode.com/submissions/detail/393748699/)

/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 */

use crate::util::singly_linked_list::*;

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(n1), Some(n2)) => {
                let sum = n1.val + n2.val;
                if sum < 10 {
                    Some(Box::new(ListNode {
                        val: sum,
                        next: Solution::add_two_numbers(n1.next, n2.next),
                    }))
                } else {
                    let carry = Some(Box::new(ListNode::new(1)));
                    Some(Box::new(ListNode {
                        val: sum - 10,
                        next: Solution::add_two_numbers(
                            Solution::add_two_numbers(carry, n1.next),
                            n2.next,
                        ),
                    }))
                }
            }
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    #[test]
    fn test() {
        assert_eq!(
            Solution::add_two_numbers(list![2, 4, 3], list![5, 6, 4]),
            list![7, 0, 8]
        );
        assert_eq!(
            Solution::add_two_numbers(list![9, 9, 9, 9], list![9, 9, 9, 9, 9, 9]),
            list![8, 9, 9, 9, 0, 0, 1]
        );
        assert_eq!(Solution::add_two_numbers(list![0], list![0]), list![0]);
    }
}
