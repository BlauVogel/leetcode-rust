// 优美的 Rust 代码！
// 第一次做的垃圾代码：https://leetcode.com/submissions/detail/430224420/

/*
 * @lc app=leetcode id=8 lang=rust
 *
 * [8] String to Integer (atoi)
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        use std::i32;

        let mut res: i32 = 0;
        let mut started = false;
        let mut negated = false;

        for c in s.chars() {
            match c {
                ' ' if !started => {}
                '+' if !started => started = true,
                '-' if !started => {
                    started = true;
                    negated = true;
                }
                '0'..='9' => {
                    started = true;

                    res = match res
                        .checked_mul(10)
                        .and_then(|res| res.checked_add(c as i32 - '0' as i32))
                    {
                        Some(v) => v,
                        None => return if negated { i32::MIN } else { i32::MAX },
                    }
                }
                _ => break,
            }
        }

        if negated {
            -res
        } else {
            res
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::my_atoi(" -42".to_string()), -42);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
        assert_eq!(Solution::my_atoi("3.14159".to_string()), 3);
        assert_eq!(Solution::my_atoi("".to_string()), 0);
        assert_eq!(Solution::my_atoi("+".to_string()), 0);
        assert_eq!(Solution::my_atoi("21474836460".to_string()), 2147483647);
        assert_eq!(Solution::my_atoi(" ".to_string()), 0);
        assert_eq!(Solution::my_atoi("10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000522545459".to_string()), 2147483647);
        assert_eq!(
            Solution::my_atoi("0000000000012345678".to_string()),
            12345678
        );
    }
}
