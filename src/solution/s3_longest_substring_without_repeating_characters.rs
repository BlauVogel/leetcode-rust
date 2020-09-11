//! - [暴力法](https://leetcode.com/submissions/detail/393975858/) 0 ms 2.2 MB
//! - [HashMap](https://leetcode.com/submissions/detail/393979108/) 4 ms 2.1 MB

/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

pub struct Solution;

// TODO 使用 byte 而非 char 也许能够减少内存
// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let seq: Vec<char> = s.chars().collect();
        let len = seq.len();
        let (mut start, mut end, mut max) = (0, 0, 0);

        while end < len {
            for idx in start..end {
                if seq[end] == seq[idx] {
                    start = idx + 1;
                    break;
                }
            }
            let curr = end - start + 1;
            if curr > max {
                max = curr;
            }
            end += 1;
        }
        max as i32
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbbb")),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        );
        assert_eq!(Solution::length_of_longest_substring(String::from("")), 0);
        assert_eq!(Solution::length_of_longest_substring(String::from(" ")), 1);
        assert_eq!(
            Solution::length_of_longest_substring(String::from("dvdf")),
            3
        );
    }
}
