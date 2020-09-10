//! 首先要抓住这个问题的本质：每个值都必须两两配对一次
//!
//! 此题解中，HashMap 的 get 方法达到了配对的目的（对某一个值，在容器中找到符合要求的那一个）。
//! HashMap 的 insert 和 get 方法都是 O(1)，最终复杂度就是 O(n)。
//! 如果每次暴力遍历所有的元素，是 O(N)，最终复杂度就是 O(n^2)。

/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

pub struct Solution;

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (index, &num) in nums.iter().enumerate() {
            match map.get(&(target - num)) {
                None => map.insert(num, index),
                Some(&i) => return vec![i as i32, index as i32],
            };
        }
        vec![]
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    fn sort_output(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut v = Solution::two_sum(nums, target);
        v.sort();
        v
    }

    #[test]
    fn test() {
        assert_eq!(sort_output(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(sort_output(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(sort_output(vec![3, 3], 6), vec![0, 1]);
    }
}
