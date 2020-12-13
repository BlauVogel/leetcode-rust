/*
 * @lc app=leetcode id=10 lang=rust
 *
 * [10] Regular Expression Matching
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        dp[0][0] = true;
        for i in 0..=s.len() {
            for j in 1..=p.len() {
                dp[i][j] = match p[j - 1] {
                    b'*' => {
                        dp[i][j - 2]
                            || (i > 0 && dp[i - 1][j] && (s[i - 1] == p[j - 2] || p[j - 2] == b'.'))
                    }
                    _ => i > 0 && dp[i - 1][j - 1] && (s[i - 1] == p[j - 1] || p[j - 1] == b'.'),
                }
            }
        }

        dp[s.len()][p.len()]
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            false,
            Solution::is_match(String::from("aa"), String::from("a"))
        );
        assert_eq!(
            true,
            Solution::is_match(String::from("aa"), String::from("a*"))
        );
        assert_eq!(
            false,
            Solution::is_match(String::from("aaaab"), String::from("a*c"))
        );
        assert_eq!(
            false,
            Solution::is_match(String::from("aaaaa"), String::from("a*c"))
        );
        assert_eq!(
            true,
            Solution::is_match(String::from("ab"), String::from(".*"))
        );
        assert_eq!(
            true,
            Solution::is_match(String::from("aab"), String::from("c*a*b"))
        );
        assert_eq!(
            false,
            Solution::is_match(String::from("mississippi"), String::from("mis*is*p*."))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            true,
            Solution::is_match(String::from("aaa"), String::from("a*a"))
        );
        assert_eq!(
            true,
            Solution::is_match(String::from("aaaaaaaa"), String::from("aa*aa"))
        );
        assert_eq!(
            true,
            Solution::is_match(String::from(""), String::from(".*"))
        );
    }
}
