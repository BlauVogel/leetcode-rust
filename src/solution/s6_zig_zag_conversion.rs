//! 0 ms 2 MB：https://leetcode.com/submissions/detail/394257247/
//! 另外一种使用迭代器的解法：https://leetcode.com/submissions/detail/394268476/

/*
 * @lc app=leetcode id=6 lang=rust
 *
 * [6] ZigZag Conversion
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let len = s.len();
        if num_rows == 0 || len == 0 {
            return s;
        }
        let num_rows = num_rows as usize;
        let base = if num_rows == 1 { 1 } else { 2 * num_rows - 2 }; // 每一组的个数
        let times = len / base;
        let mut result = String::with_capacity(len);
        let chars: Vec<char> = s.chars().collect();

        // 处理每一行
        for row in 0..num_rows {
            // 如果是第一行或者最后一行
            if row == 0 || row == num_rows - 1 {
                // 处理该行有规律的部分
                for i in 0..times {
                    result.push(chars[i * base + row]);
                }
                // 处理该行的剩余部分
                if times * base + row < len {
                    result.push(chars[times * base + row]);
                }
            } else {
                // 处理该行有规律的部分
                for i in 0..times {
                    result.push(chars[i * base + row]);
                    result.push(chars[(i + 1) * base - row]);
                }
                // 处理该行的剩余部分
                if times * base + row < len {
                    result.push(chars[times * base + row]);
                }
                if (times + 1) * base - row < len {
                    result.push(chars[(times + 1) * base - row]);
                }
            }
        }

        result
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 3),
            String::from("PAHNAPLSIIGYIR")
        );
        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 4),
            String::from("PINALSIGYAHRPI")
        );
        assert_eq!(Solution::convert(String::from(""), 1), String::from(""));
        assert_eq!(Solution::convert(String::from("AB"), 1), String::from("AB"));
        assert_eq!(
            Solution::convert(String::from("ABCD"), 2),
            String::from("ACBD")
        );
    }
}
