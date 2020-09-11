/*
 * @lc app=leetcode id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        if len == 0 {
            return String::from("");
        }
        let seq: Vec<u8> = s.bytes().collect();
        let (mut l, mut r);
        let (mut max, mut start, mut end) = (0, 0, 0);
        for i in 1..len {
            l = i;
            r = i;
            // 先处理相同的
            let ch = seq[i];
            while l > 0 && seq[l - 1] == ch {
                l -= 1;
            }
            while r < len - 1 && seq[r + 1] == ch {
                r += 1;
            }
            // 再处理不同的
            while l >= 1 && r <= len - 2 {
                l -= 1;
                r += 1;
                if seq[l] != seq[r] {
                    l += 1;
                    r -= 1;
                    break;
                }
            }
            if (r - l) > max {
                max = r - l;
                start = l;
                end = r;
            }
        }

        s[start..=end].to_string()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::longest_palindrome(String::from("babad")),
            String::from("bab")
        );
        assert_eq!(
            Solution::longest_palindrome(String::from("cbbd")),
            String::from("bb")
        );
        assert_eq!(
            Solution::longest_palindrome(String::from("")),
            String::from("")
        );
        assert_eq!(
            Solution::longest_palindrome(String::from("bb")),
            String::from("bb")
        );
    }
}
