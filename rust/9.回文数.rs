/*
 * @lc app=leetcode.cn id=9 lang=rust
 *
 * [9] 回文数
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 {
            return false
        } else if x < 10 {
            return true
        }
        if x % 10 == 0 {
            return false
        }

        let mut y = 0;
        while x > y {
            y = y * 10 + x % 10;
            x /= 10;
        }

        return x == y || y / 10 == x
    }
}
// @lc code=end

