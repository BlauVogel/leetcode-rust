/*
 * @lc app=leetcode id=7 lang=rust
 *
 * [7] Reverse Integer
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut i = 1;
        if x < 0 {
            i = -1;
            x = -x;
        }
        let s = x.to_string();
        let num: Vec<u8> = s.into_bytes().iter().rev().map(|&x| x).collect();
        let s = unsafe { String::from_utf8_unchecked(num) };

        i * s.parse().unwrap_or(0)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(1000000009), 0);
    }
}
