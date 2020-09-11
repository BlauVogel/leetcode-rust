/*
 * @lc app=leetcode id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (len1, len2) = (nums1.len(), nums2.len());
        let len = len1 + len2;
        let mut nums = Vec::<i32>::with_capacity(len);
        let (mut i1, mut i2) = (0, 0);
        while i1 < len1 && i2 < len2 {
            if nums1[i1] < nums2[i2] {
                nums.push(nums1[i1]);
                i1 += 1;
            } else {
                nums.push(nums2[i2]);
                i2 += 1;
            }
        }
        while i1 < len1 {
            nums.push(nums1[i1]);
            i1 += 1;
        }
        while i2 < len2 {
            nums.push(nums2[i2]);
            i2 += 1;
        }

        if len % 2 == 0 {
            (nums[len / 2] + nums[len / 2 - 1]) as f64 / 2.0
        } else {
            nums[len / 2] as f64
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![0, 0], vec![0, 0]),
            0.0
        );
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![2], vec![]), 2.0);
    }
}
